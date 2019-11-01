use std::io::Error as IoError;
use std::sync::Arc;
use std::time::Duration;

use futures_util::SinkExt;
use futures_util::stream::select;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use tokio::future::ready;
use tokio::io::ErrorKind;
use tokio::prelude::Sink;
use tokio::stream::StreamExt as _;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::com::Connection;
use crate::packet::Packet;
use crate::requests::{RequestError, Requests};
use crate::state::{Event, State, UpdateError};

pub struct Connector {
    sender: Sender<Command>,
    receiver: Receiver<Arc<Packet>>,
    state: State,
}

impl Connector {
    pub async fn login(user: &str, password: &str) -> Result<Self, UpdateError> {
        let mut connection = Connection::connect(user, password).await?;
        let mut handle = ConnectionHandle::default();

        while let Some(packet) = connection.receive().await.transpose()? {
            if let Ok(Some(Event::LoginCompleted)) = handle.state.update(&packet) {
                return Ok(Connector {
                    receiver: handle.new_listener(),
                    state: handle.state.clone(),
                    sender: handle.spawn(connection)
                });
            }
        }

        Err(UpdateError::from(IoError::from(ErrorKind::ConnectionAborted)))
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub async fn update_state<'a>(&'a mut self, timeout: Duration) -> Option<Result<Event<'a>, UpdateError>> {
        if let Ok(packet) = (&mut self.receiver).timeout(timeout).next().await? {
            self.state.update(&packet).transpose()
        } else {
            None
        }
    }

    pub async fn send_request(&mut self, packet: Packet) -> oneshot::Receiver<Result<Packet, RequestError>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Command::SendRequest(packet, sender)).await.expect("ConnectionHandle gone");
        receiver
    }

    pub async fn clone(&mut self) -> Result<Self, ()> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Command::Clone(sender)).await.expect("ConnectionHandle gone");
        let (state, receiver) = receiver.await.expect("ConnectionHandle did not respond");
        Ok(Connector {
            sender: self.sender.clone(),
            receiver,
            state
        })
    }
}

type ResultReceiver = oneshot::Sender<(State, Receiver<Arc<Packet>>)>;
type RequestSender = oneshot::Sender<Result<Packet, RequestError>>;
// IN THE FUTURE https://github.com/rust-lang/rust/issues/63063
// type ConSync = impl Sink<Packet, Error = IoError> + Unpin;

enum Command {
    Clone(ResultReceiver),
    Received(Packet),
    SendRequest(Packet, RequestSender)
}

#[derive(Default)]
struct ConnectionHandle {
    listeners: Vec<Sender<Arc<Packet>>>,
    state: State,
    requests: Requests,
}

impl ConnectionHandle {
    fn new_listener(&mut self) -> Receiver<Arc<Packet>> {
        let (sender, receiver) = mpsc::channel(1024);
        self.listeners.push(sender);
        receiver
    }

    fn spawn(self, connection: Connection) -> Sender<Command> {
        let (sender, receiver) = mpsc::channel(1024);
        tokio::spawn(self.execute(receiver, connection));
        sender
    }

    async fn execute(mut self, commands: Receiver<Command>, connection: Connection) {
        let (connection, connection_stream) = connection.split();
        let commands = select(
            commands,
            connection_stream.filter_map(|p| ready(p.map(Command::Received).ok()))
        );

        match self.process_commands(commands, connection).await {
            Err(e) => {
                error!("Aborting ConnectionHandle because of the following error: {:?}", e);
            }
            Ok(()) => info!("ConnectionHandle is shutting down gracefully")
        }
    }

    async fn process_commands(&mut self, mut commands: impl Stream<Item = Command> + Unpin, mut connection: impl Sink<Packet, Error = IoError> + Unpin) -> Result<(), UpdateError> {
        while let Some(command) = commands.next().await {
            match command {
                Command::Clone(receiver) => self.process_clone(receiver).await,
                Command::Received(packet) => self.process_received(packet).await?,
                Command::SendRequest(packet, sender) => self.process_send_request(&mut connection,packet, sender).await?,
            }
        }
        Ok(())
    }

    async fn process_clone(&mut self, result_receiver: ResultReceiver) {
        let (sender, receiver) = mpsc::channel(1024);
        if let Ok(_) = result_receiver.send((self.state.clone(), receiver)) {
            self.listeners.push(sender);
        }
    }

    async fn process_received(&mut self, packet: Packet) -> Result<(), UpdateError> {
        if let Some(packet) = self.requests.maybe_respond(packet) {
            self.state.update(&packet)?;
            self.publish_packet(Arc::new(packet));
        }
        Ok(())
    }

    fn publish_packet(&mut self, packet: Arc<Packet>) {
        // IN THE FUTURE:
        //    self.listeners.drain_filter(|listener| listener.try_send(packet.clone()).is_err());

        // Vec::default does not allocate heap memory until push(..) - so its common usage here is cheap
        let mut to_delete = Vec::default();

        for (index, listener) in self.listeners.iter_mut().enumerate() {
            if let Err(_) = listener.try_send(packet.clone()) {
                to_delete.push(index);
            }
        }

        for index in to_delete {
            self.listeners.remove(index);
        }
    }

    async fn process_send_request(&mut self, connection: &mut (impl Sink<Packet, Error = IoError> + Unpin), mut packet: Packet, sender: RequestSender) -> Result<(), UpdateError> {
        if self.requests.enqueue_with(&mut packet, sender).is_some() {
            connection.send(packet).await?;
            connection.send(Packet::new_oob()).await?;
            connection.flush().await?;
        }
        Ok(())
    }
}