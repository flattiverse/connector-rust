use std::io::Error;
use std::thread::sleep;
use std::time::Duration;

use block_modes::BlockMode;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use tokio::codec::Framed;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::codec::Flattiverse;
use crate::crypt::{Aes128Cbc, to_blocks};
use crate::packet::Packet;

pub struct Connection {
    sink: SplitSink<Framed<TcpStream, Flattiverse>, Packet>,
    stream: SplitStream<Framed<TcpStream, Flattiverse>>,
}

impl Connection {
    pub async fn connect(user: &str, password: &str) -> Result<Connection, Error> {
        let iv = Self::random_init_vector();
        let mut packet_data = [0u8; 64];

        let user_hash = crate::crypt::sha256(&user.to_lowercase());
        println!("{} {:x?}", user_hash.len(), user_hash);

        (&mut packet_data[0..16]).copy_from_slice(&iv);
        for i in 0..32 {
            packet_data[i + 16] = user_hash[i] ^ iv[i % 16];
        }

        let connect = TcpStream::connect("galaxy.flattiverse.com:80");
        let password_hash = crate::crypt::hash_password(user, password);
        println!("pass: {:x?}", password_hash);


        let mut stream = connect.await?;
        stream.set_nodelay(true)?;
        stream.write_all(&packet_data[..48]).await?;
        stream.read_exact(&mut packet_data[..48]).await?;


        let (server_iv, data) = (&packet_data[..48]).split_at(16);
        println!("{} {:x?}", server_iv.len(), &server_iv[..]);
        println!("{} {:x?}", data.len(), &data[..]);

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

        let version = u16::from(packet_data[14]) + u16::from(packet_data[15]) * 256;
        println!("Flattiverse version: {}", version);

        if version != 1 {
            panic!("Invalid protocol version");
        }

        let protocol = Flattiverse::new(send, recv);
        let mut framed = Framed::new(stream, protocol);


        for _ in 0..100 {
            sleep(Duration::from_millis(100));
            framed.send(Packet::default()).await.unwrap();
            framed.send(Packet::new_oob()).await.unwrap();
            framed.flush().await.unwrap();
            println!("{:?}", framed.next().await);
        }

        let (sink, stream) = framed.split();

        Ok(Self {
            sink,
            stream
        })
    }

    pub async fn send(&mut self, packet: Packet) -> Result<(), Error> {
        self.sink.send(packet).await
    }

    pub async fn flush(&mut self) -> Result<(), Error> {
        self.send(Packet::new_oob()).await
    }

    pub async fn receive(&mut self) -> Option<Result<Packet, Error>> {
        self.stream.next().await
    }

    pub fn split(self) -> (impl Sink<Packet>, impl Stream<Item = Result<Packet, Error>>) {
        (self.sink, self.stream)
    }

    fn random_init_vector() -> [u8; 16] {
        rand::random()
    }
}