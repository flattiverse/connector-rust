
use std::net::ToSocketAddrs;

use std::io;
use std::thread;

use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::cmp::min;

use sha2::Digest;
use sha2::Sha512;
use hostname;

use Task;
use Team;
use Error;
use Scores;
use Player;
use Version;
use Universe;
use DateTime;
use TimeSpan;
use IndexList;
use Tournament;
use BlockManager;
use ManagedArray;
use UniverseGroup;
use UniversalHolder;
use UniversalEnumerable;
use UniverseGroupFlowControl;

use net::Packet;
use net::Connection;
use net::BinaryWriter;
use net::BinaryReader;

use controllable;
use controllable::Controllable;
use controllable::ControllableData;

use unit;
use unit::ControllableInfo;

use item;
use item::CargoItem;
use item::CrystalCargoItem;
use item::CrystalCargoItemData;

use message::from_reader;
use message::FlattiverseMessage;

pub const PROTOCOL_VERSION  : u32       = 34;
pub const CONNECTOR_VERSION : Version   = Version::new(0, 9, 5, 0);

pub const TASK_COUNT : usize   = 32;

pub struct Connector {
    connection: Mutex<Connection>,
    block_manager: BlockManager,
    player:     RwLock<Weak<RwLock<Player>>>,
    players:    RwLock<UniversalHolder<Player>>,
    sync_account_queries:   Mutex<()>,
    sync_control_flow:      Mutex<()>,

