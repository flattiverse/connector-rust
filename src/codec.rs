use crate::packet::Packet;
use bytes::BytesMut;
use std::io::Error;
use tokio::codec::Decoder;
use tokio::codec::Encoder;

pub struct Flattiverse;

impl Encoder for Flattiverse {
    type Item = Packet;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write(dst);
        Ok(())
    }
}

impl Decoder for Flattiverse {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Packet::try_parse(src))
    }
}
