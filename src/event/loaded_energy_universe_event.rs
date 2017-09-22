
use std::fmt;

use Error;

use unit::UnitKind;

use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

#[derive(Debug)]
pub struct LoadedEnergyUniverseEvent {
    data: UniverseEventData,
    energy: f32,
    particles: f32,
    ions: f32
}

impl LoadedEnergyUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<LoadedEnergyUniverseEvent, Error> {
        Ok(LoadedEnergyUniverseEvent {
            data:       UniverseEventData::from_reader(packet, reader)?,
            energy:     reader.read_single()?,
            particles:  reader.read_single()?,
            ions:       reader.read_single()?
        })
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn particles(&self) -> f32 {
        self.particles
    }

    pub fn ions(&self) -> f32 {
        self.ions
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for LoadedEnergyUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for LoadedEnergyUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoadedEnergyUniverseEvent: {}; {}; {}",
            self.energy,
            self.particles,
            self.ions
        )
    }
}