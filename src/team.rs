
use std::fmt;
use std::cmp::PartialEq;

use std::sync::Arc;
use std::sync::Weak;

use crate::Error;
use crate::Color;
use crate::Scores;
use crate::GameType;
use crate::Connector;
use crate::UniverseGroup;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;
use crate::net::BinaryWriter;

pub struct Team {
    universe_group: Weak<UniverseGroup>,
    connector: Weak<Connector>,
    id: u8,
    color: Color,
    name: String,
    scores: Option<Scores>
}

impl Team {
    pub fn from_reader(connector: Weak<Connector>, universe_group: &Arc<UniverseGroup>, packet: &Packet, reader: &mut BinaryReader) -> Result<Team, Error> {
        let scores = if let Some(GameType::Mission) = universe_group.game_type() {
            None
        } else {
            Some(Scores::default())
        };

        Ok(Team {
            universe_group: Arc::downgrade(&universe_group),
            connector,
            id: packet.path_sub(),
            color: Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            name: reader.read_string()?,
            scores,
        })
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn scores(&self) -> &Option<Scores> {
        &self.scores
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn chat(&self, message: &str) -> Result<(), Error> {
        let connector = self.connector.upgrade().unwrap();

        if message.is_empty() || message.len() > 140 {
            return Err(Error::InvalidChatMessage)
        }

        {
            let uni_group = &self.universe_group.upgrade().unwrap();
            let player = connector.player();

            match player.upgrade() {
                None => return Err(Error::CannotSendMessageIntoAnotherUniverseGroup),
                Some(ref player) => {
                    // TODO lots of unwrap...
                    let player = player.clone();
                    let player_uni = player.universe_group().clone();
                    let player_uni = player_uni.upgrade().unwrap();
                    if player_uni.eq(&uni_group) {
                        return Err(Error::CannotSendMessageIntoAnotherUniverseGroup)
                    }
                }
            }
        }

        let mut block = connector.block_manager().block()?;
        let mut packet = Packet::default();

        packet.set_command(0x31);
        packet.set_path_sub(self.id);
        packet.set_session(block.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(&message)?;
        }

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn universe_group(&self) -> &Weak<UniverseGroup> {
        &self.universe_group
    }
}

impl UniversalEnumerable for Team {
    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Team) -> bool {
        let me = self.universe_group().upgrade();
        let ot = other.universe_group().upgrade();

        if me.is_some() && ot.is_some() {
            let me = me.unwrap();
            let ot = ot.unwrap();

            self.id == other.id && me.eq(&ot)
        } else {
            self.id == other.id && me.is_none() == ot.is_none()
        }
    }
}

impl fmt::Debug for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}