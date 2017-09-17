
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

pub struct PlayerJoinedUniverseGroupMessage {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
}

impl PlayerJoinedUniverseGroupMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerJoinedUniverseGroupMessage, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerJoinedUniverseGroupMessage {
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
impl Message for PlayerJoinedUniverseGroupMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for PlayerJoinedUniverseGroupMessage {

}

impl fmt::Display for PlayerJoinedUniverseGroupMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} joined the game.",
            self.timestamp(),
            self.player.name(),
            self.team.name(),
        )
    }
}