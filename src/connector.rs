use crate::com::Connection;
use crate::entity::Universe;
use crate::packet::Packet;
use crate::players::Player;
use crate::requests::{RequestError, Requests};
use crate::state::{Event, State, UpdateError};
use futures::channel::oneshot;
use futures::stream::select;
use futures::Stream;
use futures::{Sink, SinkExt};
use std::future::Future;
use std::io::Error as IoError;
use std::mem;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::ErrorKind;
use tokio::stream::StreamExt as _;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{channel, Receiver};

const LISTENER_CHANNEL_SIZE: usize = 1024;

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
                    sender: handle.spawn(connection),
                });
            }
        }

        Err(UpdateError::from(IoError::from(
            ErrorKind::ConnectionAborted,
        )))
    }

    pub fn universes(&self) -> impl Iterator<Item = &Universe> {
        self.state.universes.iter().filter_map(Option::as_ref)
    }

    pub fn universe(&self, id: usize) -> Option<&Universe> {
        self.state.universes.get(id).and_then(Option::as_ref)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.state.players.iter().filter_map(Option::as_ref)
    }

    pub fn player(&self, id: usize) -> Option<&Player> {
        self.state.players.get(id).and_then(Option::as_ref)
    }

    pub async fn update<'a>(
        &'a mut self,
        timeout: Duration,
    ) -> Option<Result<Event<'a>, UpdateError>> {
        if let Ok(response) =
            tokio::stream::StreamExt::next(&mut (&mut self.receiver).timeout(timeout)).await?
        {
            match response {
                Response::Packet(packet) => self.state.update(&packet).transpose(),
                Response::Clone(cloner, receiver) => {
                    Self::handle_clone_response(&self.sender, &self.state, cloner, receiver);
                    None
                }
            }
        } else {
            None
        }
    }

    fn handle_clone_response(
        sender: &Sender<Command>,
        state: &State,
        cloner: CloneSender,
        receiver: ListenerReceiver,
    ) {
        if let Err(_) = cloner.send(Connector {
            sender: sender.clone(),
            receiver,
            state: state.clone(),
        }) {
            error!("Failed to respond to connector clone response");
        }
    }

    pub async fn send_request(
        &mut self,
        packet: Packet,
    ) -> oneshot::Receiver<Result<Packet, RequestError>> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Command::SendRequest(packet, sender))
            .await
            .map_err(drop)
            .expect("ConnectionHandle gone");
        receiver
    }

    pub fn clone(&mut self) -> impl Future<Output = Self> {
        let (sender, receiver) = oneshot::channel();
        if let Err(_) = self.sender.try_send(Command::Clone(sender)) {
            panic!("ConnectionHandle gone");
        }
        async { receiver.await.expect("ConnectionHandle gone") }
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        self.receiver.close(); // from now on, no more Responses will be received

        // stuff out the Connector which is going to be dropped
        let sender = mem::replace(&mut self.sender, channel(1).0);
        let mut receiver = mem::replace(&mut self.receiver, channel(1).1);
        let state = mem::replace(&mut self.state, State::default());

        tokio::spawn(async move {
            // await all Responses that are enqueued
            while let Some(response) = receiver.recv().await {
                if let Response::Clone(cloner, receiver) = response {
                    Self::handle_clone_response(&sender, &state, cloner, receiver);
                }
            }
        });
    }
}

type CloneSender = oneshot::Sender<Connector>;
type RequestResponder = oneshot::Sender<Result<Packet, RequestError>>;
type ListenerSender = Sender<Response>;
type ListenerReceiver = Receiver<Response>;

enum Command {
    Clone(CloneSender),
    Received(Packet),
    SendRequest(Packet, RequestResponder),
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
        let (sender, receiver) = mpsc::channel(LISTENER_CHANNEL_SIZE);
        self.listeners.push(sender);
        receiver
    }

    fn spawn(self, connection: Connection) -> Sender<Command> {
        let (sender, receiver) = mpsc::channel(LISTENER_CHANNEL_SIZE);
        tokio::spawn(self.execute(receiver, connection));
        sender
    }

    async fn execute(mut self, commands: Receiver<Command>, connection: Connection) {
        let (connection, connection_stream) = connection.split();
        let commands = select(
            commands,
            tokio::stream::StreamExt::filter_map(connection_stream, |p| {
                p.map(Command::Received).ok()
            }),
        );

        match self.process_commands(commands, connection).await {
            Err(e) => {
                error!(
                    "Aborting ConnectionHandle because of the following error: {:?}",
                    e
                );
            }
            Ok(()) => debug!("ConnectionHandle is shutting down gracefully"),
        }
    }

    async fn process_commands(
        &mut self,
        mut commands: impl Stream<Item = Command> + Unpin,
        mut connection: impl Sink<Packet, Error = IoError> + Unpin,
    ) -> Result<(), UpdateError> {
        while let Some(command) = tokio::stream::StreamExt::next(&mut commands).await {
            match command {
                Command::Clone(sender) => self.process_clone(sender).await,
                Command::Received(packet) => self.process_received(packet).await?,
                Command::SendRequest(packet, sender) => {
                    self.process_send_request(&mut connection, packet, sender)
                        .await?
                }
            }
        }
        Ok(())
    }

    async fn process_clone(&mut self, clone_sender: CloneSender) {
        let (sender, receiver) = mpsc::channel(LISTENER_CHANNEL_SIZE);
        let mut response = Response::Clone(clone_sender, receiver);

        for listener in &mut self.listeners {
            match listener.try_send(response) {
                Err(TrySendError::Full(inner)) => response = inner,
                Err(TrySendError::Closed(inner)) => response = inner,
                Ok(_) => {
                    self.listeners.push(sender);
                    return;
                }
            }
        }
        error!("Failed to find suitable listener to process clone request");
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

    async fn process_send_request(
        &mut self,
        connection: &mut (impl Sink<Packet, Error = IoError> + Unpin),
        mut packet: Packet,
        sender: RequestResponder,
    ) -> Result<(), UpdateError> {
        if self.requests.enqueue_with(&mut packet, sender).is_some() {
            connection.send(packet).await?;
            connection.send(Packet::new_oob()).await?;
            connection.flush().await?;
        }
        Ok(())
    }
}
