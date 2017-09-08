
mod mobility;
mod unit_kind;
mod scan_info;
mod orbiting_state;
mod controllable_info;


mod sun;
mod unit;
mod planet;
mod corona;



pub use self::mobility::*;
pub use self::unit_kind::*;
pub use self::scan_info::*;
pub use self::orbiting_state::*;
pub use self::controllable_info::*;

pub use self::sun::*;
pub use self::unit::*;
pub use self::planet::*;
pub use self::corona::*;




use std::sync::Arc;
use std::sync::RwLock;

use Error;
use Connector;
use UniverseGroup;
use net::Packet;
use net::BinaryReader;

pub fn unit_from_packet(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet) -> Result<Arc<RwLock<Unit>>, Error> {
    let reader = &mut packet.read() as &mut BinaryReader;

    Ok(match packet.path_ship() as u8 {
        0x01 /*   1 */ => Arc::new(RwLock::new(SunData::from_reader(connector, universe_group, packet, reader)?)),
        0x02 /*   2 */ => Arc::new(RwLock::new(PlanetData::from_reader(connector, universe_group, packet, reader)?)),
        id@_ => return Err(Error::UnknownUnitType(id)),
    })
}