
use std::net::ToSocketAddrs;

use std::thread;

use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use sha2::Digest;
use sha2::Sha512;
use hostname;

use Task;
use Team;
use Error;
use Player;
use Version;
use IndexList;
use BlockManager;
use UniverseGroup;
use UniversalHolder;
use UniverseGroupFlowControl;

use net::Packet;
use net::Connection;
use net::BinaryWriter;
use net::BinaryReader;

use controllable::Controllable;
use item::CrystalCargoItem;

use message::from_reader;
use message::FlattiverseMessage;

pub const PROTOCOL_VERSION  : u32       = 34;
pub const CONNECTOR_VERSION : Version   = Version::new(0, 9, 5, 0);


pub struct Connector {
    connection: Mutex<Connection>,
    block_manager: BlockManager,
    player:     RwLock<Weak<RwLock<Player>>>,
    players:    RwLock<UniversalHolder<Player>>,
    sync_account_queries: Mutex<()>,

    tick:       RwLock<u16>,
    tasks:      RwLock<IndexList<bool>>,
    flows:      RwLock<Vec<Arc<UniverseGroupFlowControl>>>,
}

impl Connector {
    pub fn new(email: &str, password: &str, compression_enabled: bool) -> Result<(Arc<Connector>, Receiver<Box<FlattiverseMessage>>), Error> {
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
        let (message_sender, message_receiver) = channel();

        let connector = Connector {
            players: RwLock::new(UniversalHolder::new(IndexList::new(false, 512))),
            connection: Mutex::new(Connection::new(&addr, 262144, tx)?),
            block_manager: BlockManager::new(),
            player: RwLock::new(Weak::default()),
            sync_account_queries: Mutex::new(()),
            tick:  RwLock::new(0_u16),
            tasks: RwLock::new(IndexList::new(false, 32)),
            flows: RwLock::new(Vec::new()),
        };

        let connector = Arc::new(connector);
        let connector_thread = connector.clone();

        thread::spawn(move || {
            // capture
            let connector = connector_thread;
            let messages = message_sender;
            let rx = rx;

            loop {
                let packet = rx.recv().expect("Failed to retrieve packet");
                println!("Received packet: {:?}", packet);

                // answer the ping beforehand
                if packet.session() != 0x00 {
                    connector.answer(packet);
                    continue;
                }

                match Connector::handle_packet(&connector, &packet, &messages) {
                    Ok(_) => {},
                    Err(ref e) => {
                        println!("Failed to handle message {:?}: {:?}", e, packet);
                    }
                };
            }
        });

        connector.login(email, password, compression_enabled)?;
        Ok((connector, message_receiver))
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

    fn handle_packet(connector: &Arc<Connector>, packet: &Packet, messages: &Sender<Box<FlattiverseMessage>>) -> Result<(), Error> {
        match packet.command() {
            0x01 => {
                println!("Received ping request");
                connector.send(&packet).expect("Failed to respond to ping");
            },
            0x0F => { // assign player
                let mut player_slot = connector.player.write()?;
                *player_slot = connector.players.read()?.get_for_index_weak(packet.path_player() as usize);
                println!("Player assigned: {:?} (id: {})", *player_slot, packet.path_player());
            },
            0x10 => { // new player
                let reader = &mut packet.read() as &mut BinaryReader;
                connector.players.write()?.set(
                    packet.path_player() as usize,
                    Some(Arc::new(RwLock::new(Player::from_reader(connector, packet, reader)?)))
                );
            },
            0x11 => { // player status update
                match connector.players.read()?.get_for_index(packet.path_player() as usize) {
                    None => return Err(Error::MissingPlayer(packet.path_player())),
                    Some(player) => {
                        println!("Updating player status of {}", player.read()?);
                        player.write()?.update_stats(packet)?;
                    }
                }
            },
            0x12 => { // player ping update
                match connector.players.read()?.get_for_index(packet.path_player() as usize) {
                    None => return Err(Error::MissingPlayer(packet.path_player())),
                    Some(player) => {
                        println!("Updating player ping of {}", player.read()?);
                        player.write()?.update_ping(&packet)?;
                        println!("Ping of {} is now {}ms", player.read()?, player.read()?.ping().millis());
                    }
                }
            },
            // TODO 0x13 missing
            0x14 => { // player timing information
                let player = connector.player_for(packet.path_player())?;
                player.write()?.update_timing(&packet)?;
            },
            // TODO 0x15 missing
            0x16 => { // player isn't online anymore
                let player = connector.player_for(packet.path_player())?;
                player.write()?.set_online(false);
            },
            0x17 => { // player isn't active anymore
                {
                    let player = connector.player_for(packet.path_player())?;
                    player.write()?.set_active(false);
                }
                connector.players.write()?.set(packet.path_player() as usize, None);
            },
            0x30 => { // new message
                match from_reader(&connector, &packet) {
                    Err(e) => {
                        match e {
                            Error::IoError(ref backtrace, ref ioe) => println!("Backtrace {:?}", backtrace),
                            _ => {}
                        }
                        println!("Failed to decode message: {:?}", e)
                    },
                    Ok(message) => {
                        // println!("{}", message);
                        messages.send(message)?;
                    }
                };
            },
            _ => {
                println!("Received packet with unimplemented command: {:?}", packet);
            }
        };
        Ok(())
    }

    fn answer(&self, answer: Box<Packet>) {
        self.block_manager.answer(answer)
    }

    pub fn send(&self, packet: &Packet) -> Result<(), Error> {
        self.connection.lock()?.send(packet)
    }

    pub fn send_many(&self, packets: &[Packet]) -> Result<(), Error> {
        let mut connection = self.connection.lock()?;
        for i in 0..packets.len() {
            connection.send(&packets[i])?;
        }
        connection.flush()
    }

    pub fn player(&self) -> Weak<RwLock<Player>> {
        self.player.read().unwrap().clone()
    }

    pub fn player_for(&self, index: u16) -> Result<Arc<RwLock<Player>>, Error> {
        match self.players.read()?.get_for_index(index as usize) {
            None => Err(Error::MissingPlayer(index)),
            Some(arc) => Ok(arc)
        }
    }

    pub fn weak_player_for(&self, index: u16) -> Result<Weak<RwLock<Player>>, Error> {
        Ok(self.players.read()?.get_for_index_weak(index as usize))
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

    pub fn unregister_flow_control(&self, flow: &Arc<UniverseGroupFlowControl>) -> Result<bool, Error> {
        let mut vec = self.flows.write()?;
        for i in 0..vec.len() {
            if vec[i].eq(flow) {
                vec.remove(i);
                return Ok(true);
            }
        }
        return Ok(false);
    }

    pub fn flow_controls(&self) -> Result<bool, Error> {
        Ok(self.flows.read()?.len() > 0)
    }

    pub fn flow_control_check(&self, tick: u16) -> Result<bool, Error> {
        let self_tick = *self.tick.read()?;
        if tick != self_tick && tick != 0_u16 {
            return Ok(false);
        }

        {
            let lock = self.flows.read()?;
            for ref flow in lock.iter() {
                if flow.ready()? {
                    return Ok(true);
                }
            }
        }


        let block = self.block_manager.block()?;
        let mut packet = Packet::new();

        {
            let block = block.lock()?;
            packet.set_command(0x05);
            packet.set_session(block.id());

            // !!?!?!?!
            packet.set_path_sub(tick as u8);
        }

        self.send(&packet)?;
        let mut block = block.lock()?;
        match block.wait() {
            Ok(_) => Ok(true),
            Err(Error::TickIsGone) => Ok(false),
            Err(e) => Err(e)
        }
    }

    pub fn universe_group(&self, index: u16) -> Result<Arc<RwLock<UniverseGroup>>, Error> {
        unimplemented!()
    }

    pub fn team(&self, index: u16) -> Result<Arc<RwLock<Team>>, Error> {
        unimplemented!()
    }

    pub fn crystals(&self, name: &str) -> Option<Arc<CrystalCargoItem>> {
        unimplemented!();
    }

    pub fn controllable(&self, index: u8) -> Option<Arc<RwLock<Controllable>>> {
        unimplemented!();
    }

    pub fn controllable_weak(&self, index: u8) -> Option<Weak<RwLock<Controllable>>> {
        unimplemented!();
    }

    pub(crate) fn sync_account_queries(&self) -> &Mutex<()> {
        &self.sync_account_queries
    }

    pub fn check_name(name: &str) -> bool {
        if name.is_empty() || name.len() < 2 || name.len() > 64 {
            return false;
        }

        if name.starts_with(" ") || name.ends_with(" ") {
            return false;
        }

        for char in name.chars() {
            match char {
                'a'...'z' => continue,
                'A'...'Z' => continue,
                '0'...'9' => continue,
                '\u{192}'...'\u{214}' => continue,
                '\u{216}'...'\u{246}' => continue,
                '\u{248}'...'\u{687}' => continue,
                '\u{63696}'...'\u{63721}' => continue,
                '\u{63728}'...'\u{63737}' => continue,
                '\u{63741}'...'\u{63743}' => continue,
                ' '|'.'|'_'|'-' => continue,
                _ => return false,
            };
        }
        return true;
    }

    pub fn hostname() -> String {
        hostname::get_hostname().expect("Failed to retrieve hostname")
    }
}

pub trait ConnectorArc {
    fn register_flow_control(&self) -> Result<Arc<UniverseGroupFlowControl>, Error>;
}

impl ConnectorArc for Arc<Connector> {
    fn register_flow_control(&self) -> Result<Arc<UniverseGroupFlowControl>, Error> {
        let flow = Arc::new(UniverseGroupFlowControl::new(Arc::downgrade(self)));
        self.flows.write()?.push(flow.clone());
        Ok(flow)
    }
}