    tick:           RwLock<u16>,
    tasks:          RwLock<[bool; TASK_COUNT]>,
    flows:          RwLock<Vec<Arc<UniverseGroupFlowControl>>>,
    uni_groups:     RwLock<ManagedArray<Arc<RwLock<UniverseGroup>>>>,
    crystals:       RwLock<ManagedArray<Arc<RwLock<Box<CrystalCargoItem>>>>>,
    controllables:  RwLock<ManagedArray<Arc<RwLock<Box<Controllable>>>>>,
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
            sync_control_flow:    Mutex::new(()),
            tick:           RwLock::new(0_u16),
            tasks:          RwLock::new([false; TASK_COUNT]),
            flows:          RwLock::new(Vec::new()),
            uni_groups:     RwLock::new(ManagedArray::with_capacity(128)),
            crystals:       RwLock::new(ManagedArray::with_capacity(64)),
            controllables:  RwLock::new(ManagedArray::with_capacity(256)),
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
            0x01 => { // ping
                connector.send(&packet).expect("Failed to respond to ping");
            },
            0x02 => { // pre-wait
                let tick = packet.path_sub() as u16;
                *connector.tick.write()? = tick;

                // sync flow control
                let lock = connector.sync_control_flow.lock()?;

                let player = connector.player.read()?.upgrade().ok_or(Error::PlayerNotAvailable)?;
                let player = player.read()?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                let group = group.read()?;

                let time = DateTime::now() + TimeSpan::new(
                    group.avg_tick_time().ticks()
                        - min(
                            player.ping().ticks() +384_000_i64,
                            group.avg_tick_time().ticks()
                        )
                );

                for flow in connector.flows.read()?.iter() {
                    flow.set_pre_wait(time.clone(), tick)?;
                }
            },
            0x03 => { // wait
                // sync flow control
                let lock = connector.sync_control_flow.lock()?;
                let flows= connector.flows.read()?;

                if flows.is_empty() {
                    let mut packet = Packet::new();

                    {
                        packet.set_command(0x05);
                        packet.set_path_sub(*connector.tick.read()? as u8 & 0xFF);
                    }

                    connector.send(&packet)?;
                } else {
                    let mut allow_flow_control_ready = true;
                    for flow in flows.iter() {
                        if !flow.ready()? {
                            allow_flow_control_ready = false;
                            break;
                        }
                    }

                    if allow_flow_control_ready {
                        let mut packet = Packet::new();

                        {
                            packet.set_command(0x05);
                            packet.set_path_sub(*connector.tick.read()? as u8 & 0xFF);
                        }

                        connector.send(&packet)?;
                    }
                }

                let player = connector.player.read()?.upgrade().ok_or(Error::PlayerNotAvailable)?;
                let player = player.read()?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                let group = group.read()?;

                let time = DateTime::now() + TimeSpan::new(
                    group.avg_tick_time().ticks()
                        - min(
                        player.ping().ticks() +384_000_i64,
                        group.avg_tick_time().ticks()
                    )
                );

                for flow in connector.flows.read()?.iter() {
                    flow.set_wait(time.clone())?;
                }


            },
            0x07 => { // TaskList
                let mut tasks = connector.tasks.write()?;
                let read = packet.read();
                for i in 0..32 {
                    if read[i] == 1 {
                        tasks[i] = true;
                    }
                }
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
            0x13 => { // registered for UniverseGroup / Team
                let player = connector.player_for(packet.path_player())?;
                let group = connector.universe_group(packet.path_universe_group())?;

                {
                    let mut player = player.write()?;
                    player.set_universe_group(Arc::downgrade(&group));
                    player.set_team(group.read()?.team_weak(packet.path_sub()));
                }

                {
                    let mut group = group.write()?;
                    group.players().write()?.insert(player.clone())?;
                }

                {
                    let mut player = player.write()?;
                    player.update_assignment(&packet)?;
                    println!("Updated assignement, clan: {:?}", player.clan());
                }

            },
            0x14 => { // player timing information
                let player = connector.player_for(packet.path_player())?;
                player.write()?.update_timing(&packet)?;
            },
            0x15 => { // unregistered from UniverseGroup / Team
                let player = connector.player_for(packet.path_player())?;

                {
                    let player_read = player.read()?;
                    let group = player_read.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                    let group_read = group.read()?;
                    let player_id = player_read.id();

                    drop(player_read);

                    // TODO WTF!
                    let mut players = group_read.players().write()?;
                    let mut index = 0_isize;
                    let mut wipe_at = -1_isize;
                    for player in players.as_ref() {
                        if let &Some(ref player) = player {
                            let lock = player.read()?;
                            if lock.id() == player_id {
                                wipe_at = index;
                                break;
                            }
                        }
                        index += 1;
                    }
                    if wipe_at >= 0 {
                        players.wipe_index(wipe_at as usize);
                    } else {
                        panic!("...");
                    }
                }

                {
                    let mut player_write = player.write()?;
                    player_write.set_team(Weak::default());
                    player_write.clear_assignment();
                }

            },
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
            0x1B => { // crystal list update
                let len = packet.read().len();
                let reader = &mut packet.read() as &mut BinaryReader;
                let mut lock = connector.crystals.write()?;

                let mut crystal_position = 0;

                while crystal_position < 64 {
                    // TODO WTF
                    let cargo_item = match item::cargo_item_from_reader(Arc::downgrade(&connector), true, reader) {
                        Ok(item) => item,
                        Err(e) => {
                            let ok = match e {
                                Error::IoError(ref bt, ref inner_e) => {
                                    match inner_e.kind() {
                                        io::ErrorKind::UnexpectedEof => {
                                            // end of stream? --> no more items
                                            true
                                        }
                                        _ => false,
                                    }
                                },
                                _ => false
                            };
                            if !ok {
                                return Err(e);
                            } else {
                                break;
                            }
                        }

                    };
                    let crystal : Box<CrystalCargoItem> = cargo_item.downcast::<CrystalCargoItemData>().unwrap();

                    lock.set(crystal_position, Some(Arc::new(RwLock::new(crystal))));
                    crystal_position += 1;
                }

                while crystal_position < 64 {
                    lock.set(crystal_position, None);
                    crystal_position += 1;
                }

            },
            0x20 => { // new universe group
                let group = UniverseGroup::from_reader(&connector, &packet)?;
                println!("New UniverseGroup: {}", group.name());
                let mut write = connector.uni_groups.write()?;
                write.set(packet.path_universe_group() as usize, Some(Arc::new(RwLock::new(group))));
            },
            0x24 => { // new universe
                let group = connector.universe_group(packet.path_universe_group())?;
                let reader = &mut packet.read() as &mut BinaryReader;
                let universe = Universe::from_reader(&group, packet, reader)?;
                println!("New Universe: {}", universe.name());

                group.write()?.set_universe(packet.path_universe(), Some(Arc::new(RwLock::new(universe))));
            },
            0x28 => { // new team
                let group = connector.universe_group(packet.path_universe_group())?;
                let reader = &mut packet.read() as &mut BinaryReader;
                let team = Team::from_reader(Arc::downgrade(connector), &group, packet, reader)?;
                println!("New Team: {}", team.name());
                group.write()?.set_team(packet.path_sub(), Some(Arc::new(RwLock::new(team))));
            },
            0x2B => { // team score update
                let group = connector.universe_group(packet.path_universe_group())?;
                let team = group
                    .read()?
                    .team(packet.path_sub())
                    .clone()
                    .ok_or(Error::InvalidTeam(packet.path_sub()))?;

                let team = team.read()?;
                match team.scores() {
                    &None => return Err(Error::ScoresNotAvailable),
                    &Some(ref scores) => {
                        scores
                            .write()?
                            .update(&mut packet.read() as &mut BinaryReader)?;
                    }
                };
            }
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
            0x60 => { // create tournament
                let group = connector.universe_group(packet.path_universe_group())?;
                let reader = &mut packet.read() as &mut BinaryReader;
                let tournament = Tournament::from_reader(
                    Arc::downgrade(connector),
                    &group,
                    packet,
                    reader
                )?;

                group.write()?.set_tournament(Some(Arc::new(RwLock::new(tournament))));
            },
            0x61 => { // update tournament
                let group = connector.universe_group(packet.path_universe_group())?;
                match group.read()?.tournament() {
                    &None => return Err(Error::TeamNotAvailable),
                    &Some(ref tournament) => {
                        tournament.write()?.update(packet)?;
                    }
                };

                if packet.read().len() == 0 {
                    group.write()?.set_tournament(None);
                }
            },
            0x80 => { // 'ControllableStaticPacket'
                connector.controllables.write()?.set(
                    packet.path_ship() as usize,
                    Some(controllable::from_packet(&connector, packet, &mut packet.read() as &mut BinaryReader)?)
                );
            },
            0x81 => { // 'ControllableDynamicPacket'
                match connector.controllables.read()?.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.write()?.downcast_mut::<ControllableData>().unwrap().update(packet)?;
                    }
                }
            },
            0x82 => { // 'ControllableRemoved'
                let mut controllables = connector.controllables.write()?;
                match controllables.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.write()?.downcast_mut::<ControllableData>().unwrap().set_active(false);
                    }
                };
                controllables.wipe_index(packet.path_ship() as usize);
            },
            0x83 => { // 'ControllableDynamicExtendedPacket'
                match connector.controllables.read()?.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.write()?.downcast_mut::<ControllableData>().unwrap().update_extended(packet)?;
                    }
                }
            },
            0x84 => { // 'ControllableInfoStaticPacket'
                let player = connector.player_for(packet.path_player())?;
                player.write()?.set_controllable_info(
                    packet.path_ship(),
                    Some(Arc::new(RwLock::new(ControllableInfo::from_packet(packet, Arc::downgrade(&player))?))),
                );
            },
            0x85 => { // 'ControllableInfoDynamicPacket'
                let player = connector.player_for(packet.path_player())?;
                match player.read()?.controllable_info(packet.path_ship()) {
                    None => return Err(Error::InvalidControllableInfo(packet.path_ship())),
                    Some(info) => info.write()?.update(packet)?,
                };
            },
            0x86 => { // 'ControllableInfoRemoved'
                let player = connector.player_for(packet.path_player())?;
                let path_ship = packet.path_ship();

                match player.read()?.controllable_info(path_ship) {
                    None => return Err(Error::InvalidControllableInfo(path_ship)),
                    Some(info) => info.write()?.set_active(false)
                };
                player.write()?.set_controllable_info(path_ship, None);
            },
            0x87 => { // 'ControllableInfoScorePacket'
                let path_ship = packet.path_ship();
                let reader = &mut packet.read() as &mut BinaryReader;
                let player = connector.player_for(packet.path_player())?;
                let player = player.read()?;
                match player.controllable_info(path_ship) {
                    None => return Err(Error::InvalidControllableInfo(path_ship)),
                    Some(ref info) => {
                        let info = info.read()?;
                        let mut scores = info.scores().write()?;
                        scores.update(reader)?
                    },
                }
            },
            0x88 => { // 'ControllableInfoCrystalPacket'
                let mut crystals = Vec::new();

                {
                    let reader = &mut packet.read() as &mut BinaryReader;
                    loop {
                        let crystal = match item::cargo_item_from_reader(Arc::downgrade(&connector), true, reader) {
                            Ok(item) => item,
                            Err(e) => {
                                let ok = match e {
                                    Error::IoError(ref bt, ref inner_e) => {
                                        match inner_e.kind() {
                                            io::ErrorKind::UnexpectedEof => {
                                                // end of stream? --> no more items
                                                true
                                            }
                                            _ => false,
                                        }
                                    },
                                    _ => false
                                };
                                if !ok {
                                    return Err(e);
                                } else {
                                    break;
                                }
                            }
                        };

                        let crystal : Box<CrystalCargoItem> = crystal.downcast::<CrystalCargoItemData>().unwrap();

                        crystals.push(Arc::new(RwLock::new(crystal)));
                    }
                }


                let player = connector.player_for(packet.path_player())?;
                let player = player.read()?;
                let info   = player.controllable_info(packet.path_ship()).ok_or(Error::InvalidControllableInfo(packet.path_ship()))?;
                info.write()?.set_crystals(crystals);
            },
            0x89 => { // 'ControllableCrystalPacket'
                let mut crystals = Vec::new();

                {
                    let reader = &mut packet.read() as &mut BinaryReader;
                    loop {
                        let crystal = match item::cargo_item_from_reader(Arc::downgrade(&connector), true, reader) {
                            Ok(item) => item,
                            Err(e) => {
                                let ok = match e {
                                    Error::IoError(ref bt, ref inner_e) => {
                                        match inner_e.kind() {
                                            io::ErrorKind::UnexpectedEof => {
                                                // end of stream? --> no more items
                                                true
                                            }
                                            _ => false,
                                        }
                                    },
                                    _ => false
                                };
                                if !ok {
                                    return Err(e);
                                } else {
                                    break;
                                }
                            }
                        };

                        let crystal : Box<CrystalCargoItem> = crystal.downcast::<CrystalCargoItemData>().unwrap();

                        crystals.push(Arc::new(RwLock::new(crystal)));
                    }
                }

                let controllable = connector.controllable(packet.path_ship())?;
                controllable.write()?.set_crystals(crystals);
            },
            0x8A => { // 'ControllableInfoCargoPacket'
                let mut cargo_items = Vec::new();

                {
                    let reader = &mut packet.read() as &mut BinaryReader;
                    loop {
                        let crystal = match item::cargo_item_from_reader(Arc::downgrade(&connector), false, reader) {
                            Ok(item) => item,
                            Err(e) => {
                                let ok = match e {
                                    Error::IoError(ref bt, ref inner_e) => {
                                        match inner_e.kind() {
                                            io::ErrorKind::UnexpectedEof => {
                                                // end of stream? --> no more items
                                                true
                                            }
                                            _ => false,
                                        }
                                    },
                                    _ => false
                                };
                                if !ok {
                                    return Err(e);
                                } else {
                                    break;
                                }
                            }
                        };

                        cargo_items.push(Arc::new(RwLock::new(crystal)));
                    }
                }

                let player = connector.player_for(packet.path_player())?;
                let info = player.read()?.controllable_info(packet.path_ship()).ok_or(Error::InvalidControllable(packet.path_ship()))?;
                info.write()?.set_cargo_items(cargo_items);
            },
            0x8B => { // 'ControllableInfoCargoPacket'
                let mut cargo_items = Vec::new();

                {
                    let reader = &mut packet.read() as &mut BinaryReader;
                    loop {
                        let crystal = match item::cargo_item_from_reader(Arc::downgrade(&connector), false, reader) {
                            Ok(item) => item,
                            Err(e) => {
                                let ok = match e {
                                    Error::IoError(ref bt, ref inner_e) => {
                                        match inner_e.kind() {
                                            io::ErrorKind::UnexpectedEof => {
                                                // end of stream? --> no more items
                                                true
                                            }
                                            _ => false,
                                        }
                                    },
                                    _ => false
                                };
                                if !ok {
                                    return Err(e);
                                } else {
                                    break;
                                }
                            }
                        };

                        cargo_items.push(Arc::new(RwLock::new(crystal)));
                    }
                }

                let controllable = connector.controllable(packet.path_ship())?;
                controllable.write()?.set_cargo_items(cargo_items);
            },
            // TODO missing entries
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
        let known = self.tasks.read()?[task as usize];
        if !known {
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

        self.tasks.write()?[task as usize] = true;
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

    pub fn universe_groups(&self) -> &RwLock<ManagedArray<Arc<RwLock<UniverseGroup>>>> {
        &self.uni_groups
    }

    pub fn universe_group(&self, index: u16) -> Result<Arc<RwLock<UniverseGroup>>, Error> {
        let lock = self.uni_groups.read()?;
        lock.get(index as usize).clone().ok_or(Error::InvalidUniverseGroup(index))
    }

    pub fn crystals(&self, name: &str) -> Result<Arc<RwLock<Box<CrystalCargoItem>>>, Error> {
        let crystals = self.crystals.read()?;
        for i in 0..crystals.len() {
            match crystals.get(i) {
                &None => {},
                &Some(ref arc) => {
                    if arc.read()?.name().eq(name) {
                        return Ok(arc.clone());
                    }
                },
            };
        };
        Err(Error::InvalidCrystalName(String::from(name)))
    }

    pub fn controllable(&self, index: u8) -> Result<Arc<RwLock<Box<Controllable>>>, Error> {
        self.controllables.read()?.get(index as usize).clone().ok_or(Error::InvalidControllable(index))
    }

    pub fn controllable_weak(&self, index: u8) -> Weak<RwLock<Box<Controllable>>> {
        match self.controllables.read().unwrap().get(index as usize) {
            &None => Weak::default(),
            &Some(ref arc) => Arc::downgrade(arc),
        }
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

trait Test {

}

impl<T> Test for (usize, [T]) {

}