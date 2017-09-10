
mod energy_cost;
mod scan_energy_cost;
mod weapon_energy_cost;

mod sub_direction;
mod controllable;


pub use self::energy_cost::*;
pub use self::scan_energy_cost::*;
pub use self::weapon_energy_cost::*;

pub use self::sub_direction::*;
pub use self::controllable::*;




use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;


pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Arc<Controllable>, Error> {
    Ok(match packet.path_sub() {
        0 => , // platform
        1 => , // probe
        2 => , // drone
        3 => , // ship
        4 => , // base
        _ => return Err(Error::InvalidControllable(packet.path_sub()))
    })
}