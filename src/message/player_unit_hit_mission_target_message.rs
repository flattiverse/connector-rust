
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
use Error;
use Player;
use Connector;
use UniversalEnumerable;

use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerUnitHitMissionTargetMessage);
pub trait PlayerUnitHitMissionTargetMessage : GameMessage {

    fn player_unit_player(&self) -> &Arc<RwLock<Player>>;

    fn player_unit(&self) -> &Arc<RwLock<ControllableInfo>>;

    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>>;

    fn mission_target_sequence(&self) -> u16;
}

pub struct PlayerUnitHitMissionTargetMessageData {
    data:   GameMessageData,
    player: Arc<RwLock<Player>>,
    info:   Arc<RwLock<ControllableInfo>>,
    name:   String,
    team:   Option<Arc<RwLock<Team>>>,
    seq:    u16,
}

impl PlayerUnitHitMissionTargetMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitHitMissionTargetMessageData, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitHitMissionTargetMessageData {
            data,
            player: player.clone(),
            info:   {
                let index = reader.read_unsigned_byte()?;
                let player = player.read()?;
                player.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?
            },
            name:   reader.read_string()?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                if id != 0xFF {
                    let player = connector.player().upgrade();
                    let player = player.ok_or(Error::PlayerNotAvailable)?;
                    let player = player.read()?;
                    let group  = player.universe_group().upgrade();
                    let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
                    let group  = group.read()?;
                    Some(group.team(id)?)
                } else {
                    None
                }
            },
            seq:    reader.read_u16()?,
        })
    }
}

impl Borrow<GameMessageData> for PlayerUnitHitMissionTargetMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitHitMissionTargetMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitHitMissionTargetMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitHitMissionTargetMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitHitMissionTargetMessageData> + BorrowMut<PlayerUnitHitMissionTargetMessageData> + GameMessage> PlayerUnitHitMissionTargetMessage for T {
    fn player_unit_player(&self) -> &Arc<RwLock<Player>> {
        &self.borrow().player
    }

    fn player_unit(&self) -> &Arc<RwLock<ControllableInfo>> {
        &self.borrow().info
    }

    fn mission_target_name(&self) -> &str {
        &self.borrow().name
    }

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>> {
        &self.borrow().team
    }

    fn mission_target_sequence(&self) -> u16 {
        self.borrow().seq
    }
}

impl fmt::Display for PlayerUnitHitMissionTargetMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' ",

            (self as &FlattiverseMessage).timestamp(),
            match (self as &PlayerUnitHitMissionTargetMessage).player_unit().read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match (self as &PlayerUnitHitMissionTargetMessage).player_unit().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            match (self as &PlayerUnitHitMissionTargetMessage).player_unit_player().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
        )?;

        if let Some(ref team) = self.team {
            if let Ok(ref team) = team.read() {
                write!(f, "teams {} ", team.name())?;
            }
        }

        write!(f, "MissionTarget \"{}\"", self.name)?;
        if self.seq > 0 {
            write!(f, " with sequence number {}", self.seq)?;
        }
        write!(f, ".")
    }
}