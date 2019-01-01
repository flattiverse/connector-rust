
use crate::Task;
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_unit::prelude::*;

pub struct MissionTarget {
    unit:   UnitData,
    hints:  Vec<Vector>,
    sequence_number:    u16,
    domination_radius:  f32,
    domination_weight:  f32,
    domination_ticks:   u16
}

impl MissionTarget {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<MissionTarget, Error> {
        Ok(MissionTarget {
            unit:               UnitData::from_reader(connector, universe_group, packet, reader)?,
            sequence_number:    reader.read_u16()?,
            domination_radius:  reader.read_single()?,
            hints: {
                let mut vec = Vec::new();
                let count = reader.read_unsigned_byte()?;
                for _ in 0..count {
                    vec.push(Vector::from_reader(reader)?);
                }
                vec
            },
            domination_weight:  reader.read_single()?,
            domination_ticks:   reader.read_u16()?,
        })
    }

    /// The sequence number of this target
    pub fn sequence_number(&self) -> u16 {
        if let Some(connector) = self.unit.connector().upgrade() {
            connector.register_task_quitely_if_unknown(Task::UsedSequence);
        }
        self.sequence_number
    }

    /// (Direction-)Hints for further targets
    pub fn hints(&self) -> &Vec<Vector> {
        &self.hints
    }

    /// The radius in which the presence counts for domination
    pub fn domination_radius(&self) -> f32 {
        self.domination_radius
    }

    /// When this number reaches 0, the [MissionTarget] is not
    /// dominated by a team anymore. When this number reaches 1
    /// the [MissionTarget] is fully dominated by the [MissionTarget]-
    /// [Team]. Each x (currently 350) ticks a [MissionTarget] is
    /// contiguous fully dominated it scores for its [Team]
    pub fn domination_weight(&self) -> f32 {
        self.domination_weight
    }

    /// When this values reaches a certain value (currently 350)
    /// its [Team] scores and the value is reset
    pub fn domination_ticks(&self) -> u16 {
        self.domination_ticks
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for MissionTarget {
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
        UnitKind::MissionTarget
    }
}