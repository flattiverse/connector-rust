
mod energy_cost;
mod scan_energy_cost;
mod weapon_energy_cost;

mod sub_direction;

mod base;
mod ship;
mod drone;
mod probe;
mod empty;
mod platform;
mod controllable;
mod controllable_design;


pub use self::energy_cost::*;
pub use self::scan_energy_cost::*;
pub use self::weapon_energy_cost::*;

pub use self::sub_direction::*;

pub use self::base::*;
pub use self::ship::*;
pub use self::drone::*;
pub use self::probe::*;
pub use self::empty::*;
pub use self::platform::*;
pub use self::controllable::*;
pub use self::controllable_design::*;




use std::sync::Arc;
use std::sync::RwLock;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;


pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Arc<Controllable>, Error> {
    Ok(match packet.path_sub() {
        0 => Arc::new(PlatformData  ::from_reader(connector, packet, reader)?), // platform
        1 => Arc::new(ProbeData     ::from_reader(connector, packet, reader)?), // probe
        2 => Arc::new(DroneData     ::from_reader(connector, packet, reader)?), // drone
        3 => Arc::new(ShipData      ::from_reader(connector, packet, reader)?), // ship
        4 => Arc::new(BaseData      ::from_reader(connector, packet, reader)?), // base
        _ => return Err(Error::InvalidControllable(packet.path_sub()))
    })
}