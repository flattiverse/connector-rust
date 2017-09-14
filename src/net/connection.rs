
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::Shutdown;

use std::thread;
use std::sync::mpsc::Sender;
use std::io::Write;

use net::Packet;
use net::CryptRead;
use net::CryptWrite;

use Error;


pub struct Connection {
    stream: TcpStream,
    write: CryptWrite<TcpStream>
}

impl Connection {
    pub fn new(addr: &SocketAddr, max_recv_load: u32, sink: Sender<Box<Packet>>) -> Result<Connection, Error> {
        let stream= TcpStream::connect(addr)?;
        stream.set_nodelay(true)?;

        let stream_reader = stream.try_clone()?;

        thread::spawn(move || {
            // capture
            let mut reader = CryptRead ::new(stream_reader);

            // TODO check shutdown behavior
            loop {
                let packet = Box::new(Packet::from_reader(
                    max_recv_load,
                    &mut reader
                ).expect("Failed to read packet"));
                sink.send(packet).expect("Failed to send new package")
            }
        });


        Ok(Connection{
            write: CryptWrite::new(stream.try_clone()?),
            stream: stream,
        })
    }

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