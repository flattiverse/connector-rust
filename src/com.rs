
use std::io::Error;

use crypto::aes::{cbc_decryptor, cbc_encryptor};
use crypto::aes::KeySize::{KeySize256, KeySize128};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


pub struct Connection {

}

impl Connection {
    pub async fn connect(user: &str, password: &str) -> Result<Connection, Error> {
        let iv = Self::random_init_vector();
        let mut packet_data = [0u8; 64];

        let user_hash = crate::crypt::sha256(&user.to_lowercase());
        println!("{} {:?}", user_hash.len(), user_hash);

        (&mut packet_data[0..16]).copy_from_slice(&iv);
        for i in 0..32 {
            packet_data[i + 16] = user_hash[i] ^ iv[i % 16];
        }

        let connect = TcpStream::connect("galaxy.flattiverse.com:80");
        let password_hash = crate::crypt::hash_password(user, password);


        let mut stream = connect.await?;
        stream.set_nodelay(true)?;
        stream.write_all(&packet_data[..48]).await?;
        stream.read_exact(&mut packet_data[..48]).await?;


        let (server_iv, data) = (&packet_data[..48]).split_at(16);
        println!("{} {:?}", server_iv.len(), &server_iv[..]);
        println!("{} {:?}", data.len(), &data[..]);

        let mut send = cbc_encryptor(KeySize128, &password_hash[..], &iv[..], NoPadding);
        let mut recv = cbc_decryptor(KeySize128, &password_hash[..], &server_iv[..], NoPadding);

        let mut challenge = [0u8; 32];
        recv.decrypt(&mut RefReadBuffer::new(&data[..32]), &mut RefWriteBuffer::new(&mut challenge[..32]), false).unwrap();

        for i in 0..16 {
            challenge[i] ^= challenge[i + 16];
        }

        send.encrypt(&mut RefReadBuffer::new(&challenge[..16]), &mut RefWriteBuffer::new(&mut packet_data[..16]), false).unwrap();
        stream.write_all(&packet_data[..16]).await?;
        stream.read_exact(&mut packet_data[..16]).await.expect("Wrong password");

        if u16::from(packet_data[14]) + u16::from(packet_data[15]) * 256 != 1 {
            panic!("Invalid protocol version");
        }

        Ok(Self {

        })
    }

    fn random_init_vector() -> [u8; 16] {
        rand::random()
    }
}