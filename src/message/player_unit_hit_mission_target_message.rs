
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

    fn player_unit_player(&self) -> &Arc<Player>;

    fn player_unit(&self) -> &Arc<ControllableInfo>;

    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<Team>>;

    fn mission_target_sequence(&self) -> u16;
}

pub struct PlayerUnitHitMissionTargetMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
    name:   String,
    team:   Option<Arc<Team>>,
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
                player.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?
            },
            name:   reader.read_string()?,
            team:   {
                let id = reader.read_unsigned_byte()?;
                if id != 0xFF {
                    let player = connector.player().upgrade();
                    let player = player.ok_or(Error::PlayerNotAvailable)?;
                    let group  = player.universe_group().upgrade();
                    let group  = group.ok_or(Error::PlayerNotInUniverseGroup)?;
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
    fn player_unit_player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.borrow().info
    }

    fn mission_target_name(&self) -> &str {
        &self.borrow().name
    }

    fn mission_target_team(&self) -> &Option<Arc<Team>> {
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
            (self as &PlayerUnitHitMissionTargetMessage).player_unit().kind(),
            (self as &PlayerUnitHitMissionTargetMessage).player_unit().name(),
            (self as &PlayerUnitHitMissionTargetMessage).player_unit_player().name(),
        )?;

        if let Some(ref team) = self.team {
            write!(f, "teams {} ", team.name())?;
        }

        write!(f, "MissionTarget \"{}\"", self.name)?;
        if self.seq > 0 {
            write!(f, " with sequence number {}", self.seq)?;
        }
        write!(f, ".")
    }
}