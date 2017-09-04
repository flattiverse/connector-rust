
use std::fmt;
use std::cmp::PartialEq;

use std::sync::Weak;
use std::sync::RwLock;

use Error;
use Color;
use Scores;
use GameType;
use Connector;
use UniverseGroup;
use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;

pub struct Team {
    universe_group: Weak<RwLock<UniverseGroup>>,
    connector: Weak<Connector>,
    id: u8,
    color: Color,
    name: String,
    scores: Option<RwLock<Scores>>
}

impl Team {
    pub fn new(connector: Weak<Connector>, universe_group: Weak<RwLock<UniverseGroup>>, packet: &Packet) -> Result<Team, Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        let ug = universe_group.upgrade().unwrap();
        let scores = if let Some(GameType::Mission) = ug.read().unwrap().game_type() {
            Some(RwLock::new(Scores::default()))
        } else {
            None
        };

        Ok(Team {
            universe_group: universe_group,
            connector: connector,
            id: packet.path_sub(),
            color: Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            name: reader.read_string()?,
            scores: scores,
        })
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn scores(&self) -> &Option<RwLock<Scores>> {
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
            let uni_group = uni_group.read().unwrap();
            let player = connector.player();

            match player {
                &None => return Err(Error::CannotSendMessageIntoAnotherUniverseGroup),
                &Some(ref player) => {
                    let player = player.clone();
                    let player = player.read().unwrap();
                    let player_uni = player.universe_group().upgrade().unwrap();
                    let player_uni = player_uni.read().unwrap();
                    if player_uni.eq(&uni_group) {
                        return Err(Error::CannotSendMessageIntoAnotherUniverseGroup)
                    }
                }
            }
        }

        let block = connector.block_manager().block()?;
        let mut packet = Packet::new();

        packet.set_command(0x31);
        packet.set_path_sub(self.id);
        packet.set_session(block.lock().unwrap().id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(&message)?;
        }

        connector.send(&packet)?;
        block.lock().unwrap().wait()?;
        Ok(())
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn universe_group(&self) -> &Weak<RwLock<UniverseGroup>> {
        &self.universe_group
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
            let me = me.read().unwrap();
            let ot = ot.read().unwrap();

            self.id == other.id && me.eq(&ot)
        } else {
            self.id == other.id && me.is_none() == ot.is_none()
        }
    }
}