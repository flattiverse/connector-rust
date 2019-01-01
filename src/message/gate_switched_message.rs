
use std::fmt;
use std::sync::Arc;

use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::unit::GateSwitchInfo;
use crate::unit::ControllableInfo;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::message::any_game_message::prelude::*;

pub struct GateSwitchedMessage {
    data:   GameMessageData,
    player: Option<Arc<Player>>,
    info:   Option<Arc<ControllableInfo>>,
    switch: String,
    gates:  Vec<GateSwitchInfo>,
}

impl GateSwitchedMessage {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<GateSwitchedMessage, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let invoked = reader.read_bool()?;
        let player;
        let info;

        if invoked {
            let p_strong = connector.player_for(reader.read_u16()?)?;
            let index = reader.read_unsigned_byte()?;

            player = Some(p_strong.clone());
            info   = Some(p_strong.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?);
        } else  {
            player = None;
            info   = None;
        }

        Ok(GateSwitchedMessage {
            data,
            player,
            info,
            switch: reader.read_string()?,
            gates: {
                let count = reader.read_u16()?;
                let mut vec = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    vec.push(GateSwitchInfo::from_reader(reader)?);
                }
                vec
            },
        })
    }

    pub fn invoker_player(&self) -> &Option<Arc<Player>> {
        &self.player
    }

    pub fn invoker_player_info(&self) -> &Option<Arc<ControllableInfo>> {
        &self.info
    }

    pub fn switch_string(&self) -> &str {
        &self.switch
    }

    pub fn gates(&self) -> &Vec<GateSwitchInfo> {
        &self.gates
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Message for GateSwitchedMessage {
    fn timestamp(&self) -> &DateTime {
        self.data.timestamp()
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl GameMessage for GateSwitchedMessage {

}

impl fmt::Display for GateSwitchedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", self.timestamp())?;

        if self.player.is_some() {
            let info = self.info.clone().unwrap();
            let player = self.player.clone().unwrap();
            write!(f, "{:?} of {}", info.name(), player.name())?;

        } else {
            write!(f, "A neutral unit ")?;
        }

        write!(f, " triggered Switch {}. {} Gate", self.switch, self.gates.len())?;

        if self.gates.len() > 0 {
            write!(f, "s")?;
        }

        write!(f, " affected, including: ")?;
        for i in 0..self.gates.len() {
            let gate = &self.gates[i];
            if i > 0 {
                write!(f, "; ")?;
            }
            write!(f, "{} ({})",
                gate.name(),
                if gate.state() {"Closed"} else {"Open"},
            )?;

            if i == 11 && self.gates.len() > 12 {
                write!(f, "..")?;
                break;
            }
        }
        write!(f, ".")
    }
}