
use std::fmt;
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Team;
use Error;
use Player;
use Connector;
use UniverseGroup;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(PlayerJoinedUniverseGroupMessage);
pub trait PlayerJoinedUniverseGroupMessage : GameMessage {

    fn player(&self) -> &Arc<Player>;

    fn universe_group(&self) -> &Arc<UniverseGroup>;

    fn team(&self) -> &Arc<Team>;
}

pub struct PlayerJoinedUniverseGroupMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
}

impl PlayerJoinedUniverseGroupMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerJoinedUniverseGroupMessageData, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerJoinedUniverseGroupMessageData {
            data,
            player,
            group,
            team
        })
    }
}

impl Borrow<GameMessageData> for PlayerJoinedUniverseGroupMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerJoinedUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerJoinedUniverseGroupMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerJoinedUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerJoinedUniverseGroupMessageData> + BorrowMut<PlayerJoinedUniverseGroupMessageData> + GameMessage> PlayerJoinedUniverseGroupMessage for T {
    fn player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.borrow().group
    }

    fn team(&self) -> &Arc<Team> {
        &self.borrow().team
    }
}

impl fmt::Display for PlayerJoinedUniverseGroupMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} joined the game.",
            (self as &FlattiverseMessage).timestamp(),
            self.player.name(),
            self.team.name(),
        )
    }
}