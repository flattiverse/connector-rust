use crate::network::connection_handle::ConnectionHandle;
use crate::network::{ConnectError, Connection, ConnectionEvent, Packet, SenderData};
use crate::utils::current_time_millis;
use async_channel::{Receiver, Sender};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub const PING_INTERVAL: Duration = Duration::from_secs(1);
pub const DEFAULT_PORT_WEB: u16 = 443;
pub const DEFAULT_PORT_PROXY: u16 = 80;
pub const ENV_PROXY: &'static str = "http_proxy";

pub async fn connect(url: &str) -> Result<Connection, ConnectError> {
    let url = Url::from_str(url).map_err(ConnectError::MalformedHostUrl)?;
    let (mut stream, _response) = match std::env::var(ENV_PROXY).ok() {
        Some(proxy) => {
            if cfg!(feature = "debug-proxy") {
                eprintln!("detected proxy environment variable {}={proxy}", ENV_PROXY);
            }
            let proxy = Url::from_str(&proxy).map_err(ConnectError::MalformedProxyUrl)?;
            let proxy = format!(
                "{}:{}",
                proxy.host_str().unwrap_or_default(),
                proxy.port_or_known_default().unwrap_or(DEFAULT_PORT_PROXY)
            );

            if cfg!(feature = "debug-proxy") {
                eprintln!("establishing connection via proxy through {proxy}");
            }
            let mut stream = TcpStream::connect(proxy)
                .await
                .map_err(ConnectError::ProxyConnectionError)?;

            async_http_proxy::http_connect_tokio(
                &mut stream,
                url.host_str().unwrap_or_default(),
                url.port_or_known_default().unwrap_or(DEFAULT_PORT_WEB),
            )
            .await?;

            tokio_tungstenite::client_async_tls_with_config(url, stream, None, None).await?
        }
        None => connect_async(url).await?,
    };

    try_set_tcp_nodelay(&mut stream);

    let (sender, receiver) = {
        let (sink, stream) = stream.split();
        let (sender, receiver) = async_channel::unbounded();
        let (event_sender, event_receiver) = async_channel::unbounded();
        let counter = Arc::new(AtomicUsize::new(0));

        let mut sender_handle =
            tokio::spawn(ConnectionSender { sink }.run(receiver, PING_INTERVAL));

        tokio::spawn({
            let sender = sender.clone();
            let counter = Arc::clone(&counter);
            async move {
                let receiver = ConnectionReceiver { stream }.run(sender, event_sender, counter);
                tokio::select! {
                    r = &mut sender_handle => {
                        if let Err(e) = r {
                            eprintln!("ConnectionSender failed: {e:?}");
                        }
                    },
                    r = receiver => {
                        sender_handle.abort();
                        if let Err(e) = r {
                            eprintln!("ConnectionReceiver failed: {e:?}")
                        }
                    }
                }
            }
        });

        (sender, event_receiver)
    };

    Ok(Connection::from_existing(
        ConnectionHandle { sender },
        receiver,
    ))
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

#[derive(thiserror::Error, Debug)]
pub enum OpenError {
    #[error("Underlying connection error")]
    IoError(tokio_tungstenite::tungstenite::Error),
    #[error("The provided url is malformed: {0}")]
    MalformedHostUrl(url::ParseError),
    #[error("The url to the proxy server is malformed: {0}")]
    MalformedProxyUrl(url::ParseError),
    #[error("Failed to connect to the proxy server: {0}")]
    ProxyConnectionError(std::io::Error),
    #[error("The proxy server sent and unexpected response: {0}")]
    ProxyResponseError(#[from] async_http_proxy::HttpError),
    // --- parsed from status code
    #[error("No auth parameter was given, or a malformed or non-existing auth key was given. A proper auth parameter consists of string of 64 characters representing hex values. A connection as a spectator was attempted, but the UniverseGroup does not allow spectators")]
    MissingAuthOr(Option<String>),
    #[error("A connection with a wrong connector version was attempted.")]
    WrongConnectorVersion(Option<String>),
    #[error("A connection as a player or admin was attempted, but the associated account is still online with another connection. As disconnecting players will linger for a while, a connection may not be possible for a short time even if a previous connection has been closed or severed")]
    StillOnline(Option<String>),
    #[error("A connection with a wrong team was attempted")]
    WrongTeam(Option<String>),
    #[error("The UniverseGroup is currently at capacity and no further connections are possible.")]
    UniverseFull(Option<String>),
    #[error("The UniverseGroup is currently offline.")]
    UniverseOffline(Option<String>),
}

impl From<tokio_tungstenite::tungstenite::Error> for OpenError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        if let tokio_tungstenite::tungstenite::Error::Http(response) = value {
            fn into_msg(
                response: tokio_tungstenite::tungstenite::http::Response<Option<Vec<u8>>>,
            ) -> Option<String> {
                response.into_body().and_then(|b| String::from_utf8(b).ok())
            }

            match response.status().as_u16() {
                401 => OpenError::MissingAuthOr(into_msg(response)),
                409 => OpenError::WrongConnectorVersion(into_msg(response)),
                412 => OpenError::StillOnline(into_msg(response)),
                415 => OpenError::WrongTeam(into_msg(response)),
                417 => OpenError::UniverseFull(into_msg(response)),
                502 => OpenError::UniverseOffline(into_msg(response)),
                _ => OpenError::IoError(tokio_tungstenite::tungstenite::Error::Http(response)),
            }
        } else {
            OpenError::IoError(value)
        }
    }
}

