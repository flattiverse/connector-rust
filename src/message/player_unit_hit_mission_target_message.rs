
use std::fmt;
use std::sync::Arc;

use Team;
use Error;
use Player;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use unit::ControllableInfo;

use message::any_game_message::prelude::*;

pub struct PlayerUnitHitMissionTargetMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    info:   Arc<ControllableInfo>,
    name:   String,
    team:   Option<Arc<Team>>,
    seq:    u16,
}

impl PlayerUnitHitMissionTargetMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitHitMissionTargetMessage, Error> {
        let data   = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitHitMissionTargetMessage {
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

    pub fn mission_target_sequence(&self) -> u16 {
        self.seq
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerUnitHitMissionTargetMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerUnitHitMissionTargetMessage {

}

impl fmt::Display for PlayerUnitHitMissionTargetMessage {
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

        write!(f, "MissionTarget \"{}\"", self.name)?;
        if self.seq > 0 {
            write!(f, " with sequence number {}", self.seq)?;
        }
        write!(f, ".")
    }
}