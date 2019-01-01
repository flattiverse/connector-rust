
use std::fmt;
use std::sync::Arc;

use crate::Team;
use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::ControllableInfo;

use crate::message::any_game_message::prelude::*;

pub struct PlayerUnitHitOwnTargetMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
    name:   String,
    team:   Option<Arc<Team>>,
}

impl PlayerUnitHitOwnTargetMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitHitOwnTargetMessage, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitHitOwnTargetMessage {
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

    pub fn player_unit_player(&self) -> &Arc<Player> {
        &self.player
    }

    pub fn player_unit(&self) -> &Arc<ControllableInfo> {
        &self.info
    }

    pub fn mission_target_name(&self) -> &str {
        &self.name
    }

    pub fn mission_target_team(&self) -> &Option<Arc<Team>> {
        &self.team
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitHitOwnTargetMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitHitOwnTargetMessage {

}

impl fmt::Display for PlayerUnitHitOwnTargetMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:?} '{}' of '{}' ",
            self.timestamp(),
            self.player_unit().kind(),
            self.player_unit().name(),
            self.player_unit_player().name(),
        )?;

        if let Some(ref team) = self.team {
            write!(f, "teams {} ", team.name())?;
        }

        write!(f, "successfully hit his own MissionTarget \"{}\"", self.name)
    }
}