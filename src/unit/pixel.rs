
use Error;
use Color;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::any_unit::prelude::*;

pub struct Pixel {
    unit:   UnitData,
    color:  Color,
}

impl Pixel {
    pub fn new(connector: &Arc<Connector>, group: &UniverseGroup, name: String, radius: f32,
               position: Vector, r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            unit: UnitData::new(
                connector,
                group,
                name,
                radius,
                0_f32,
                position,
                Vector::new(0_f32, 0_f32),
                false,
                false,
                true,
                Mobility::Still
            ),
            color: Color::from_rgb(
                r as f32 / 255_f32,
                g as f32 / 255_f32,
                b as f32 / 255_f32,
            )
        }
    }

    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Pixel, Error> {
        Ok(Pixel {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_unsigned_byte()? as f32 / 255_f32,
                reader.read_unsigned_byte()? as f32 / 255_f32,
                reader.read_unsigned_byte()? as f32 / 255_f32,
            ),
        })
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn red(&self) -> f32 {
        self.color.red
    }

    pub fn green(&self) -> f32 {
        self.color.green
    }

    pub fn blue(&self) -> f32 {
        self.color.blue
    }

    pub fn alpha(&self) -> f32 {
        self.color.alpha
    }

    pub fn is_relevant(&self) -> bool {
        let color = self.color();
        color.red() > 0_f32 || color.green() > 0_f32 || (color.blue() - 1_f32).abs() > ::std::f32::EPSILON
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Pixel {
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
        UnitKind::Pixel
    }
}