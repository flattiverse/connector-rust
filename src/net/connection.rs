
use std::io::Error;
use std::net::SocketAddr;
use std::net::TcpStream;


pub struct Connection {

}

impl Connection {
    pub fn new(addr: &SocketAddr) -> Result<Connection, Error> {
        let stream_read= TcpStream::connect(addr)?;
        let stream_write = stream_read.try_clone()?;

        Ok(Connection{

        })
    }
}