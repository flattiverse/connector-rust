use crate::network::connection_handle::ConnectionHandle;
use crate::network::query::{Query, QueryCommand, QueryKeeper, QueryResult};
use crate::network::{ServerEvent, ServerMessage};
use crate::utils::current_time_millis;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct Connection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    queries: QueryKeeper,
}

impl Connection {
    pub const PING_INTERVAL: Duration = Duration::from_secs(1);

    pub async fn connect_to(uri: &str, api_key: &str) -> Result<Self, OpenError> {
        let (mut stream, _response) = connect_async(format!("{uri}?auth={api_key}")).await?;
        Self::try_set_tcp_nodelay(&mut stream);
        Ok(Self {
            stream,
            queries: QueryKeeper::default(),
        })
    }

    fn try_set_tcp_nodelay(stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) {
        match stream.get_mut() {
            MaybeTlsStream::Plain(s) => {
                if let Err(e) = s.set_nodelay(true) {
                    warn!("Failed to set TCP_NODELAY: {e:?}");
                }
            }
            // MaybeTlsStream::NativeTls(s) => s.set_nodelay(true)?,
            MaybeTlsStream::Rustls(s) => {
                if let Err(e) = s.get_mut().0.set_nodelay(true) {
                    warn!("Failed to set TCP_NODELAY: {e:?}");
                }
            }
            s => {
                warn!("Unable to set TCP_NODELAY, unexpected MayeTlsStream-Variant: {s:?}");
            }
        };
    }

    pub fn spawn(self) -> (Arc<ConnectionHandle>, UnboundedReceiver<ConnectionEvent>) {
        let (sink, stream) = self.stream.split();
        let (sender, receiver) = mpsc::unbounded_channel();
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        let queries = Arc::new(Mutex::new(self.queries));

        let sender_handle = tokio::spawn({
            ConnectionSender {
                sink,
                queries: Arc::clone(&queries),
            }
            .run(receiver, Self::PING_INTERVAL)
        });

        (
            Arc::new(ConnectionHandle {
                sender: sender.clone(),
                handle: tokio::spawn(async move {
                    let receiver = ConnectionReceiver { stream, queries }.run(sender, event_sender);
                    tokio::select! {
                        r = sender_handle => {
                            if let Err(e) = r {
                                error!("ConnectionSender failed: {e:?}");
                            }
                        },
                        r = receiver => {
                            if let Err(e) = r {
                                error!("ConnectionReceiver failed: {e:?}")
                            }
                        }
                    }
                }),
            }),
            event_receiver,
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OpenError {
    #[error("Underlying connection error")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}

struct ConnectionSender {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    queries: Arc<Mutex<QueryKeeper>>,
}

impl ConnectionSender {
    async fn run(
        mut self,
        mut receiver: UnboundedReceiver<SenderData>,
        ping_interval: Duration,
    ) -> Result<(), SenderError> {
        let mut ping_interval = interval(ping_interval);
        loop {
            tokio::select! {
                _ = ping_interval.tick() => self.send_ping().await?,
                cmd = receiver.recv() => {
                    match cmd {
                        Some(SenderData::Query(command, target)) => {
                            let id = self.queries.lock().await.register_new_for(target);
                            self.send(Message::Text(serde_json::to_string(&Query {
                                id,
                                command,
                            })?)).await?
                        },
                        Some(SenderData::Raw(message)) => {
                            self.send(message).await?;
                        }
                        None => break,
                    }
                }
            }
        }
        Ok(())
    }

    #[inline]
    async fn send_ping(&mut self) -> Result<(), SenderError> {
        self.send(Message::Ping(current_time_millis().to_be_bytes().to_vec()))
            .await
    }

    #[inline]
    async fn send(&mut self, msg: Message) -> Result<(), SenderError> {
        debug!("SENDING: {msg:?}");
        Ok(self.sink.send(msg).await?)
    }
}

pub enum SenderData {
    Raw(Message),
    Query(QueryCommand, oneshot::Sender<QueryResult>),
}

#[derive(thiserror::Error, Debug)]
pub enum SenderError {
    #[error("Failed to transmit request: {0}")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Failed to encode the JSON request: {0}")]
    EncodeError(#[from] serde_json::Error),
}

struct ConnectionReceiver {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    queries: Arc<Mutex<QueryKeeper>>,
}

impl ConnectionReceiver {
    async fn run(
        mut self,
        sender: mpsc::UnboundedSender<SenderData>,
        event_sender: mpsc::UnboundedSender<ConnectionEvent>,
    ) -> Result<(), ReceiveError> {
        while let Some(message) = self.stream.next().await.transpose()? {
            match message {
                Message::Text(text) => match serde_json::from_str(&text)? {
                    ServerMessage::Success { id, result } => {
                        self.queries.lock().await.answer(&id, Ok(result));
                    }
                    ServerMessage::Failure { id, code } => {
                        self.queries.lock().await.answer(&id, Err(code.into()));
                    }
                    ServerMessage::Events { events } => {
                        for event in events {
                            if event_sender
                                .send(ConnectionEvent::ServerEvent(event))
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                },
                b @ (Message::Frame(_) | Message::Binary(_)) => {
                    return Err(ReceiveError::UnexpectedData(format!("{b:?}")));
                }
                Message::Ping(data) => {
                    if sender.send(SenderData::Raw(Message::Pong(data))).is_err() {
                        break;
                    }
                }
                Message::Pong(data) => {
                    let mut millis = 0_u64.to_be_bytes();
                    let millis_len = millis.len();
                    if millis_len <= data.len() {
                        millis.copy_from_slice(&data[..millis_len]);
                        if event_sender
                            .send(ConnectionEvent::PingMeasured(Duration::from_millis(
                                current_time_millis().saturating_sub(u64::from_be_bytes(millis)),
                            )))
                            .is_err()
                        {
                            break;
                        }
                    }
                }
                Message::Close(_) => break,
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReceiveError {
    #[error("Connection is closed")]
    ConnectionClosed,
    #[error("Connection has encountered an error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Unexpected data received: {0}")]
    UnexpectedData(String),
    #[error("Failed to decode the JSON response: {0}")]
    DecodeError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub enum ConnectionEvent {
    PingMeasured(Duration),
    ServerEvent(ServerEvent),
}
