
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(LoadedEnergyUniverseEvent);
pub trait LoadedEnergyUniverseEvent : UniverseEvent + Display + Debug {

    fn energy(&self) -> f32;

    fn particles(&self) -> f32;

    fn ions(&self) -> f32;
}


#[derive(Debug)]
pub struct LoadedEnergyUniverseEventData {
    data: UniverseEventData,
    energy: f32,
    particles: f32,
    ions: f32
}

impl LoadedEnergyUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<LoadedEnergyUniverseEventData, Error> {
        Ok(LoadedEnergyUniverseEventData {
            data:       UniverseEventData::from_reader(packet, reader)?,
            energy:     reader.read_single()?,
            particles:  reader.read_single()?,
            ions:       reader.read_single()?
        })
    }
}

// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for LoadedEnergyUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}


impl<T: 'static + Borrow<LoadedEnergyUniverseEventData> + UniverseEvent + Display + Debug> LoadedEnergyUniverseEvent for T {
    fn energy(&self) -> f32 {
        self.borrow().energy
    }

    fn particles(&self) -> f32 {
        self.borrow().particles
    }

    fn ions(&self) -> f32 {
        self.borrow().ions
    }
}

impl Display for LoadedEnergyUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoadedEnergyUniverseEvent: {}; {}; {}",
            self.energy,
            self.particles,
            self.ions
        )
    }
}