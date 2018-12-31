
use std::io::Read;

use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::any_unit::prelude::*;

use flate2::read::GzDecoder;

pub struct PixelCluster {
    unit: UnitData,
    data: Vec<u8>,
}

impl PixelCluster {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PixelCluster, Error> {
        Ok(PixelCluster {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            data: {
                let count = reader.read_unsigned_byte()?;
                if count == 0 {
                    let mut vec = vec!(0u8; 768);
                    reader.read_exact(&mut vec[..])?;
                    vec

                } else {
                    let bytes = reader.read_bytes_available(count as usize)?;
                    let read = &mut &bytes[..] as &mut Read;
                    let mut decoder = GzDecoder::new(read);
                    let mut vec = Vec::new();
                    decoder.read_to_end(&mut vec)?;
                    vec
                }
            }
        })
    }


    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for PixelCluster {
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
        UnitKind::PixelCluster
    }
}