
use std::io::Error;


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::crypt::Aes128Cbc;
use block_modes::BlockMode;


pub struct Connection {

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

        recv.decrypt(&mut packet_data[16..16+32]).unwrap();
        for i in 16..32 {
            packet_data[i] = packet_data[i] ^ packet_data[i + 16];
        }

        //send.encrypt(&mut RefReadBuffer::new(&challenge[..16]), &mut RefWriteBuffer::new(&mut packet_data[..16]), false).unwrap();
        send.encrypt(&mut packet_data[16..32], 16);
        stream.write_all(&packet_data[16..32]).await?;
        stream.read_exact(&mut packet_data[..16]).await.expect("Wrong password");

        let version = u16::from(packet_data[14]) + u16::from(packet_data[15]) * 256;
        println!("Flattiverse version: {}", version);

        if version != 1 {
            panic!("Invalid protocol version");
        }

        Ok(Self {

        })
    }

    fn random_init_vector() -> [u8; 16] {
        rand::random()
    }
}