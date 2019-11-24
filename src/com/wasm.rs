use std::io::Error as IoError;

use futures::channel::mpsc::Sender;
use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::channel;
use block_modes::BlockMode;
use crate::crypt::{Aes128Cbc, to_blocks, AES128CBC_BLOCK_BYTE_LENGTH};
use crate::packet::Packet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::TcpSocket;
use futures::{Sink, Stream};

const BLOCK_LENGTH: usize = AES128CBC_BLOCK_BYTE_LENGTH;

pub struct Connection {
    version: u16,
}

impl Connection {
    pub async fn connect(user: &str, password: &str) -> Result<Self, IoError> {

        let socket = TcpSocket::new("galaxy.flattiverse.com", 80);


        /*



        let iv = Self::random_init_vector();
        let mut packet_data = [0u8; 64];

        let user_hash = crate::crypt::sha256(&user.to_lowercase());
        debug!("user hash: {} {:x?}", user_hash.len(), user_hash);

        (&mut packet_data[0..16]).copy_from_slice(&iv);
        for i in 0..32 {
            packet_data[i + 16] = user_hash[i] ^ iv[i % 16];
        }

        let password_hash = crate::crypt::hash_password(user, password);
        debug!("pass hash: {:x?}", password_hash);


        let mut stream = connect.await?;
        stream.set_nodelay(true)?;
        stream.write_all(&packet_data[..48]).await?;
        stream.read_exact(&mut packet_data[..48]).await?;


        let (server_iv, data) = (&packet_data[..48]).split_at(16);
        debug!("server iv: {} {:x?}", server_iv.len(), &server_iv[..]);
        debug!(" local iv: {} {:x?}", data.len(), &data[..]);

        let mut send = Aes128Cbc::new_var(&password_hash[..], &iv[..]).unwrap();
        let mut recv = Aes128Cbc::new_var(&password_hash[..], &server_iv[..]).unwrap();

        recv.decrypt_blocks(to_blocks(&mut packet_data[16..16+32]));
        for i in 16..32 {
            packet_data[i] = packet_data[i] ^ packet_data[i + 16];
        }

        //send.encrypt(&mut RefReadBuffer::new(&challenge[..16]), &mut RefWriteBuffer::new(&mut packet_data[..16]), false).unwrap();
        send.encrypt_blocks(to_blocks(&mut packet_data[16..32]));
        stream.write_all(&packet_data[16..32]).await?;
        stream.read_exact(&mut packet_data[..16]).await.expect("Wrong password");
        debug!("Connected to flattiverse server");

        let version = u16::from(packet_data[14]) + u16::from(packet_data[15]) * 256;

        if version != 1 {
            panic!("Invalid protocol version: {}", version);
        } else {
            debug!("Using protocol version {}", version);
        }

        let protocol = Flattiverse::new(send, recv);
        let framed = Framed::new(stream, protocol);
        let (sink, stream) = framed.split();
        */

        let version = 1;

        Ok(Self {
            version,
        })
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub async fn send(&mut self, packet: Packet) -> Result<(), IoError> {
        unimplemented!()
    }

    pub async fn flush(&mut self) -> Result<(), IoError> {
        unimplemented!()
    }

    pub async fn receive(&mut self) -> Option<Result<Packet, IoError>> {
        unimplemented!()
    }

    pub fn split(self) -> (impl Sink<Packet, Error = IoError>, impl Stream<Item = Result<Packet, IoError>>) {
        unimplemented!()
    }

    fn random_init_vector() -> [u8; 16] {
        unimplemented!()
    }
}