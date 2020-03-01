use crate::command;
use crate::connector::Connector;
use crate::io::{BinaryReader, BinaryWriter};
use crate::packet::Packet;
use crate::players::{Account, Team};
use crate::requesting::Request;
use crate::requests::{AmbiguousXmlData, IllegalName, RequestError};
use byteorder::ReadBytesExt;
use bytes::Bytes;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::ops::RangeInclusive;

const DEFAULT_TEAMS: usize = 16;
const DEFAULT_GALAXIES: usize = 32;

#[derive(Debug, Clone)]
pub struct Universe {
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) difficulty: Difficulty,
    pub(crate) mode: UniverseMode,
    pub(crate) owner_id: u32,
    pub(crate) max_players: u16,
    pub(crate) max_players_per_team: u16,
    pub(crate) max_ships_per_player: u8,
    pub(crate) max_ships_per_team: u16,
    pub(crate) status: Status,
    pub(crate) default_privileges: Privileges,
    pub(crate) avatar: Vec<u8>,
    pub(crate) teams: Vec<Option<Team>>,
    pub(crate) galaxies: Vec<Option<Galaxy>>,
    pub(crate) systems: Vec<System>,
}

impl Universe {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn mode(&self) -> UniverseMode {
        self.mode
    }

    pub fn owner_iod(&self) -> u32 {
        self.owner_id
    }

    pub fn max_players(&self) -> u16 {
        self.max_players
    }

    pub fn max_ships_per_player(&self) -> u8 {
        self.max_ships_per_player
    }

