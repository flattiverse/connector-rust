
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

downcast!(PlayerUnitHitOwnTargetMessage);
pub trait PlayerUnitHitOwnTargetMessage : GameMessage {

    fn player_unit_player(&self) -> &Arc<RwLock<Player>>;

    fn player_unit(&self) -> &Arc<RwLock<ControllableInfo>>;

    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<RwLock<Team>>>;
}

pub struct PlayerUnitHitOwnTargetMessageData {
    data:   GameMessageData,
    player: Arc<RwLock<Player>>,
    info:   Arc<RwLock<ControllableInfo>>,
    name:   String,
    team:   Option<Arc<RwLock<Team>>>,
}

impl PlayerUnitHitOwnTargetMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitHitOwnTargetMessageData, Error> {
        Ok(PlayerUnitHitOwnTargetMessageData {
            data:   GameMessageData::from_packet(connector, packet, reader)?,
            player: connector.player_for(reader.read_u16()?)?,
            info:   {
                let player = connector.player_for(reader.read_u16()?)?;
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
                    Some(group.team(id).clone().ok_or(Error::TeamNotAvailable)?)
                } else {
                    None
                }
            },
        })
    }
}

impl Borrow<GameMessageData> for PlayerUnitHitOwnTargetMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitHitOwnTargetMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitHitOwnTargetMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitHitOwnTargetMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitHitOwnTargetMessageData> + BorrowMut<PlayerUnitHitOwnTargetMessageData> + GameMessage> PlayerUnitHitOwnTargetMessage for T {
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
}

impl fmt::Display for PlayerUnitHitOwnTargetMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' ",

            (self as &FlattiverseMessage).timestamp(),
            match (self as &PlayerUnitHitOwnTargetMessage).player_unit().read() {
                Err(_) => String::new(),
                Ok(ref read) => {
                    let mut string = String::new();
                    write!(string, "{:?}", read.kind())?;
                    string
                },
            },
            match (self as &PlayerUnitHitOwnTargetMessage).player_unit().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
            match (self as &PlayerUnitHitOwnTargetMessage).player_unit_player().read() {
                Err(_) => "",
                Ok(ref read) => read.name()
            },
        )?;

        if let Some(ref team) = self.team {
            if let Ok(ref team) = team.read() {
                write!(f, "teams {} ", team.name())?;
            }
        }

        write!(f, "successfully hit his own MissionTarget \"{}\"", self.name)
    }
}