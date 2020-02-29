use crate::connector::Connector;
use crate::packet::Packet;
use crate::requests::RequestError;
use futures::channel::oneshot;
use futures::channel::oneshot::Canceled;
use std::convert::TryFrom;
use std::future::Future;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Request<P: TryFromRef<Packet, Error = IoError>> {
    packet: Packet,
    _p: PhantomData<P>,
}

impl<P: TryFromRef<Packet, Error = IoError>> Request<P> {
    pub fn new(request: Packet) -> Self {
        Self {
            packet: request,
            _p: Default::default(),
        }
    }

    pub async fn send(self, connector: &mut Connector) -> Result<PendingRequest<P>, RequestError> {
        let receiver = connector.send_request(self.packet).await;
        Ok(PendingRequest {
            receiver,
            _p: self._p,
        })
    }
}

impl<P: TryFromRef<Packet, Error = IoError>> From<Packet> for Request<P> {
    fn from(packet: Packet) -> Self {
        Self::new(packet)
    }
}

impl<P: TryFromRef<Packet, Error = IoError>> Into<Packet> for Request<P> {
    fn into(self) -> Packet {
        self.packet
    }
}

impl<P: TryFromRef<Packet, Error = IoError>> Deref for Request<P> {
    type Target = Packet;

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}

pub struct PendingRequest<P: TryFromRef<Packet, Error = IoError>> {
    receiver: oneshot::Receiver<Result<Packet, RequestError>>,
    _p: PhantomData<P>,
}

impl<P: TryFromRef<Packet, Error = IoError>> Future for PendingRequest<P> {
    type Output = Result<P, RequestError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe { self.map_unchecked_mut(|s| &mut s.receiver) }.poll(cx) {
            Poll::Ready(Ok(Ok(packet))) => {
                Poll::Ready({ P::try_from_ref(&packet).map_err(RequestError::from) })
            }
            Poll::Ready(Ok(Err(e))) => Poll::Ready(Err(e)),
            Poll::Ready(Err(Canceled)) => {
                Poll::Ready(Err(IoError::from(IoErrorKind::NotConnected).into()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

pub trait TryFromRef<V>: Sized {
    type Error;

    fn try_from_ref(value: &V) -> Result<Self, Self::Error>;
}

impl<V, T> TryFromRef<V> for T
where
    // I dont understand exactly why this for<'a> thingy solves it
    // https://stackoverflow.com/questions/34630695/how-to-write-a-trait-bound-for-adding-two-references-of-a-generic-type
    // Higher Ranked Trait Bound (HRTB): https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md
    for<'a> T: TryFrom<&'a V, Error = IoError>,
{
    type Error = IoError;

    fn try_from_ref(value: &V) -> Result<Self, Self::Error> {
        T::try_from(value)
    }
}
