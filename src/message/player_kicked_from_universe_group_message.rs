
use std::fmt;
use std::sync::Arc;

use Team;
use Error;
use Player;
use Connector;
use UniverseGroup;

use net::Packet;
use net::BinaryReader;

use message::any_game_message::prelude::*;

pub struct PlayerKickedFromUniverseGroupMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
    reason: String,
}

impl PlayerKickedFromUniverseGroupMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerKickedFromUniverseGroupMessage, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerKickedFromUniverseGroupMessage {
            data,
            player,
            group,
            team,
            reason: reader.read_string()?,
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

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for PlayerKickedFromUniverseGroupMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerKickedFromUniverseGroupMessage {

}

impl fmt::Display for PlayerKickedFromUniverseGroupMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} has been kicked from the game: {}.",
            self.timestamp(),
            self.player.name(),
            self.team.name(),
            self.reason()
        )
    }
}