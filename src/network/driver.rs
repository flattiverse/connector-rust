use crate::galaxy_hierarchy::Galaxy;
use crate::network::connection_handle::ConnectionHandle;
use crate::network::packet::MultiPacketBuffer;
use crate::network::{ConnectError, Connection, SenderData};
use crate::FlattiverseEvent;
use bytes::BytesMut;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub const PING_INTERVAL: Duration = Duration::from_secs(1);
pub const DEFAULT_PORT_WEB: u16 = 443;
pub const DEFAULT_PORT_PROXY: u16 = 80;
pub const ENV_PROXY: &str = "http_proxy";

pub async fn connect(
    url: &str,
    f: impl FnOnce(ConnectionHandle, async_channel::Receiver<FlattiverseEvent>) -> Arc<Galaxy>,
) -> Result<Arc<Galaxy>, ConnectError> {
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

    let (sink, stream) = stream.split();
    let (data_sender, data_receiver) = tokio::sync::mpsc::channel(1024);
    let (event_sender, event_receiver) = async_channel::unbounded();

    let handle = ConnectionHandle::from(data_sender.clone());
    let galaxy = f(handle.clone(), event_receiver);
    let connection = Connection {
        handle,
        galaxy: Arc::downgrade(&galaxy),
        sender: event_sender,
    };

    let mut sender_handle =
        tokio::spawn(ConnectionSender { sink }.run(data_receiver, PING_INTERVAL));
    let receiver_handle = ConnectionReceiver { stream, connection }.run(data_sender);

    tokio::spawn({
        async move {
            tokio::select! {
                r = &mut sender_handle => {
                    if let Err(e) = r {
                        eprintln!("ConnectionSender failed: {e:?}");
                    }
                },
                r = receiver_handle => {
                    sender_handle.abort();
                    if let Err(e) = r {
                        eprintln!("ConnectionReceiver failed: {e:?}")
                    }
                }
            }
        }
    });

    Ok(galaxy)
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

struct ConnectionSender {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl ConnectionSender {
    async fn run(
        mut self,
        mut receiver: Receiver<SenderData>,
        ping_interval: Duration,
    ) -> Result<(), SenderError> {
        let mut ping_interval = interval(ping_interval);
        loop {
            tokio::select! {
                _ = ping_interval.tick() => self.send_ping().await?,
                cmd = receiver.recv() => {
                    match cmd {
                        Some(SenderData::Close) => {
                            debug!("ConnectionSender received close request");
                            return Ok(())
                        }
                        Some(SenderData::Raw(message)) => {
                            self.send(message).await?;
                        }
                        Some(SenderData::Packet(packet)) => {
                            self.send(Message::Binary(packet.into_buf().to_vec())).await?;
                        }
                        None => return Ok(())
                    }
                }
            }
        }
    }

    #[inline]
    async fn send_ping(&mut self) -> Result<(), SenderError> {
        self.send(Message::Ping(current_time_micros().to_le_bytes().to_vec()))
            .await
    }

    #[inline]
    #[instrument(level = "trace", skip(self, msg))]
    async fn send(&mut self, msg: Message) -> Result<(), SenderError> {
        self.sink.send(msg).await?;
        self.sink.flush().await?;
        Ok(())
    }
}

fn current_time_micros() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_micros() as _
}

#[derive(thiserror::Error, Debug)]
pub enum SenderError {
    #[error("Failed to transmit request: {0}")]
    IoError(#[from] tokio_tungstenite::tungstenite::Error),
}

struct ConnectionReceiver {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    connection: Connection,
}

impl ConnectionReceiver {
    async fn run(mut self, sender: Sender<SenderData>) -> Result<(), ReceiveError> {
        while let Some(message) = self.stream.next().await.transpose()? {
            match message {
                b @ (Message::Frame(_) | Message::Text(_)) => {
                    return Err(ReceiveError::UnexpectedData(format!("{b:?}")));
                }
                Message::Binary(bin) => {
                    // TODo sad copy
                    let mut packet = MultiPacketBuffer::from(BytesMut::from(&bin[..]));
                    while let Some(packet) = packet.next_packet() {
                        if let Err(e) = self.connection.handle(packet) {
                            error!("Failed to handle Packet: {e:?}");
                            return Err(ReceiveError::GalaxyGone);
                        }
                    }
                }
                Message::Ping(data) => {
                    if sender
                        .try_send(SenderData::Raw(Message::Pong(data)))
                        .is_err()
                    {
                        return Err(ReceiveError::SenderGone);
                    }
                }
                Message::Pong(data) => {
                    let mut micros = 0_u64.to_le_bytes();
                    let micros_len = micros.len();
                    if micros_len <= data.len() {
                        micros.copy_from_slice(&data[..micros_len]);
                        let duration = Duration::from_micros(
                            current_time_micros().saturating_sub(u64::from_le_bytes(micros)),
                        );
                        if let Err(e) = self.connection.on_ping_measured(duration) {
                            error!("Failed to handle ping={duration:?} measurement: {e:?}");
                            return Err(ReceiveError::GalaxyGone);
                        }
                    }
                }
                Message::Close(msg) => {
                    info!(
                        "Connection closed by the server: code={:?}, reason={:?}",
                        msg.as_ref().map(|m| m.code),
                        msg.as_ref().map(|m| m.reason.as_ref()).unwrap_or_default()
                    );
                    let _ = sender.try_send(SenderData::Close);
                    self.connection.on_close();
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReceiveError {
    #[error("Sender channel gone")]
    SenderGone,
    #[error("Connection has encountered an error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("The Galaxy is no longer reachable")]
    GalaxyGone,
    #[error("Unexpected data received: {0}")]
    UnexpectedData(String),
}
