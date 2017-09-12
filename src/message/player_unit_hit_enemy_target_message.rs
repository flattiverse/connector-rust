
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

downcast!(PlayerUnitHitEnemyTargetMessage);
pub trait PlayerUnitHitEnemyTargetMessage : GameMessage {

    fn player_unit_player(&self) -> &Arc<Player>;

    fn player_unit(&self) -> &Arc<ControllableInfo>;

    fn mission_target_name(&self) -> &str;

    fn mission_target_team(&self) -> &Option<Arc<Team>>;
}

pub struct PlayerUnitHitEnemyTargetMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
    name:   String,
    team:   Option<Arc<Team>>,
}

impl PlayerUnitHitEnemyTargetMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitHitEnemyTargetMessageData, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitHitEnemyTargetMessageData {
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
        })
    }
}

impl Borrow<GameMessageData> for PlayerUnitHitEnemyTargetMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerUnitHitEnemyTargetMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerUnitHitEnemyTargetMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerUnitHitEnemyTargetMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerUnitHitEnemyTargetMessageData> + BorrowMut<PlayerUnitHitEnemyTargetMessageData> + GameMessage> PlayerUnitHitEnemyTargetMessage for T {
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
}

impl fmt::Display for PlayerUnitHitEnemyTargetMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' ",
            (self as &FlattiverseMessage).timestamp(),
            (self as &PlayerUnitHitEnemyTargetMessage).player_unit().kind(),
            (self as &PlayerUnitHitEnemyTargetMessage).player_unit().name(),
            (self as &PlayerUnitHitEnemyTargetMessage).player_unit_player().name(),
        )?;

        write!(f, "successfully hit ")?;
        if let Some(ref team) = self.team {
            write!(f, "teams \"{}\" ", team.name())?;
        }

        write!(f, "MissionTarget \"{}\".", self.name)
    }
}