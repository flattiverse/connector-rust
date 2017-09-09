
mod energy_cost;
mod controllable;


pub use self::energy_cost::*;
pub use self::controllable::*;




use std::sync::Arc;
use std::sync::RwLock;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

/*
pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Arc<RwLock<Controllable>>, Error> {
    Ok(match packet.path_sub() {
        0 => , // platform
        1 => , // probe
        2 => , // drone
        3 => , // ship
        4 => , // base
        _ => return Err(Error::InvalidControllable(packet.path_sub()))
    })
}*/