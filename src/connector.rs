
use std::net::ToSocketAddrs;

use std::thread;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

use sha2::Digest;
use sha2::Sha512;

use Version;
use Error;
use Block;
use BlockManager;
use IndexList;
use Player;

use net::Packet;
use net::Connection;
use net::BinaryWriter;

use message::from_reader;
use message::FlattiverseMessage;

pub const PROTOCOL_VERSION  : u32       = 34;
pub const CONNECTOR_VERSION : Version   = Version::new(0, 9, 5, 0);

pub struct ConnectorData {
    players: Mutex<IndexList<Player>>
}

impl ConnectorData {
    pub fn player(&self, index: u16) -> Option<Arc<Player>> {
        self.players
            .lock()
            .expect("Failed to acquire lock")
            .get(index as usize)
    }
}

pub struct Connector {
    data: Arc<ConnectorData>,
    connection: Arc<Mutex<Connection>>,
    block_manager: Arc<Mutex<BlockManager>>
}

impl Connector {
    pub fn new(email: &str, password: &str, compression_enabled: bool) -> Result<Connector, Error> {
        // param check
        if email.len() < 6 || email.len() > 256 || password.is_empty() {
            return Err(Error::EmailAndOrPasswordInvalid);
        }

        let addr = "galaxy.flattiverse.com:22".to_socket_addrs()?.next().unwrap();

        // TODO missing block manager
        // TODO missing addConnectionClosedListener
        // TODO missing addPacketReceivedListener

        // TODO missing
        /*
        this.players            = new UniversalHolder<>(playersArray);
        this.universeGroups     = new UniversalHolder<>(universeGroupsArray);
        this.controllables      = new UniversalHolder<>(controllablesArray);
        this.crystals           = new UniversalHolder<>(crystalsArray);
        */

        let (tx, rx) = channel();

        let mut connector = Connector {
            data: Arc::new(ConnectorData{
                players: Mutex::new(IndexList::new(false, 512))
            }),
            connection: Arc::new(Mutex::new(Connection::new(&addr, 262144, tx)?)),
            block_manager: Arc::new(Mutex::new(BlockManager::new()))
        };

        let block_manager = connector.block_manager.clone();
        let connection = connector.connection.clone();
        let data = connector.data.clone();
        thread::spawn(move || {
            // capture
            let block_manager = block_manager;
            let connection = connection;
            let data = data;
            let rx = rx;

            loop {
                let packet = match rx.recv() {
                    Ok(packet) => packet,
                    Err(e) => {
                        println!("Receiving failed: {:?}", e);
                        break;
                    }
                };
                println!("Received packet: {:?}", packet);

                if packet.session() != 0x00 {
                    let mut lock = block_manager.lock().expect("Failed to acquire lock");
                    lock.answer(packet);
                    continue;
                }

                match packet.command() {
                    0x01 => {
                        println!("Received ping request");
                        let mut lock = connection.lock().expect("Failed to acquire lock");
                        lock.send(&packet).expect("Failed to respond to ping");
                    },
                    0x30 => { // new message
                        match from_reader(data.clone(), &packet) {
                            Err(e) => println!("Failed to decode message: {:?}", e),
                            Ok(message) => {
                                println!("{}", message);
                            }
                        };
                    },
                    _ => {
                        println!("Received packet with unimplemented command: {:?}", packet);
                    }
                }
            }
        });

        connector.login(email, password, compression_enabled)?;
        Ok(connector)
    }

    fn login(&mut self, email: &str, password: &str, compression_enabled: bool) -> Result<(), Error> {
        let mut lock = self.block_manager.lock().expect("Failed to acquire lock");
        let mut block = lock.block()?;
        let mut packet = Packet::new();

        drop(lock);

        {
            let writer = (&mut packet.write()) as &mut BinaryWriter;

            println!("write protocol version");
            writer.write_u32(PROTOCOL_VERSION)?;
            println!("write platform kind");
            writer.write_byte(0u8)?; // platform kind
            println!("write connector version");
            writer.write_u32(CONNECTOR_VERSION.raw())?;

            // login features: 0b00000001 = Performance data
            let mut features = 0u8;

            // TODO feature performance mark
            // TODO feature compression

            println!("write features");
            writer.write_byte(features)?;

            // TODO write performance mark

            writer.write_string(&email)?;
            let mut hasher = Sha512::default();
            hasher.input(password.as_bytes());
            println!("Hasher result: {:?}", &hasher.result()[..]);
            writer.write_all(&hasher.result()[..]);
        }

        {
            let mut block = block.lock().expect("Failed to acquire lock");
            packet.set_session(block.id());
            self.connection.lock().expect("Failed to acquire lock").send(&packet)?;
            let response = block.wait()?;
            println!("Response: {:?}", response);
        }

        Ok(())

    }
}