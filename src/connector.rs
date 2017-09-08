
use std::net::ToSocketAddrs;

use std::thread;

use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc::channel;

use sha2::Digest;
use sha2::Sha512;
use hostname;

use Task;
use Version;
use Error;
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


pub struct Connector {
    players: Mutex<IndexList<Player>>,
    connection: Mutex<Connection>,
    block_manager: BlockManager,
    player: Option<Arc<RwLock<Player>>>,
    sync_account_queries: Mutex<()>,
    tasks: RwLock<IndexList<bool>>
}

impl Connector {
    pub fn new(email: &str, password: &str, compression_enabled: bool) -> Result<Arc<Connector>, Error> {
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

        let connector = Connector {
            players: Mutex::new(IndexList::new(false, 512)),
            connection: Mutex::new(Connection::new(&addr, 262144, tx)?),
            block_manager: BlockManager::new(),
            player: None,
            sync_account_queries: Mutex::new(()),
            tasks: RwLock::new(IndexList::new(false, 32)),
        };

        let connector = Arc::new(connector);
        let connector_thread = connector.clone();

        thread::spawn(move || {
            // capture
            let connector = connector_thread;
            let rx = rx;

            loop {
                let packet = rx.recv().expect("Failed to retrieve packet");
                println!("Received packet: {:?}", packet);

                // answer the ping beforehand
                if packet.session() != 0x00 {
                    connector.answer(packet);
                    continue;
                }

                match packet.command() {
                    0x01 => {
                        println!("Received ping request");
                        connector.send(&packet).expect("Failed to respond to ping");
                    },
                    0x30 => { // new message
                        match from_reader(connector.clone(), &packet) {
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

    fn login(&self, email: &str, password: &str, compression_enabled: bool) -> Result<(), Error> {
        let block = self.block_manager.block()?;
        let mut packet = Packet::new();

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
            writer.write_all(&hasher.result()[..])?;
        }

        {
            let mut block = block.lock().expect("Failed to acquire lock");
            packet.set_session(block.id());
            self.send(&packet)?;
            let response = block.wait()?;
            println!("Response: {:?}", response);
        }

        Ok(())

    }

    fn answer(&self, answer: Box<Packet>) {
        self.block_manager.answer(answer)
    }

    pub fn send(&self, packet: &Packet) -> Result<(), Error> {
        self.connection.lock().unwrap().send(packet)
    }

    pub fn player(&self) -> &Option<Arc<RwLock<Player>>> {
        &self.player
    }

    pub fn player_for(&self, index: u16) -> Option<Arc<Player>> {
        self.players
            .lock()
            .unwrap()
            .get(index as usize)
    }

    pub fn weak_player_for(&self, index: u16) -> Option<Weak<Player>> {
        self.players
            .lock()
            .unwrap()
            .get_weak(index as usize)
    }

    pub fn block_manager(&self) -> &BlockManager {
        &self.block_manager
    }

    pub fn register_task_quitely_if_unknown(&self, task: Task) {
        match self.register_task_if_unknown(task) {
            Ok(_) => {}, // fine
            Err(ref e) => {
                println!("'register_task_if_unknown' failed: {:?}", e);
            }
        }
    }

    pub fn register_task_if_unknown(&self, task: Task) -> Result<(), Error> {
        let read = self.tasks.read()?;
        let option = read.get(task as usize);
        if option.is_none() || !*option.unwrap() {
            self.register_task(task)?;
        }
        Ok(())
    }

    pub fn register_task(&self, task: Task) -> Result<(), Error> {
        let block = self.block_manager().block()?;

        let mut packet = Packet::new();

        {
            let block = block.lock()?;
            packet.set_session(block.id());
            packet.set_command(0x07u8);
            packet.set_path_sub(task as u8);
        }

        self.send(&packet)?;
        block.lock()?.wait()?;

        self.tasks.write()?.set(task as usize, Some(Arc::new(true)));
        Ok(())
    }

    pub(crate) fn sync_account_queries(&self) -> &Mutex<()> {
        &self.sync_account_queries
    }

    pub fn hostname() -> String {
        hostname::get_hostname().expect("Failed to retrieve hostname")
    }
}