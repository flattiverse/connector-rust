
mod universe_event;
mod scan_universe_event;
mod damage_universe_event;
mod loaded_energy_universe_event;
mod repair_universe_event;
mod harvest_universe_event;
mod transferred_energy_universe_event;
mod tractorbeam_universe_event;

pub use self::universe_event::*;
pub use self::scan_universe_event::*;
pub use self::damage_universe_event::*;
pub use self::loaded_energy_universe_event::*;
pub use self::repair_universe_event::*;
pub use self::harvest_universe_event::*;
pub use self::transferred_energy_universe_event::*;
pub use self::tractorbeam_universe_event::*;





use std::sync::Arc;

use Error;
use net::Packet;
use net::BinaryReader;

pub fn event_from_packet(packet: &Packet) -> Result<Arc<UniverseEvent>, Error> {
    let reader = &mut packet.read() as &mut BinaryReader;
    Ok(match packet.path_ship() {
        0x01 => Arc::new(ScanUniverseEventData              ::from_packet(packet, reader)?),
        0x02 => Arc::new(DamageUniverseEventData            ::from_packet(packet, reader)?),
        0x03 => Arc::new(LoadedEnergyUniverseEventData      ::from_packet(packet, reader)?),
        0x04 => Arc::new(RepairUniverseEventData            ::from_packet(packet, reader)?),
        0x05 => Arc::new(HarvestUniverseEventData           ::from_packet(packet, reader)?),
        0x06 => Arc::new(TransferredEnergyUniverseEventData ::from_packet(packet, reader)?),
        0x07 => Arc::new(TractorbeamUniverseEventData       ::from_packet(packet, reader)?),
        _ => return Err(Error::InvalidEvent(packet.path_ship()))
    })
}