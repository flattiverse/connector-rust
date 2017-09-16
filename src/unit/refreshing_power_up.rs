
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::any_refreshing_power_up::prelude::*;

pub trait RefreshingPowerUp : PowerUp {
    fn amount(&self) -> f32;
}

pub(crate) struct RefreshingPowerUpData {
    unit:   PowerUpData,
    amount: f32,
}

impl RefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<RefreshingPowerUpData, Error> {
        Ok(RefreshingPowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader)?,
            amount: reader.read_single()?,
        })
    }
}

impl RefreshingPowerUp for RefreshingPowerUpData {
    fn amount(&self) -> f32 {
        self.amount
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for RefreshingPowerUpData {
    fn name(&self) -> &str {
        self.unit.name()
    }

    fn position(&self) -> &Vector {
        self.unit.position()
    }

    fn movement(&self) -> &Vector {
        self.unit.movement()
    }

    fn radius(&self) -> f32 {
        self.unit.radius()
    }

    fn gravity(&self) -> f32 {
        self.unit.gravity()
    }

    fn team(&self) -> &Weak<Team> {
        self.unit.team()
    }

    fn is_solid(&self) -> bool {
        self.unit.is_solid()
    }

    fn is_masking(&self) -> bool {
        self.unit.is_masking()
    }

    fn is_visible(&self) -> bool {
        self.unit.is_visible()
    }

    fn is_orbiting(&self) -> bool {
        self.unit.is_orbiting()
    }

    fn orbiting_center(&self) -> &Option<Vector> {
        self.unit.orbiting_center()
    }

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>> {
        self.unit.orbiting_states()
    }

    fn mobility(&self) -> Mobility {
        self.unit.mobility()
    }

    fn connector(&self) -> &Weak<Connector> {
        self.unit.connector()
    }

    fn kind(&self) -> UnitKind {
        unimplemented!();
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl PowerUp for RefreshingPowerUpData {

}