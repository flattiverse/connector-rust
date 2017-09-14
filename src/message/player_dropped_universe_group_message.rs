
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

downcast!(PlayerDroppedFromUniverseGroupMessage);
pub trait PlayerDroppedFromUniverseGroupMessage : GameMessage {

    fn player(&self) -> &Arc<Player>;

    fn universe_group(&self) -> &Arc<UniverseGroup>;

    fn team(&self) -> &Arc<Team>;
}

pub struct PlayerDroppedFromUniverseGroupMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
}

impl PlayerDroppedFromUniverseGroupMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerDroppedFromUniverseGroupMessageData, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerDroppedFromUniverseGroupMessageData {
            data,
            player,
            group,
            team
        })
    }
}

impl Borrow<GameMessageData> for PlayerDroppedFromUniverseGroupMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerDroppedFromUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerDroppedFromUniverseGroupMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerDroppedFromUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerDroppedFromUniverseGroupMessageData> + BorrowMut<PlayerDroppedFromUniverseGroupMessageData> + GameMessage> PlayerDroppedFromUniverseGroupMessage for T {
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

impl fmt::Display for PlayerDroppedFromUniverseGroupMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} got dropped from the game.",
            (self as &FlattiverseMessage).timestamp(),
            self.player.name(),
            self.team.name(),
        )
    }
}