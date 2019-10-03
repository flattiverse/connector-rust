use std::io::Error;

use aes::block_cipher_trait::generic_array::ArrayLength;
use aes::Aes128;
use bytes::{BufMut, Bytes, BytesMut};
use tokio::codec::Decoder;
use tokio::codec::Encoder;

use crate::crypt::{to_blocks, Aes128Cbc, AES128CBC_BLOCK_BYTE_LENGTH};

use crate::packet::Packet;
use block_modes::BlockMode;

const BLOCK_LENGTH: usize = AES128CBC_BLOCK_BYTE_LENGTH;

pub struct Flattiverse {
    send: Aes128Cbc,
    recv: Aes128Cbc,
    send_block: BytesMut,
    recv_block: BytesMut,
}

impl Flattiverse {
    pub fn new(send: Aes128Cbc, recv: Aes128Cbc) -> Self {
        Flattiverse {
            send,
            recv,
            send_block: BytesMut::new(),
            recv_block: BytesMut::new(),
        }
    }
}

impl Encoder for Flattiverse {
    type Item = Packet;
    type Error = Error;

    fn encode(&mut self, mut item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item.out_of_band {
            let fill_bytes = BLOCK_LENGTH - self.send_block.len() % BLOCK_LENGTH - 1;
            item.payload = Some(vec![0x55u8; fill_bytes].into());
        }
        item.write(&mut self.send_block);

        let blocks = self.send_block.len() / BLOCK_LENGTH;
        if blocks > 0 {
            let mut buffer = self.send_block.split_to(blocks * BLOCK_LENGTH);
            self.send.encrypt_blocks(to_blocks(&mut buffer[..]));
            dst.put_slice(&buffer[..]);
        }
        Ok(())
    }
}

impl Decoder for Flattiverse {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let blocks = src.len() / BLOCK_LENGTH;
        if blocks > 0 {
            let len = blocks * BLOCK_LENGTH;
            self.recv.decrypt_blocks(to_blocks(&mut src[..len]));
            self.recv_block.reserve(len);
            self.recv_block.put_slice(&src[..len]);
            src.advance(len);
        }
        Ok(Packet::try_parse(&mut self.recv_block).and_then(|p| {
            if p.out_of_band {
                None
            } else {
                Some(p)
            }
        }))
    }
}
