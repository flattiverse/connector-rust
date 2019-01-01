
use std::fmt;
use std::sync::Arc;

use crate::Team;
use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniverseGroup;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct PlayerPartedUniverseGroupMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
}

impl PlayerPartedUniverseGroupMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerPartedUniverseGroupMessage, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerPartedUniverseGroupMessage {
            data,
            player,
            group,
            team
        })
    }

    pub fn player(&self) -> &Arc<Player> {
        &self.player
    }

    pub fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.group
    }

    pub fn team(&self) -> &Arc<Team> {
        &self.team
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerPartedUniverseGroupMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerPartedUniverseGroupMessage {

}

impl fmt::Display for PlayerPartedUniverseGroupMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} parted the game.",
            self.timestamp(),
            self.player.name(),
            self.team.name(),
        )
    }
}