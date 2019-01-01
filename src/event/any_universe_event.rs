
use std::sync::Arc;
use std::ops::Deref;

use crate::Error;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::event::*;

pub enum AnyUniverseEvent {
    Damage          (Arc<DamageUniverseEvent>),
    Harvest         (Arc<HarvestUniverseEvent>),
    LoadedEnergy    (Arc<LoadedEnergyUniverseEvent>),
    Repair          (Arc<RepairUniverseEvent>),
    Scan            (Arc<ScanUniverseEvent>),
    Tractorbeam     (Arc<TractorbeamUniverseEvent>),
    TransferredEnergy(Arc<TransferredEnergyUniverseEvent>),
}

impl AnyUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<AnyUniverseEvent, Error> {
        Ok(match packet.path_ship() {
            0x01 => AnyUniverseEvent::Scan              (Arc::new(ScanUniverseEvent             ::from_packet(packet, reader)?)),
            0x02 => AnyUniverseEvent::Damage            (Arc::new(DamageUniverseEvent           ::from_packet(packet, reader)?)),
            0x03 => AnyUniverseEvent::LoadedEnergy      (Arc::new(LoadedEnergyUniverseEvent     ::from_packet(packet, reader)?)),
            0x04 => AnyUniverseEvent::Repair            (Arc::new(RepairUniverseEvent           ::from_packet(packet, reader)?)),
            0x05 => AnyUniverseEvent::Harvest           (Arc::new(HarvestUniverseEvent          ::from_packet(packet, reader)?)),
            0x06 => AnyUniverseEvent::TransferredEnergy (Arc::new(TransferredEnergyUniverseEvent::from_packet(packet, reader)?)),
            0x07 => AnyUniverseEvent::Tractorbeam       (Arc::new(TractorbeamUniverseEvent      ::from_packet(packet, reader)?)),
            _ => return Err(Error::InvalidEvent(packet.path_ship()))
        })
    }
}

impl Deref for AnyUniverseEvent {
    type Target = UniverseEvent;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyUniverseEvent::Damage           (ref event) => event.deref(),
            &AnyUniverseEvent::Harvest          (ref event) => event.deref(),
            &AnyUniverseEvent::LoadedEnergy     (ref event) => event.deref(),
            &AnyUniverseEvent::Repair           (ref event) => event.deref(),
            &AnyUniverseEvent::Scan             (ref event) => event.deref(),
            &AnyUniverseEvent::Tractorbeam      (ref event) => event.deref(),
            &AnyUniverseEvent::TransferredEnergy(ref event) => event.deref(),
        }
    }
}