struct ConnectionSender {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl ConnectionSender {
    async fn run(
        mut self,
        receiver: Receiver<SenderData>,
        ping_interval: Duration,
    ) -> Result<(), SenderError> {
        let mut ping_interval = interval(ping_interval);
        loop {
            tokio::select! {
                _ = ping_interval.tick() => self.send_ping().await?,
                cmd = receiver.recv() => {
                    match cmd {
                        Ok(SenderData::Raw(message)) => {
                            self.send(message).await?;
                        }
                        Err(_) => return Ok(()),
                    }
                }
            }
        }
    }

    #[inline]
    async fn send_ping(&mut self) -> Result<(), SenderError> {
        self.send(Message::Ping(current_time_millis().to_be_bytes().to_vec()))
            .await
    }

    #[inline]
    async fn send(&mut self, msg: Message) -> Result<(), SenderError> {
        debug!("SENDING: {msg:?}");
        self.sink.send(msg).await?;
        self.sink.flush().await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SenderError {
    #[error("Failed to transmit request: {0}")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}

struct ConnectionReceiver {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl ConnectionReceiver {
    async fn run(
        mut self,
        sender: Sender<SenderData>,
        event_sender: Sender<ConnectionEvent>,
        counter: Arc<AtomicUsize>,
    ) -> Result<(), ReceiveError> {
        while let Some(message) = self.stream.next().await.transpose()? {
            match message {
                b @ (Message::Frame(_) | Message::Text(_)) => {
                    return Err(ReceiveError::UnexpectedData(format!("{b:?}")));
                }
                Message::Binary(bin) => {
                    let mut packet = Packet::new(bin);
                    while let Some(reader) = packet.next_reader() {
                        match crate::network::ConnectionEvent::try_from(reader) {
                            Err(e) => error!("Failed to decode ConnectionEvent {e:?}"),
                            Ok(event) => {
                                if let Err(e) = event_sender.send(event).await {
                                    error!("Failed to send ConnectionEvent {e:?}");
                                    return Err(ReceiveError::ConnectionHandleGone);
                                }
                            }
                        }
                    }
                }
                Message::Ping(data) => {
                    if sender
                        .try_send(SenderData::Raw(Message::Pong(data)))
                        .is_err()
                    {
                        break;
                    }
                }
                Message::Pong(data) => {
                    let mut millis = 0_u64.to_be_bytes();
                    let millis_len = millis.len();
                    if millis_len <= data.len() {
                        millis.copy_from_slice(&data[..millis_len]);
                        if event_sender
                            .try_send(ConnectionEvent::PingMeasured(Duration::from_millis(
                                current_time_millis().saturating_sub(u64::from_be_bytes(millis)),
                            )))
                            .is_err()
                        {
                            break;
                        } else {
                            counter.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Message::Close(msg) => {
                    if event_sender
                        .try_send(ConnectionEvent::Closed(
                            msg.map(|msg| format!("{} - {}", msg.code, msg.reason)),
                        ))
                        .is_ok()
                    {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReceiveError {
    #[error("Connection has encountered an error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("The ConnectionHandle is no longer reachable")]
    ConnectionHandleGone,
    #[error("Unexpected data received: {0}")]
    UnexpectedData(String),
}
