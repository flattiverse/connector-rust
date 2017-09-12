
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Task;
use Error;
use Vector;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(MissionTarget);
pub trait MissionTarget : Unit {

    /// The sequence number of this target
    fn sequence_number(&self) -> u16;

    /// (Direction-)Hints for further targets
    fn hints(&self) -> &Vec<Vector>;

    /// The radius in which the presence counts for domination
    fn domination_radius(&self) -> f32;

    /// When this number reaches 0, the [MissionTarget] is not
    /// dominated by a team anymore. When this number reaches 1
    /// the [MissionTarget] is fully dominated by the [MissionTarget]-
    /// [Team]. Each x (currently 350) ticks a [MissionTarget] is
    /// contiguous fully dominated it scores for its [Team]
    fn domination_weight(&self) -> f32;

    /// When this values reaches a certain value (currently 350)
    /// its [Team] scores and the value is reset
    fn domination_ticks(&self) -> u16;
}

pub struct MissionTargetData {
    unit:   UnitData,
    hints:  Vec<Vector>,
    sequence_number:    u16,
    domination_radius:  f32,
    domination_weight:  f32,
    domination_ticks:   u16
}

impl MissionTargetData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<MissionTargetData, Error> {
        Ok(MissionTargetData {
            unit:               UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::MissionTarget)?,
            sequence_number:    reader.read_u16()?,
            domination_radius:  reader.read_single()?,
            hints: {
                let mut vec = Vec::new();
                let count = reader.read_u16()?;
                for _ in 0..count {
                    vec.push(Vector::from_reader(reader)?);
                }
                vec
            },
            domination_weight:  reader.read_single()?,
            domination_ticks:   reader.read_u16()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for MissionTargetData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for MissionTargetData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<MissionTargetData> + BorrowMut<MissionTargetData> + Unit> MissionTarget for  T {
    fn sequence_number(&self) -> u16 {
        if let Some(connector) = self.connector().upgrade() {
            connector.register_task_quitely_if_unknown(Task::UsedSequence);
        }
        self.borrow().sequence_number
    }

    fn hints(&self) -> &Vec<Vector> {
        &self.borrow().hints
    }

    fn domination_radius(&self) -> f32 {
        self.borrow().domination_radius
    }

    fn domination_weight(&self) -> f32 {
        self.borrow().domination_weight
    }

    fn domination_ticks(&self) -> u16 {
        self.borrow().domination_ticks
    }
}