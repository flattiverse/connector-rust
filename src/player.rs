
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Team;
use Task;
use Error;
use Scores;
use Version;
use IndexList;
use Connector;
use PlatformKind;
use PerformanceMark;
use UniversalHolder;
use UniversalEnumerable;
use UniverseGroup;
use dotnet::TimeSpan;
use unit::ControllableInfo;
use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;
use net::is_set_u8;

pub struct Player {
    name:        String,
    platform:    PlatformKind,
    version:     Version,
    performance: Option<PerformanceMark>,

    id:     u16,
    rank:   u32,
    level:   u8,
    elo:    i32,

    game_scores:         Scores,
    player_scores:       Scores,
    clan:                Option<String>,
    average_commit_time: TimeSpan,
    last_commit_time:    TimeSpan,
    ping:                TimeSpan,

    connector:      Weak<Connector>,
    universe_group: Weak<RwLock<UniverseGroup>>,
    team:           Weak<RwLock<Team>>,

    active: bool,
    online: bool,

    controllables: RwLock<UniversalHolder<ControllableInfo>>
}

impl PartialEq<Player> for Player {
    fn eq(&self, other: &Player) -> bool {
        self.id == other.id
    }
}

impl Player {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Player, Error> {
        Ok(Player {
            connector:  Arc::downgrade(connector),
            id:         packet.path_player(),
            name:       reader.read_string()?,
            platform:   PlatformKind::from_id(reader.read_byte()?),
            version:    Version::from_raw(reader.read_u32()?),
            performance:{
                let mark = PerformanceMark::from_reader(reader)?;
                if mark.memory_access_mark() <= 0.01_f64 && mark.multi_threaded_mark() <= 0.01_f64 && mark.single_threaded_mark() <= 0.01_f64 {
                    None
                } else {
                    Some(mark)
                }
            },
            controllables: RwLock::new(UniversalHolder::new(IndexList::new(true, 256))),
            game_scores:   Scores::default(),
            player_scores: Scores::default(),

            // doesn't exist in the C# connector
            ping:                   TimeSpan::from_seconds(1),
            average_commit_time:    TimeSpan::from_seconds(1),
            last_commit_time:       TimeSpan::from_seconds(1),

            // defaults
            clan:           None,
            rank:           0u32,
            level:          0u8,
            universe_group: Weak::new(),
            team:           Weak::new(),
            active:         false,
            online:         false,
            elo:            0i32,
        })
    }

    pub fn big_avatar_raw(&self) -> Result<Vec<u8>, Error> {
        self.avatar_raw(false)
    }

    pub fn small_avatar_raw(&self) -> Result<Vec<u8>, Error> {
        self.avatar_raw(true)
    }

    pub fn avatar_raw(&self, small: bool) -> Result<Vec<u8>, Error> {
        match self.connector.upgrade() {
            None            => Err(Error::ConnectorNotAvailable),
            Some(connector) => Ok({
                let block = connector.block_manager().block()?;
                let mut packet = Packet::new();

                {
                    let block = block.lock()?;
                    packet.set_command(if small {0x02_u8} else {0x03_u8});
                    packet.set_session(block.id());
                    packet.set_path_player(self.id);
                }

                connector.send(&packet)?;
                let response = block.lock()?.wait()?;

                match connector.player().upgrade() {
                    None => {},
                    Some(ref arc) => {
                        if arc.read()?.id() != self.id() {
                            connector.register_task_quitely_if_unknown(Task::UsedAvatar);
                        }
                    }
                };


                Vec::from(response.read())
            })
        }
    }

    pub fn clear_assignment(&mut self) {
        self.clan = None
    }

    pub fn update_assignment(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        self.clan = if is_set_u8(reader.read_unsigned_byte()?, 0x01) {
            Some(reader.read_string()?)
        } else {
            None
        };
        Ok(())
    }

    pub fn update_stats(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        self.rank   = reader.read_u32()?;
        self.level  = reader.read_unsigned_byte()?;
        self.elo    = reader.read_u16()? as i32;

        self.game_scores    .update(reader)?;
        self.player_scores  .update(reader)?;
        Ok(())
    }

    pub fn update_ping(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        self.ping.update(reader)?;
        Ok(())
    }

    pub fn update_timing(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        self.average_commit_time.update(reader)?;
        self.last_commit_time   .update(reader)?;
        Ok(())
    }

