
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

downcast!(PlayerKickedFromUniverseGroupMessage);
pub trait PlayerKickedFromUniverseGroupMessage : GameMessage {

    fn player(&self) -> &Arc<Player>;

    fn universe_group(&self) -> &Arc<UniverseGroup>;

    fn team(&self) -> &Arc<Team>;

    fn reason(&self) -> &str;
}

pub struct PlayerKickedFromUniverseGroupMessageData {
    data:   GameMessageData,
    player: Arc<Player>,
    group:  Arc<UniverseGroup>,
    team:   Arc<Team>,
    reason: String,
}

impl PlayerKickedFromUniverseGroupMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerKickedFromUniverseGroupMessageData, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        let group = connector.universe_group(reader.read_u16()?)?;
        let team = group.team(reader.read_unsigned_byte()?)?;

        Ok(PlayerKickedFromUniverseGroupMessageData {
            data,
            player,
            group,
            team,
            reason: reader.read_string()?,
        })
    }
}

impl Borrow<GameMessageData> for PlayerKickedFromUniverseGroupMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for PlayerKickedFromUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for PlayerKickedFromUniverseGroupMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for PlayerKickedFromUniverseGroupMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<PlayerKickedFromUniverseGroupMessageData> + BorrowMut<PlayerKickedFromUniverseGroupMessageData> + GameMessage> PlayerKickedFromUniverseGroupMessage for T {
    fn player(&self) -> &Arc<Player> {
        &self.borrow().player
    }

    fn universe_group(&self) -> &Arc<UniverseGroup> {
        &self.borrow().group
    }

    fn team(&self) -> &Arc<Team> {
        &self.borrow().team
    }

    fn reason(&self) -> &str {
        &self.borrow().reason
    }
}

impl fmt::Display for PlayerKickedFromUniverseGroupMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Player {} from Team {} has been kicked from the game: {}.",
            (self as &FlattiverseMessage).timestamp(),
            self.player.name(),
            self.team.name(),
            self.reason()
        )
    }
}