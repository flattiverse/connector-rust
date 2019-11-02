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
use crate::entity::Universe;
use std::future::Future;

pub struct Connector {
    sender: Sender<Command>,
    receiver: ListenerReceiver,
    state: State,
}

impl Connector {
    pub async fn login(user: &str, password: &str) -> Result<Self, UpdateError> {
        let mut connection = Connection::connect(user, password).await?;
        let mut handle = ConnectionHandle::default();
        let mut state = State::default();

        while let Some(packet) = connection.receive().await.transpose()? {
            if let Ok(Some(Event::LoginCompleted)) = state.update(&packet) {
                return Ok(Connector {
                    receiver: handle.new_listener(),
                    state,
                    sender: handle.spawn(connection)
                });
            }
        }

        Err(UpdateError::from(IoError::from(ErrorKind::ConnectionAborted)))
    }

    pub fn universes(&self) -> impl Iterator<Item = &Universe> {
        self.state.universes()
    }

    pub async fn update_state<'a>(&'a mut self, timeout: Duration) -> Option<Result<Event<'a>, UpdateError>> {
        if let Ok(response) = (&mut self.receiver).timeout(timeout).next().await? {
            match response {
                Response::Packet(packet) => self.state.update(&packet).transpose(),
                Response::Clone(cloner, receiver) => {
                    if let Err(_) = cloner.send(Connector {
                        sender: self.sender.clone(),
                        receiver,
                        state: self.state.clone(),
                    }) {
                        error!("Failed to respond to connector clone response");
                    }
                    None
                }
            }
        } else {
            None
        }
    }

    pub async fn send_request(&mut self, packet: Packet) -> oneshot::Receiver<Result<Packet, RequestError>> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Command::SendRequest(packet, sender)).await.expect("ConnectionHandle gone");
        receiver
    }


    pub fn clone(&mut self) -> impl Future<Output = Self> {
        let (sender, receiver) = oneshot::channel();
        if let Err(_) = self.sender.try_send(Command::Clone(sender)) {
            panic!("ConnectionHandle gone");
        }
        async {
            receiver.await.expect("ConnectionHandle gone")
        }
    }
}

type CloneSender = oneshot::Sender<Connector>;
type RequestResponder = oneshot::Sender<Result<Packet, RequestError>>;
type ListenerSender = Sender<Response>;
type ListenerReceiver = Receiver<Response>;

enum Command {
    Clone(CloneSender),
    Received(Packet),
    SendRequest(Packet, RequestResponder)
}

enum Response {
    Packet(Arc<Packet>),
    Clone(CloneSender, ListenerReceiver),
}

#[derive(Default)]
struct ConnectionHandle {
    listeners: Vec<ListenerSender>,
    requests: Requests,
}

impl ConnectionHandle {
    fn new_listener(&mut self) -> ListenerReceiver {
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
            Ok(()) => debug!("ConnectionHandle is shutting down gracefully")
        }
    }

    async fn process_commands(&mut self, mut commands: impl Stream<Item = Command> + Unpin, mut connection: impl Sink<Packet, Error = IoError> + Unpin) -> Result<(), UpdateError> {
        while let Some(command) = commands.next().await {
            match command {
                Command::Clone(sender) => self.process_clone(sender).await,
                Command::Received(packet) => self.process_received(packet).await?,
                Command::SendRequest(packet, sender) => self.process_send_request(&mut connection,packet, sender).await?,
            }
        }
        Ok(())
    }

    async fn process_clone(&mut self, clone_sender: CloneSender) {
        let (sender, receiver) = mpsc::channel(1024);
        if let Err(_) = self.listeners[0].send(Response::Clone(clone_sender, receiver)).await {
            error!("Failed to respond to clone request");
        } else {
            self.listeners.push(sender);
        }
    }

    async fn process_received(&mut self, packet: Packet) -> Result<(), UpdateError> {
        debug!("ConnectionHandle received {:?}", packet);
        if let Some(packet) = self.requests.maybe_respond(packet) {
            self.publish_packet(Arc::new(packet));
            debug!("Packet has been published to all listeners");
        }
        Ok(())
    }

    fn publish_packet(&mut self, packet: Arc<Packet>) {
        // IN THE FUTURE:
        //    self.listeners.drain_filter(|listener| listener.try_send(packet.clone()).is_err());

        // Vec::default does not allocate heap memory until push(..) - so most of the usage here is cheap
        let mut to_delete = Vec::default();

        for (index, listener) in self.listeners.iter_mut().enumerate() {
            if let Err(_) = listener.try_send(Response::Packet(packet.clone())) {
                warn!("Notifying listener at index {} failed", index);
                to_delete.push(index);
            } else {
                debug!("Notifying listener at index {} succeeded", index);
            }
        }

        for index in to_delete {
            self.listeners.remove(index);
        }
    }

    async fn process_send_request(&mut self, connection: &mut (impl Sink<Packet, Error = IoError> + Unpin), mut packet: Packet, sender: RequestResponder) -> Result<(), UpdateError> {
        if self.requests.enqueue_with(&mut packet, sender).is_some() {
            connection.send(packet).await?;
            connection.send(Packet::new_oob()).await?;
            connection.flush().await?;
        }
        Ok(())
    }
}