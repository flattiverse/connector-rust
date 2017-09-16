
use Error;
use Color;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::any_unit::prelude::*;

pub struct Gate {
    unit:       UnitData,
    color:      Color,
    switched:   bool,
}

impl Gate {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Gate, Error> {
        Ok(Gate {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            switched: reader.read_bool()?,
        })
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn is_switched(&self) -> bool {
        self.switched
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Gate {
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
        UnitKind::Gate
    }
}