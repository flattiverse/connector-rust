
use std::sync::Arc;
use std::ops::Deref;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::*;

pub(crate) mod prelude {
    pub use ::std::sync::Arc;
    pub use ::std::sync::Weak;
    pub use ::std::sync::RwLock;
    pub use ::std::sync::RwLockReadGuard;

    pub use Vector;
    pub use Scores;
    pub use Universe;

    pub use unit::Unit;
    pub use unit::AnyUnit;
    pub use unit::UnitKind;
    pub use unit::ScanInfo;

    pub use item::AnyCargoItem;
    pub use item::CrystalCargoItem;

    pub use controllable::AnyControllable;

    pub use controllable::EnergyCost;
    pub use controllable::ScanEnergyCost;
    pub use controllable::WeaponEnergyCost;
}

#[derive(Clone)]
pub enum AnyControllable {
    Ship    (Arc<Ship>),
    Base    (Arc<Base>),
    Probe   (Arc<Probe>),
    Drone   (Arc<Drone>),
    Platform(Arc<Platform>),
}

impl AnyControllable {
    pub fn from_packet(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<AnyControllable, Error> {
        Ok(match packet.path_sub() {
            0 => AnyControllable::Platform(Arc::new(Platform::from_reader(connector, packet, reader)?)),
            1 => AnyControllable::Probe   (Arc::new(Probe   ::from_reader(connector, packet, reader)?)),
            2 => AnyControllable::Drone   (Arc::new(Drone   ::from_reader(connector, packet, reader)?)),
            3 => AnyControllable::Ship    (Arc::new(Ship    ::from_reader(connector, packet, reader)?)),
            4 => AnyControllable::Base    (Arc::new(Base    ::from_reader(connector, packet, reader)?)),
            _ => return Err(Error::InvalidControllable(packet.path_sub()))
        })
    }
}

impl Deref for AnyControllable {
    type Target = Controllable;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyControllable::Platform(ref controllable) => controllable.deref(),
            &AnyControllable::Probe   (ref controllable) => controllable.deref(),
            &AnyControllable::Drone   (ref controllable) => controllable.deref(),
            &AnyControllable::Ship    (ref controllable) => controllable.deref(),
            &AnyControllable::Base    (ref controllable) => controllable.deref(),
        }
    }
}