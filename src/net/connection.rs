use crate::net::CryptRead;
use crate::net::CryptWrite;
use crate::net::Packet;
use std::io::Write;
use std::net::Shutdown;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread;
use crate::Error;

pub struct Connection;

impl Connection {
    pub fn new(
        addr: &SocketAddr,
        max_recv_load: u32,
    ) -> Result<(ConnectionRead, ConnectionWrite), Error> {
        let stream = TcpStream::connect(addr)?;
        stream.set_nodelay(true)?;

        Ok((
            ConnectionRead {
                max_recv_load,
                read: CryptRead::new(stream.try_clone()?),
            },
            ConnectionWrite {
                stream: stream.try_clone()?,
                write: CryptWrite::new(stream),
            },
        ))
    }
}

pub struct ConnectionRead {
    max_recv_load: u32,
    read: CryptRead<TcpStream>,
}

impl ConnectionRead {
    pub fn spawn<S: FnMut(Packet) + Send + 'static>(mut self, mut sink: S) {
        thread::Builder::new()
            .name(String::from("ConnectionRead"))
            .spawn(move || {
                // TODO check shutdown behavior
                loop {
                    let packet = Packet::from_reader(self.max_recv_load, &mut self.read)
                        .expect("Failed to read packet");
                    sink(packet);
                }
            })
            .unwrap();
    }
}

pub struct ConnectionWrite {
    stream: TcpStream,
    write: CryptWrite<TcpStream>,
}

impl ConnectionWrite {
    pub fn flush(&mut self) -> Result<(), Error> {
        self.write.flush()?;
        Ok(())
    }

    pub fn send(&mut self, packet: &Packet) -> Result<(), Error> {
        packet.write_to(&mut self.write)?;
        self.flush()?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}
