use std::collections::hash_map::RandomState;
use std::io::Error;

use tokio::codec::Framed;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::codec::Flattiverse;

pub struct Connection {

}

impl Connection {
    pub async fn connect(user: &str, password: &str) -> Result<Connection, Error> {
        let iv = Self::random_init_vector();
        let mut ip = [0u8; 64];

        let user_hash = crate::crypt::sha512(&user.to_lowercase());
        let password_hash = crate::crypt::hash_password(user, password);

        (&mut ip[0..16]).copy_from_slice(&iv);
        for i in 0..32 {
            ip[i + 16] = user_hash[i] ^ iv[i % 16];
        }

        let mut stream = TcpStream::connect("galaxy.flattiverse.com:80").await?;
        stream.set_nodelay(true)?;

        //let mut framed = Framed::new(stream, Flattiverse);


        stream.write_all(&ip[..]).await?;


        Ok(Self {

        })
    }

    fn random_init_vector() -> [u8; 16] {
        rand::random()
    }
}