
use std::net::ToSocketAddrs;

use std::io;
use std::thread;

use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::cmp::min;
use std::time::Duration;

use sha2::Digest;
use sha2::Sha512;

use hostname;

use Task;
use Team;
use Error;
use Player;
use Vector;
use Version;
use Universe;
use DateTime;
use TimeSpan;
use IndexList;
use Tournament;
use PlatformKind;
use BlockManager;
use ManagedArray;
use UniverseGroup;
use UniversalHolder;
use UniversalEnumerable;
use UniverseGroupFlowControl;

use PerformanceMark;
use PerformanceTest;
use ManualResetEvent;

use net::Packet;
use net::Connection;
use net::BinaryWriter;
use net::BinaryReader;

use controllable::AnyControllable;
use controllable::ControllableDesign;

use unit::*;

use item::AnyCargoItem;
use item::CrystalCargoItem;

use message::AnyMessage;

pub const PROTOCOL_VERSION  : u32       = 35;
pub const CONNECTOR_VERSION : Version   = Version::new(0, 9, 6, 0);

pub const TASK_COUNT : usize   = 32;

pub struct Connector {
    connection: Mutex<Connection>,
    block_manager: BlockManager,
    player:                 RwLock<Weak<Player>>,
    players:                RwLock<UniversalHolder<Player>>,
    sync_account_queries:   Mutex<()>,
    sync_control_flow:      Mutex<()>,

    tick:           RwLock<u16>,
    tasks:          RwLock<[bool; TASK_COUNT]>,
    flows:          RwLock<Vec<Arc<UniverseGroupFlowControl>>>,
    uni_groups:     RwLock<ManagedArray<Arc<UniverseGroup>>>,
    crystals:       RwLock<ManagedArray<Arc<CrystalCargoItem>>>,
    controllables:  RwLock<ManagedArray<AnyControllable>>,

    receiver: Arc<Mutex<Receiver<AnyMessage>>>,

    benchmark: Option<PerformanceMark>,
}

impl Connector {
    pub fn new(email: &str, password: &str, compression_enabled: bool, benchmark: Option<PerformanceMark>) -> Result<Arc<Connector>, Error> {
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
            players: RwLock::new(UniversalHolder::new(IndexList::new(false, 4096))),
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

            receiver: Arc::new(Mutex::new(message_receiver)),
            benchmark,
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
        Ok(connector)
    }

