
mod energy_cost;
mod scan_energy_cost;
mod weapon_energy_cost;

mod sub_direction;

mod probe;
mod platform;
mod controllable;


pub use self::energy_cost::*;
pub use self::scan_energy_cost::*;
pub use self::weapon_energy_cost::*;

pub use self::sub_direction::*;

pub use self::probe::*;
pub use self::platform::*;
pub use self::controllable::*;




use std::sync::Arc;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;


pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Arc<Controllable>, Error> {
    Ok(match packet.path_sub() {
        0 => Arc::new(PlatformData  ::from_reader(connector, packet, reader)?), // platform
        1 => Arc::new(ProbeData     ::from_reader(connector, packet, reader)?), // probe
        2 => , // drone
        3 => , // ship
        4 => , // base
        _ => return Err(Error::InvalidControllable(packet.path_sub()))
    })
}