    /// Sens a chat message to this [Player].
    /// The message needs to be none empty and not
    /// longer than 140 characters.
    pub fn chat(&self, message: &str) -> Result<(), Error> {
        if message.is_empty() || message.len() > 140 {
            return Err(Error::InvalidMessage);
        }

        if !self.active {
            return Err(Error::CantSendMessageToInactivePlayer);
        }

        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let block = connector.block_manager().block()?;
                let mut packet = Packet::new();

                {
                    let block = block.lock()?;
                    packet.set_command(0x30_u8);
                    packet.set_path_player(self.id);
                    packet.set_session(block.id());
                }

                {
                    let writer = packet.write() as &mut BinaryWriter;
                    writer.write_string(message)?;
                }

                connector.send(&packet)?;
                block.lock()?.wait()?;
                Ok(())
            }
        }
    }

    // Sens a binary message (up to 255 bytes) to this [Player]
    pub fn chat_binary(&self, data: &[u8]) -> Result<(), Error> {
        if data.is_empty() || data.len() > 255 {
            return Err(Error::InvalidMessage);
        }

        if !self.active {
            return Err(Error::CantSendMessageToInactivePlayer);
        }

        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let block = connector.block_manager().block()?;
                let mut packet = Packet::new();

                {
                    let block = block.lock()?;
                    packet.set_command(0x33_u8);
                    packet.set_path_player(self.id);
                    packet.set_session(block.id());
                }

                {
                    let writer = packet.write() as &mut BinaryWriter;
                    writer.write_u8(data.len() as u8)?;
                    writer.write(data)?;
                }

                connector.send(&packet)?;
                block.lock()?.wait()?;
                Ok(())
            }
        }
    }

    /// Sends up to 32 binary chat messages to this [Player]. This method
    /// guarantees to keep the order of the messages and sends them parallel
    /// without opening additional threads. This method opens as many parallel
    /// requests as given messages to send.
    /// The flattiverse-protocol limits parallel requests (currently at 255).
    /// Trying to use more requests will result in an [Error]
    pub fn chat_binaries(&self, data: &[&[u8]]) -> Result<(), Error> {
        if data.is_empty() || data.len() > 32 {
            return Err(Error::InvalidMessageList);
        }

        for i in 0..data.len() {
            if data[i].is_empty() || data[i].len() > 255 {
                return Err(Error::InvalidMessageAtIndex(i as u8));
            }
        }

        if !self.active {
            return Err(Error::CantSendMessageToInactivePlayer);
        }

        match self.connector.upgrade() {
            None => Err(Error::ConnectorNotAvailable),
            Some(connector) => {
                let mut blocks = Vec::with_capacity(data.len());
                let mut packets = Vec::with_capacity(data.len());

                for i in 0..data.len() {
                    let block = connector.block_manager().block()?;
                    let mut packet = Packet::new();

                    {
                        let block = block.lock()?;
                        packet.set_command(0x33_u8);
                        packet.set_path_player(self.id);
                        packet.set_session(block.id());
                    }

                    {
                        let writer = packet.write() as &mut BinaryWriter;
                        writer.write_u8(data[i].len() as u8)?;
                        writer.write(data[i])?;
                    }


                    blocks.push(block);
                    packets.push(packet);
                }

                connector.send_many(&packets)?;
                for i in 0..blocks.len() {
                    blocks[i].lock()?.wait()?;
                }
                Ok(())
            }
        }
    }

    pub fn platform(&self) -> PlatformKind {
        match self.connector.upgrade() {
            None => {},
            Some(connector) => {
                match connector.player().upgrade() {
                    None => {},
                    Some(ref player) => {
                        match player.read() {
                            Err(_) => {},
                            Ok(ref player) => {
                                if self.id != player.id() {
                                    connector.register_task_quitely_if_unknown(Task::UsedPlatform);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.platform
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn performance(&self) -> &Option<PerformanceMark> {
        &self.performance
    }

    pub fn rank(&self) -> u32 {
        self.rank
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn elo(&self) -> i32 {
        self.elo
    }

    pub fn ping(&self) -> &TimeSpan {
        &self.ping
    }

    pub fn game_scores(&self) -> &Scores {
        &self.game_scores
    }

    pub fn player_scores(&self) -> &Scores {
        &self.player_scores
    }

    pub fn clan(&self) -> &Option<String> {
        &self.clan
    }

    pub fn average_commit_time(&self) -> &TimeSpan {
        match self.connector.upgrade() {
            None => {},
            Some(connector) => {
                match connector.player().upgrade() {
                    None => {},
                    Some(ref player) => {
                        match player.read() {
                            Err(_) => {},
                            Ok(ref player) => {
                                if self.id != player.id() {
                                    connector.register_task_quitely_if_unknown(Task::UsedAvgCommitTime);
                                }
                            }
                        }
                    }
                }
            }
        }
        &self.average_commit_time
    }

    pub fn last_commit_time(&self) -> &TimeSpan {
        &self.last_commit_time
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn team(&self) -> &Weak<RwLock<Team>> {
        &self.team
    }

    pub(crate) fn set_team(&mut self, team: Weak<RwLock<Team>>) -> &mut Self {
        self.team = team;
        self
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub(crate) fn set_active(&mut self, active: bool) -> &mut Self {
        self.active = active;
        self
    }

    pub fn online(&self) -> bool {
        self.online
    }

    pub(crate) fn set_online(&mut self, online: bool) -> &mut Self {
        self.online = online;
        self
    }

    pub fn controllable_info_list(&self) -> &RwLock<UniversalHolder<ControllableInfo>> {
        &self.controllables
    }

    pub fn controllable_info(&self, index: u8) -> Option<Arc<RwLock<ControllableInfo>>> {
        self.controllables.read().unwrap().get_for_index(index as usize)
    }

    pub(crate) fn set_controllable_info(&mut self, index: u8, value: Option<Arc<RwLock<ControllableInfo>>>) -> &mut Self {
        self.controllables.write().unwrap().set(index as usize, value);
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn universe_group(&self) -> &Weak<RwLock<UniverseGroup>> {
        &self.universe_group
    }

    pub(crate) fn set_universe_group(&mut self, group: Weak<RwLock<UniverseGroup>>) {
        self.universe_group = group;
    }
}

impl UniversalEnumerable for Player {
    fn name(&self) -> &str {
        &self.name
    }
}

use std::fmt;
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) {:?} {}", self.name, self.id, self.platform, self.version)
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}