    pub fn max_ships_per_team(&self) -> u16 {
        self.max_ships_per_team
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn default_privileges(&self) -> Privileges {
        self.default_privileges
    }

    pub fn avatar(&self) -> &[u8] {
        &self.avatar
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter().filter_map(Option::as_ref)
    }

    pub fn team(&self, id: u8) -> Option<&Team> {
        self.teams.get(usize::from(id)).and_then(Option::as_ref)
    }

    pub fn galaxies(&self) -> impl Iterator<Item = &Galaxy> {
        self.galaxies.iter().filter_map(Option::as_ref)
    }

    pub fn galaxy(&self, id: u8) -> Option<&Galaxy> {
        self.galaxies.get(usize::from(id)).and_then(Option::as_ref)
    }

    pub fn systems(&self) -> impl Iterator<Item = &System> {
        self.systems.iter()
    }

    #[must_use]
    pub fn join(&self) -> Packet {
        debug!(
            "Issuing join request for universe[{}] '{}' and auto-select team",
            self.id, self.name,
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_JOIN;
        packet.sub_address = 0x00; // auto selection
        packet
    }

    #[must_use]
    pub fn join_with_team(&self, team_id: u8) -> Packet {
        debug!(
            "Issuing join request for universe[{}] '{}' on team[{}] '{}'",
            self.id,
            self.name,
            team_id,
            self.teams
                .get(usize::from(team_id))
                .and_then(Option::<Team>::as_ref)
                .map(Team::name)
                .unwrap_or("")
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_JOIN;
        packet.base_address = self.id;
        packet.sub_address = team_id;
        packet
    }

    #[must_use]
    pub fn part(&self) -> Packet {
        debug!(
            "Issuing part request for universe[{}] '{}'",
            self.id, self.name
        );
        let mut packet = Packet::default();
        packet.command = command::id::C2S_UNIVERSE_PART;
        packet.base_address = self.id;
        packet
    }

    #[must_use]
    pub fn query_privileges(&self) -> Request<AccountPrivileges> {
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_QUERY_PRIVILEGES;
        packet.base_address = self.id;
        packet.into()
    }

    /// Changes the privileges of the given account for this universe. Use
    /// [`Universe::default_privileges`] to remove an entry for an account.
    ///
    /// [`Universe::default_privileges`]: crate::entity::Universe::default_privileges
    #[must_use]
    pub fn alter_privileges(&self, account: &Account, privileges: Privileges) -> Request<()> {
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_UPDATE_PRIVILEGES;
        packet.base_address = self.id;
        packet.id = account.id();
        packet.helper = privileges.into();
        packet.into()
    }

    /// Removes any privileges of the given account for this universe.
    #[must_use]
    pub fn reset_privileges(&self, account: &Account) -> Request<()> {
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_UPDATE_PRIVILEGES;
        packet.base_address = self.id;
        packet.id = account.id();
        packet.helper = self.default_privileges.into();
        packet.into()
    }
}

impl TryFrom<&Packet> for Universe {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Universe {
            id: packet.base_address,
            name: reader.read_string()?,
            description: reader.read_string()?,
            difficulty: Difficulty::from_u8(reader.read_byte()?)
                .ok_or_else(|| IoError::from(IoErrorKind::InvalidInput))?,
            mode: UniverseMode::from_u8(reader.read_byte()?)
                .ok_or_else(|| IoError::from(IoErrorKind::InvalidInput))?,
            owner_id: reader.read_u32()?,
            max_players: reader.read_uint16()?,
            max_players_per_team: reader.read_uint16()?,
            max_ships_per_player: reader.read_byte()?,
            max_ships_per_team: reader.read_uint16()?,
            status: Status::from_u8(reader.read_byte()?)
                .ok_or_else(|| IoError::from(IoErrorKind::InvalidInput))?,
            default_privileges: Privileges::from(reader.read_byte()?),
            avatar: Vec::default(),
            teams: vec_of_none!(DEFAULT_TEAMS),
            galaxies: vec_of_none!(DEFAULT_GALAXIES),
            systems: Vec::default(),
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Insane = 3,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum UniverseMode {
    Mission = 0,
    ShootTheFlag = 1,
    Domination = 2,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum Status {
    Online = 0,
    Offline = 1,
    Maintenance = 2,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum Privilege {
    Nothing = 0,
    Join = 1,
    ManageUnits = 2,
    ManageRegions = 4,
    ManageSystems = 8,
    ManageUniverse = 16,
}

#[derive(Copy, Clone)]
pub struct Privileges(u8);

impl fmt::Debug for Privileges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let list = self.list().collect::<Vec<_>>();
        f.debug_tuple("Privileges").field(&list).finish()
    }
}

impl From<u8> for Privileges {
    fn from(value: u8) -> Self {
        Privileges(value)
    }
}

impl From<&[Privilege]> for Privileges {
    fn from(slice: &[Privilege]) -> Self {
        Privileges(slice.iter().map(|p| *p as u8).fold(0u8, |a, b| a ^ b))
    }
}

impl Into<u8> for Privileges {
    fn into(self) -> u8 {
        self.0
    }
}

impl Privileges {
    pub const fn has(&self, privilege: Privilege) -> bool {
        self.0 & (privilege as u8) != 0
    }

    pub fn list(&self) -> impl Iterator<Item = Privilege> {
        [
            Privilege::Nothing,
            Privilege::Join,
            Privilege::ManageUnits,
            Privilege::ManageRegions,
            Privilege::ManageSystems,
            Privilege::ManageUniverse,
        ]
        .iter()
        .filter_map(|p| if self.has(*p) { Some(*p) } else { None })
        .collect::<Vec<_>>()
        .into_iter()
    }
}

pub struct AccountPrivileges(Vec<(u32, Privileges)>);

impl AccountPrivileges {
    pub fn privileges(&self) -> impl Iterator<Item = &(u32, Privileges)> {
        self.0.iter()
    }

    pub fn into_stream(mut self, connector: &mut Connector) -> AccountPrivilegesStream {
        self.0.reverse();
        AccountPrivilegesStream(connector, self.0)
    }
}

impl TryFrom<&Packet> for AccountPrivileges {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let len = packet.payload().len() / (std::mem::size_of::<u8>() + std::mem::size_of::<u32>());
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push((reader.read_u32()?, Privileges::from(reader.read_u8()?)));
        }
        Ok(AccountPrivileges(vec))
    }
}

impl Into<Vec<(u32, Privileges)>> for AccountPrivileges {
    fn into(self) -> Vec<(u32, Privileges)> {
        self.0
    }
}

pub struct AccountPrivilegesStream<'a>(&'a mut Connector, Vec<(u32, Privileges)>);

impl<'a> AccountPrivilegesStream<'a> {
    pub async fn next(&mut self) -> Option<Result<(Option<Account>, Privileges), RequestError>> {
        if self.1.is_empty() {
            None
        } else {
            Some(self.retrieve_next().await)
        }
    }

    async fn retrieve_next(&mut self) -> Result<(Option<Account>, Privileges), RequestError> {
        let index = self.1.len() - 1;
        let (id, privilege) = self.1[index];
        let account = Account::query_by_id(id).send(self.0).await?.await?;
        self.1.remove(index);
        Ok((account, privilege))
    }
}

#[derive(Debug, Clone)]
pub struct Galaxy {
    id: u8,
    universe: u16,
    name: String,
    spawn: bool,
}

impl Galaxy {
    pub const fn id(&self) -> u8 {
        self.id
    }

    pub const fn universe(&self) -> u16 {
        self.universe
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether you can spawn into this galaxy
    pub fn spawn(&self) -> bool {
        self.spawn
    }

    /// Queries a a unit for this galaxy in xml representation. To succeed,
    /// the issues requires to have [`ManageUnits`] privileges.
    ///
    /// [`ManageUnits`]: crate::entity::Privileges::allowed_to_manage_units
    pub fn query_unit_xml_by_name(&self, name: &str) -> Result<Request<String>, IllegalName> {
        if !Unit::check_name(name) {
            return Err(IllegalName);
        }
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_QUERY_UNIT;
        packet.base_address = self.universe;
        packet.sub_address = self.id;
        packet.payload = Some(Bytes::from({
            let mut payload = Vec::default();
            let writer = &mut payload as &mut dyn BinaryWriter;
            writer.write_string(name).expect("Failed to encode name");
            payload
        }));
        Ok(packet.into())
    }

    /// Updates or crates an unit according to the specified xml data. To succeed,
    /// the issues requires to have [`ManageUnits`] privileges.
    ///
    /// [`ManageUnits`]: crate::entity::Privileges::allowed_to_manage_units
    pub fn update_unit_xml(&self, xml: &str) -> Result<Request<()>, AmbiguousXmlData> {
        if xml.len() < 5 || xml.len() > 8192 {
            return Err(AmbiguousXmlData);
        }
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_UPDATE_OR_CREATE_UNIT;
        packet.base_address = self.universe;
        packet.sub_address = self.id;
        packet.payload = Some(Bytes::from({
            let mut payload = Vec::default();
            let writer = &mut payload as &mut dyn BinaryWriter;
            writer.write_string(xml).expect("Failed to encode xml data");
            payload
        }));
        Ok(packet.into())
    }

    /// Deletes the unit with the given name. To succeed,
    /// the issues requires to have [`ManageUnits`] privileges.
    ///
    /// [`ManageUnits`]: crate::entity::Privileges::allowed_to_manage_units
    pub fn delete_unit_by_name(&self, name: &str) -> Result<Request<()>, IllegalName> {
        if !Unit::check_name(name) {
            return Err(IllegalName);
        }
        let mut packet = Packet::default();
        packet.command = crate::command::id::C2S_DELETE_UNIT;
        packet.base_address = self.universe;
        packet.sub_address = self.id;
        packet.payload = Some(Bytes::from({
            let mut payload = Vec::default();
            let writer = &mut payload as &mut dyn BinaryWriter;
            writer.write_string(name).expect("Failed to encode name");
            payload
        }));
        Ok(packet.into())
    }
}

impl TryFrom<&Packet> for Galaxy {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        Ok(Galaxy {
            id: packet.sub_address,
            universe: packet.base_address,
            name: reader.read_string()?,
            spawn: reader.read_bool()?,
        })
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum SystemKind {
    /// The ships hull, a higher level indicates more hull points TODO ...
    Hull = 0x00,

    /// The ships armor, a higher level indicates more effective hit-points for TODO ...
    Armor = 0x01,

    /// The ships primary and short range scanner
    Scanner0 = 0x08,

    /// The ships secondary and long range scanner
    Scanner1 = 0x09,

    /// A higher leveled engine allows greater acceleration
    Engine = 0x10,

    /// A higher leveled thruster allows a greater turn-rate acceleration
    Thruster = 0x11,

    /// Meaning things like a Solar**Cell**
    /// A Higher level (Solar)Cell allows faster harvesting (of energy from suns)
    Cell = 0x18,

    /// The storage capacity of your ship
    Battery = 0xC0,
}

#[derive(Debug, Clone)]
pub struct System {
    kind: SystemKind,
    levels: RangeInclusive<u8>,
}

impl System {
    pub fn kind(&self) -> SystemKind {
        self.kind
    }

    pub fn levels(&self) -> RangeInclusive<u8> {
        self.levels.clone()
    }

    // The inclusive start level of this system
    pub fn level_start(&self) -> u8 {
        *self.levels.start()
    }

    /// The inclusive end level for this system
    pub fn level_end(&self) -> u8 {
        *self.levels.end()
    }

    pub(crate) fn vec_from(packet: &Packet) -> Result<Vec<System>, IoError> {
        let count = packet.payload().len() / 3; // TODO
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(System {
                kind: SystemKind::from_u8(reader.read_byte()?)
                    .ok_or_else(|| IoError::from(IoErrorKind::InvalidInput))?,
                levels: RangeInclusive::new(reader.read_byte()?, reader.read_byte()?),
            });
        }
        Ok(vec)
    }
}

pub struct Unit;

impl Unit {
    /// See [`Account::check_name`]
    ///
    /// [`Account::check_name`]: crate::players::Account::check_name
    pub fn check_name(name: &str) -> bool {
        Account::check_name(name)
    }
}