    fn login(&self, email: &str, password: &str, compression_enabled: bool) -> Result<(), Error> {
        let block = self.block_manager.block()?;
        let mut packet = Packet::new();

        {
            let writer = (&mut packet.write()) as &mut BinaryWriter;

            writer.write_u32(PROTOCOL_VERSION)?;
            writer.write_byte(PlatformKind::Rust as u8)?;
            writer.write_u32(CONNECTOR_VERSION.raw())?;

            // login features: 0b00000001 = Performance data
            let mut features = 0u8;

            if self.benchmark.is_some() {
                features |= 0x01;
            }

            if compression_enabled {
                features |= 0x02;
            }

            writer.write_byte(features)?;

            if let Some(ref mark) = self.benchmark {
                mark.write(writer)?;
            }

            writer.write_string(&email)?;
            writer.write_all(&Self::sha512(password)[..])?;
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

    fn handle_packet(connector: &Arc<Connector>, packet: &Packet, messages: &Sender<AnyMessage>) -> Result<(), Error> {
        match packet.command() {
            0x02|0x03|0x14|0x20|0x24|0x28|0x30|0x81|0x83|0x84|0x85|0x90 => {},
            _id@_ => {
                // println!("## Processing command: 0x{:02x}", _id)
            },
        };

        match packet.command() {
            0x01 => { // ping
                connector.send(&packet)?;
            },
            0x02 => { // pre-wait
                let tick = packet.path_sub() as u16;
                *connector.tick.write()? = tick;

                // sync flow control
                let _ = connector.sync_control_flow.lock()?;
                let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;

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
                let _ = connector.sync_control_flow.lock()?;
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

                let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;

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
                    Some(Arc::new(Player::from_reader(connector, packet, reader)?))
                );
            },
            0x11 => { // player status update
                match connector.players.read()?.get_for_index(packet.path_player() as usize) {
                    None => return Err(Error::missing_player(packet.path_player())),
                    Some(player) => {
                        player.update_stats(packet)?;
                    }
                }
            },
            0x12 => { // player ping update
                match connector.players.read()?.get_for_index(packet.path_player() as usize) {
                    None => return Err(Error::missing_player(packet.path_player())),
                    Some(player) => {
                        player.update_ping(&packet)?;
                        // println!("Ping of {} is now {}ms", player, player.ping().millis());
                    }
                }
            },
            0x13 => { // registered for UniverseGroup / Team
                let player = connector.player_for(packet.path_player())?;
                let group = connector.universe_group(packet.path_universe_group())?;

                player.set_universe_group(Arc::downgrade(&group))?;
                player.set_team(group.team_weak(packet.path_sub()))?;

                group.players_mut()?.insert(player.clone())?;

                player.update_assignment(&packet)?;
                // println!("Updated assignement, clan: {:?}", player.clan());

            },
            0x14 => { // player timing information
                connector.player_for(packet.path_player())?.update_timing(&packet)?;
            },
            0x15 => { // unregistered from UniverseGroup / Team
                let player = connector.player_for(packet.path_player())?;

                {
                    let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                    let player_id = player.id();

                    // TODO WTF!
                    let mut players = group.players_mut()?;
                    let mut index = 0_isize;
                    let mut wipe_at = -1_isize;
                    for player in players.as_ref() {
                        if let &Some(ref player) = player {
                            if player.id() == player_id {
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

                player.set_team(Weak::default())?;
                player.clear_assignment();

            },
            0x16 => { // player isn't online anymore
                connector.player_for(packet.path_player())?.set_online(false)?;
            },
            0x17 => { // player isn't active anymore
                connector.player_for(packet.path_player())?.set_active(false)?;
                connector.players.write()?.set(packet.path_player() as usize, None);
            },
            0x1B => { // crystal list update
                let reader = &mut packet.read() as &mut BinaryReader;
                let mut lock = connector.crystals.write()?;

                let mut crystal_position = 0;

                while crystal_position < 64 {
                    let cargo_item = match AnyCargoItem::from_reader(&connector, reader, true) {
                        Ok(item) => item,
                        Err(e) => {
                            if let Error::IoError(_, ref inner_e) = e {
                                if let io::ErrorKind::UnexpectedEof = inner_e.kind() {
                                    // everything is fine, no more crystals
                                    break;
                                }
                            }
                            return Err(e);
                        }
                    };

                    if let AnyCargoItem::Crystal(crystal) = cargo_item {
                        lock.set(crystal_position, Some(crystal));
                        crystal_position += 1;

                    } else {
                        return Err(Error::not_crystal_cargo_item());
                    }
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
                write.set(packet.path_universe_group() as usize, Some(Arc::new(group)));
            },
            0x24 => { // new universe
                let group = connector.universe_group(packet.path_universe_group())?;
                let reader = &mut packet.read() as &mut BinaryReader;
                let universe = Universe::from_reader(&group, packet, reader)?;
                println!("New Universe: {}", universe.name());

                group.set_universe(packet.path_universe(), Some(Arc::new(universe)));
            },
            0x28 => { // new team
                let group = connector.universe_group(packet.path_universe_group())?;
                let reader = &mut packet.read() as &mut BinaryReader;
                let team = Team::from_reader(Arc::downgrade(connector), &group, packet, reader)?;
                println!("New Team: {:?}", team);
                group.set_team(packet.path_sub(), Some(Arc::new(team)));
            },
            0x2B => { // team score update
                let group = connector.universe_group(packet.path_universe_group())?;
                let team = group.team(packet.path_sub())?;

                match team.scores() {
                    &None => return Err(Error::ScoresNotAvailable),
                    &Some(ref scores) => {
                        scores.update(&mut packet.read() as &mut BinaryReader)?;
                    }
                };
            }
            0x30 => { // new message
                messages.send(AnyMessage::from_reader(&connector, &packet, &mut packet.read() as &mut BinaryReader)?)?;
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

                group.set_tournament(Some(Arc::new(tournament)))?;
            },
            0x61 => { // update tournament
                let group = connector.universe_group(packet.path_universe_group())?;
                match group.tournament() {
                    None => return Err(Error::TeamNotAvailable),
                    Some(ref tournament) => {
                        tournament.update(packet)?;
                    }
                };

                if packet.read().len() == 0 {
                    group.set_tournament(None)?;
                }
            },
            0x80 => { // 'ControllableStaticPacket'
                connector.controllables.write()?.set(
                    packet.path_ship() as usize,
                    Some(AnyControllable::from_packet(&connector, packet, &mut packet.read() as &mut BinaryReader)?)
                );
            },
            0x81 => { // 'ControllableDynamicPacket'
                match connector.controllables.read()?.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.update(packet)?;
                    }
                }
            },
            0x82 => { // 'ControllableRemoved'
                let mut controllables = connector.controllables.write()?;
                match controllables.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.set_active(false)?;
                    }
                };
                controllables.wipe_index(packet.path_ship() as usize);
            },
            0x83 => { // 'ControllableDynamicExtendedPacket'
                match connector.controllables.read()?.get(packet.path_ship() as usize) {
                    &None => return Err(Error::InvalidControllable(packet.path_ship())),
                    &Some(ref controllable) => {
                        controllable.update_extended(packet)?;
                    }
                }
            },
            0x84 => { // 'ControllableInfoStaticPacket'
                let player = connector.player_for(packet.path_player())?;
                player.set_controllable_info(
                    packet.path_ship(),
                    Some(Arc::new(ControllableInfo::from_packet(packet, Arc::downgrade(&player))?)),
                )?;
            },
            0x85 => { // 'ControllableInfoDynamicPacket'
                let player = connector.player_for(packet.path_player())?;
                match player.controllable_info(packet.path_ship()) {
                    None => return Err(Error::InvalidControllableInfo(packet.path_ship())),
                    Some(info) => info.update(packet)?,
                };
            },
            0x86 => { // 'ControllableInfoRemoved'
                let player = connector.player_for(packet.path_player())?;
                let path_ship = packet.path_ship();

                match player.controllable_info(path_ship) {
                    None => return Err(Error::InvalidControllableInfo(path_ship)),
                    Some(info) => info.set_active(false)?
                };
                player.set_controllable_info(path_ship, None)?;
            },
            0x87 => { // 'ControllableInfoScorePacket'
                let path_ship = packet.path_ship();
                let reader = &mut packet.read() as &mut BinaryReader;
                let player = connector.player_for(packet.path_player())?;
                match player.controllable_info(path_ship) {
                    None => return Err(Error::InvalidControllableInfo(path_ship)),
                    Some(ref info) => {
                        info.scores().update(reader)?
                    },
                }
            },
            0x88 => { // 'ControllableInfoCrystalPacket'
                let mut crystals = Vec::new();

                {
                    let reader = &mut packet.read() as &mut BinaryReader;
                    loop {
                        let cargo_item = match AnyCargoItem::from_reader(&connector, reader, true) {
                            Ok(item) => item,
                            Err(e) => {
                                if let Error::IoError(_, ref inner_e) = e {
                                    if let io::ErrorKind::UnexpectedEof = inner_e.kind() {
                                        // everything is fine, no more crystals
                                        break;
                                    }
                                }
                                return Err(e);
                            }
                        };

                        if let AnyCargoItem::Crystal(crystal) = cargo_item {
                            crystals.push(crystal);

                        } else {
                            return Err(Error::not_crystal_cargo_item());
                        }
                    }
                }


                let player = connector.player_for(packet.path_player())?;
                let info   = player.controllable_info(packet.path_ship()).ok_or(Error::InvalidControllableInfo(packet.path_ship()))?;
                info.set_crystals(crystals)?;
            },
            0x89 => { // 'ControllableCrystalPacket'
                let crystals     = Self::read_all_crystals(&connector, packet)?;
                let controllable = connector.controllable(packet.path_ship())?;
                controllable.set_crystals(crystals)?;
            },
            0x8A => { // 'ControllableInfoCargoPacket'
                let cargo_items = Connector::read_all_cargo_items(&connector, packet)?;
                let player = connector.player_for(packet.path_player())?;
                let info = player.controllable_info(packet.path_ship()).ok_or(Error::InvalidControllable(packet.path_ship()))?;
                info.set_cargo_items(cargo_items)?;
            },
            0x8B => { // 'ControllableInfoCargoPacket'
                let cargo_items = Connector::read_all_cargo_items(&connector, packet)?;
                let controllable = connector.controllable(packet.path_ship())?;
                controllable.set_cargo_items(cargo_items)?;
            },
            0x90 => { // scan result entry received
                let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
                let group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
                let unit = AnyUnit::from_reader(connector, &group, packet, &mut packet.read() as &mut BinaryReader)?;


                let controllable = connector.controllable(packet.path_universe())?;

                if let AnyUnit::PixelCluster(ref cluster) = unit {
                    let pixels = Connector::read_pixels_from_pixel_cluster(connector, &*cluster)?;
                    let mut scan = controllable.scan_list().write()?;
                    for pixel in pixels.into_iter() {
                        scan.push(AnyUnit::Pixel(pixel));
                    }

                } else {
                    controllable.scan_list().write()?.push(unit);
                }

            },
            // TODO missing entries
            _ => {
                println!("Received packet with unimplemented command: {:?}", packet);
            }
        };
        Ok(())
    }

    fn read_all_cargo_items(connector: &Arc<Connector>, packet: &Packet) -> Result<Vec<AnyCargoItem>, Error> {
        let mut items = Vec::new();

        loop {
            match AnyCargoItem::from_reader(&connector, &mut packet.read() as &mut BinaryReader, true) {
                Ok(item) => items.push(item),
                Err(e) => {
                    if let Error::IoError(_, ref inner_e) = e {
                        if let io::ErrorKind::UnexpectedEof = inner_e.kind() {
                            // everything is fine, no more crystals
                            break;
                        }
                    }
                    return Err(e);
                }
            };
        }

        Ok(items)
    }

    fn read_all_crystals(connector: &Arc<Connector>, packet: &Packet) -> Result<Vec<Arc<CrystalCargoItem>>, Error> {
        let mut crystals = Vec::new();

        loop {
            let cargo_item = match AnyCargoItem::from_reader(&connector, &mut packet.read() as &mut BinaryReader, true) {
                Ok(item) => item,
                Err(e) => {
                    if let Error::IoError(_, ref inner_e) = e {
                        if let io::ErrorKind::UnexpectedEof = inner_e.kind() {
                            // everything is fine, no more crystals
                            break;
                        }
                    }
                    return Err(e);
                }
            };

            if let AnyCargoItem::Crystal(crystal) = cargo_item {
                crystals.push(crystal);

            } else {
                return Err(Error::not_crystal_cargo_item());
            }
        }

        Ok(crystals)
    }

    fn read_pixels_from_pixel_cluster(connector: &Arc<Connector>, cluster: &PixelCluster) -> Result<Vec<Arc<Pixel>>, Error> {
        let mut pixels = Vec::new();
        let reader = &mut &cluster.data()[..] as &mut BinaryReader;

        let group= {
            let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
            player.universe_group().clone().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?
        };

        let radius  = cluster.radius() / 16_f32;
        let position = cluster.position();
        let mut y_pos  = position.y() - cluster.radius() + radius;
        let mut x_pos;

        for y in 0..16 {
            x_pos = position.x() - cluster.radius() + radius;

            for x in 0..16 {
                let pixel = Arc::new(Pixel::new(
                    connector,
                    &group,
                    format!("{}{:08X}{:08X}", cluster.name(), x, y),
                    radius,
                    Vector::new(x_pos, y_pos),
                    reader.read_unsigned_byte()?,
                    reader.read_unsigned_byte()?,
                    reader.read_unsigned_byte()?,
                ));

                if pixel.is_relevant() {
                    pixels.push(pixel);
                }

                x_pos += radius * 2_f32;
            }
            y_pos += radius * 2_f32;
        }
        Ok(pixels)
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

    pub fn players(&self) -> RwLockReadGuard<UniversalHolder<Player>> {
        self.players.read().unwrap()
    }

    pub fn player(&self) -> Weak<Player> {
        self.player.read().unwrap().clone()
    }

    pub fn player_for(&self, index: u16) -> Result<Arc<Player>, Error> {
        match self.players.read()?.get_for_index(index as usize) {
            None => Err(Error::missing_player(index)),
            Some(arc) => Ok(arc.clone())
        }
    }

    pub fn weak_player_for(&self, index: u16) -> Result<Weak<Player>, Error> {
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
                if !flow.ready()? {
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

    pub fn universe_groups(&self) -> &RwLock<ManagedArray<Arc<UniverseGroup>>> {
        &self.uni_groups
    }

    pub fn universe_group(&self, index: u16) -> Result<Arc<UniverseGroup>, Error> {
        let lock = self.uni_groups.read()?;
        lock.get(index as usize).clone().ok_or(Error::InvalidUniverseGroup(index))
    }

    pub fn universe_group_for_name(&self, name: &str) -> Result<Arc<UniverseGroup>, Error> {
        let lock = self.uni_groups.read()?;
        for index in 0..lock.len() {
            let group = lock.get(index);
            if let &Some(ref group) = group {
                if group.name().eq(name) {
                    return Ok(group.clone());
                }
            }
        }
        Err(Error::InvalidName)
    }

    pub fn crystals(&self, name: &str) -> Result<Arc<CrystalCargoItem>, Error> {
        let crystals = self.crystals.read()?;
        for i in 0..crystals.len() {
            match crystals.get(i) {
                &None => {},
                &Some(ref arc) => {
                    if arc.name().eq(name) {
                        return Ok(arc.clone());
                    }
                },
            };
        };
        Err(Error::InvalidCrystalName(String::from(name)))
    }

    pub fn controllable(&self, index: u8) -> Result<AnyControllable, Error> {
        self.controllables.read()?.get(index as usize).clone().ok_or(Error::InvalidControllable(index))
    }

    pub fn controllable_opt(&self, index: u8) -> Option<AnyControllable> {
        self.controllables.read().unwrap().get(index as usize).clone()
    }

    /// Queries up to 128 designs.
    ///
    /// You can edit these [ControllableDesign]s via the flattiverse.com homepage
    ///
    /// Beware: The flattiverse-server will queue your and the requsts of other players
    /// to avoid high loads on the database. So this method might need some time in until
    /// it returns. Parallel requests to this method will be declined by the flattiverse-server
    /// resultin in an empty return [Vec]. You have been warned.
    ///
    pub fn query_designes(&self) -> Result<Vec<ControllableDesign>, Error> {
        let _ = self.sync_account_queries().lock()?;

        let mut vec     = Vec::new();
        let mut packet  = Packet::new();

        let block    = self.block_manager().block()?;
        let response = {
            let mut block = block.lock()?;
            packet.set_command(0x10);
            packet.set_session(block.id());

            self.send(&packet)?;
            block.wait()?
        };

        {
            let reader = &mut response.read() as &mut BinaryReader;
            loop {
                // TODO WTF
                vec.push(match ControllableDesign::from_reader(reader) {
                    Ok(item) => item,
                    Err(e) => {
                        let ok = match e {
                            Error::IoError(_, ref inner_e) => {
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
                });
            }
        }

        Ok(vec)
    }

    /// The [Receiver] for messages
    pub fn messages(&self) -> &Arc<Mutex<Receiver<AnyMessage>>> {
        &self.receiver
    }

    pub fn next_message(&self) -> Option<AnyMessage> {
        let lock = match self.receiver.lock() {
            Ok(lock) => lock,
            Err(err) => {
                println!("Failed to acquire lock for self.receiver: {:?}", err);
                return None;
            }
        };
        match lock.try_recv() {
            Ok(message) => Some(message),
            Err(_) => {
                // either TryRecvError::Empty or TryRecvError::Disconnected
                None
            }
        }
    }

    pub fn has_flows(&self) -> Result<bool, Error> {
        Ok(!self.flows.read()?.is_empty())
    }

    pub fn close(&self) -> Result<(), Error> {
        self.connection.lock()?.close()
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

    pub fn benchmark() -> Result<PerformanceMark, Error> {
        let single_threaded_mark;
        let mut multi_threadedmark = 0;
        let memory_access_mark;

        let mre = Arc::new(ManualResetEvent::new(false));

        println!("one short test to warm up");
        // one short test to warm up
        {
            let test = PerformanceTest::new(mre.clone(), TimeSpan::from_dhmsm(0, 0, 0, 0, 100), true);
            test.ready().wait_one()?;
            mre.set()?;
            thread::sleep(Duration::from_millis(100));
            mre.reset()?;
            test.ready().wait_one()?;
            mre.set()?;
            thread::sleep(Duration::from_millis(100));
            mre.reset()?;
            test.ready().wait_one()?;
        }

        println!("single CPU test");
        // single CPU test
        {
            let test = PerformanceTest::new(mre.clone(), TimeSpan::from_dhmsm(0, 0, 0, 1, 600), true);
            test.ready().wait_one()?;
            mre.set()?;
            thread::sleep(Duration::from_millis(1_500));
            mre.reset()?;
            test.ready().wait_one()?;

            single_threaded_mark = test.try_result()?;

            mre.set()?;
            thread::sleep(Duration::from_millis(1_500));
            mre.reset()?;
            test.ready().wait_one()?;

            memory_access_mark = test.try_result()?;
        }

        println!("multi CPU test");
        // multi CPU test
        let mut tests = Vec::with_capacity(16);
        for _ in 0..16 {
            tests.push(PerformanceTest::new(mre.clone(), TimeSpan::from_dhmsm(0, 0, 0, 3, 600), false))
        }

        for test in tests.iter() {
            test.ready().wait_one()?;
        }

        mre.set()?;
        thread::sleep(Duration::from_millis(3_500));
        mre.reset()?;

        for test in tests.iter() {
            test.ready().wait_one()?;
            multi_threadedmark += test.try_result()?;
        }

        Ok(PerformanceMark::from_save(single_threaded_mark, multi_threadedmark, memory_access_mark, Self::hostname())?)
    }

    pub fn hostname() -> String {
        hostname::get_hostname().expect("Failed to retrieve hostname")
    }

    fn sha512(text: &str) -> Vec<u8> {
        let mut hasher = Sha512::default();
        hasher.input(text.as_bytes());
        Vec::from(&hasher.result()[..])
    }
}

// TODo WHY IN EXTRA TRAIT
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