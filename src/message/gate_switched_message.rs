
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniversalEnumerable;

use unit::GateSwitchInfo;
use unit::ControllableInfo;

use net::Packet;
use net::BinaryReader;

use message::GameMessage;
use message::GameMessageData;
use message::FlattiverseMessage;
use message::FlattiverseMessageData;

downcast!(GateSwitchedMessage);
pub trait GateSwitchedMessage : GameMessage {

    fn invoker_player(&self) -> &Option<Arc<RwLock<Player>>>;

    fn invoker_player_info(&self) -> &Option<Arc<RwLock<ControllableInfo>>>;

    fn switch_string(&self) -> &str;

    fn gates(&self) -> &Vec<GateSwitchInfo>;
}

pub struct GateSwitchedMessageData {
    data:   GameMessageData,
    player: Option<Arc<RwLock<Player>>>,
    info:   Option<Arc<RwLock<ControllableInfo>>>,
    switch: String,
    gates:  Vec<GateSwitchInfo>,
}

impl GateSwitchedMessageData {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<GateSwitchedMessageData, Error> {
        let data = GameMessageData::from_packet(connector, packet, reader)?;
        let invoked = reader.read_bool()?;
        let player;
        let info;

        if invoked {
            let p_strong = connector.player_for(reader.read_u16()?)?;
            player = Some(p_strong.clone());
            let index = reader.read_unsigned_byte()?;
            info   = Some(p_strong.read()?.controllable_info(index).ok_or(Error::InvalidControllableInfo(index))?);
        } else  {
            player = None;
            info   = None;
        }

        Ok(GateSwitchedMessageData {
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
}

impl Borrow<GameMessageData> for GateSwitchedMessageData {
    fn borrow(&self) -> &GameMessageData {
        &self.data
    }
}
impl BorrowMut<GameMessageData> for GateSwitchedMessageData {
    fn borrow_mut(&mut self) -> &mut GameMessageData {
        &mut self.data
    }
}
impl Borrow<FlattiverseMessageData> for GateSwitchedMessageData {
    fn borrow(&self) -> &FlattiverseMessageData {
        (self.borrow() as &GameMessageData).borrow()
    }
}
impl BorrowMut<FlattiverseMessageData> for GateSwitchedMessageData {
    fn borrow_mut(&mut self) -> &mut FlattiverseMessageData {
        (self.borrow_mut() as &mut GameMessageData).borrow_mut()
    }
}


impl<T: 'static + Borrow<GateSwitchedMessageData> + BorrowMut<GateSwitchedMessageData> + GameMessage> GateSwitchedMessage for T {
    fn invoker_player(&self) -> &Option<Arc<RwLock<Player>>> {
        &self.borrow().player
    }

    fn invoker_player_info(&self) -> &Option<Arc<RwLock<ControllableInfo>>> {
        &self.borrow().info
    }

    fn switch_string(&self) -> &str {
        &self.borrow().switch
    }

    fn gates(&self) -> &Vec<GateSwitchInfo> {
        &self.borrow().gates
    }
}

impl fmt::Display for GateSwitchedMessageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] ", (self as &FlattiverseMessage).timestamp())?;

        if self.player.is_some() {
            let info = self.info.clone().unwrap();
            let info = info.read().unwrap();
            let player = self.player.clone().unwrap();
            let player = player.read().unwrap();
            write!(f, "{:?} of {}",
                info.name(),
                player.name(),
            )?;
        } else {
            write!(f, "A neutral unit ")?;
        }

        write!(f, " triggered Switch {}. {} Gate",
            self.switch,
            self.gates.len()
        )?;

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