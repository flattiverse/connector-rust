
use std::sync::Arc;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Task;
use Team;
use Error;
use Vector;
use Connector;
use UniverseGroup;
use unit::Mobility;
use unit::UnitKind;
use unit::OrbitingState;
use net::Packet;
use net::BinaryReader;
use net::is_set_u8;


pub trait Unit {
    fn name(&self) -> &str;

    fn position(&self) -> &Vector;

    fn movement(&self) -> &Vector;

    fn radius(&self) -> f32;

    fn gravity(&self) -> f32;

    fn team(&self) -> &Option<Arc<RwLock<Team>>>;

    fn solid(&self) -> bool;

    fn masking(&self) -> bool;

    fn visible(&self) -> bool;

    fn orbiting(&self) -> bool;

    fn orbiting_center(&self) -> &Option<Vector>;

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>>;

    fn mobility(&self) -> Mobility;

    fn connector(&self) -> &Arc<Connector>;

    fn kind(&self) -> UnitKind;
}


pub struct UnitData {
    pub(crate) name: String,
    pub(crate) position: Vector,
    pub(crate) movement: Vector,
    pub(crate) radius: f32,
    pub(crate) gravity: f32,
    pub(crate) team: Option<Arc<RwLock<Team>>>,
    pub(crate) solid: bool,
    pub(crate) masking: bool,
    pub(crate) visible: bool,
    pub(crate) orbiting: bool,
    pub(crate) orbiting_center: Option<Vector>,
    pub(crate) orbiting_state: Option<Vec<OrbitingState>>,
    pub(crate) mobility: Mobility,
    pub(crate) connector: Arc<Connector>,
}

impl UnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<UnitData, Error> {
        let team = universe_group.team(packet.path_sub()).clone();

        let name = reader.read_string()?;
        let radius = reader.read_single()?;
        let gravity = reader.read_single()?;
        let position = Vector::from_reader(reader)?;
        let movement = Vector::from_reader(reader)?;

        let header = reader.read_unsigned_byte()?;

        let solid = is_set_u8(header, 0x04);
        let masking = is_set_u8(header, 0x08);
        let visible = is_set_u8(header, 0x10);
        let orbiting = is_set_u8(header, 0x20);
        let mobility = Mobility::from_id(header & 0x03).unwrap();

        let orbiting_center;
        let orbiting_list;

        if orbiting {
            orbiting_center = Some(Vector::from_reader_with_connector(reader, connector)?);
            let count = reader.read_unsigned_byte()?;
            let mut list = Vec::new();

            for _ in 0..count {
                list.push(OrbitingState::from_reader(reader)?);
            }

            orbiting_list = Some(list);

        } else {
            orbiting_center = None;
            orbiting_list   = None;
        }

        Ok(UnitData {
            name,
            position,
            movement,
            radius,
            gravity,
            team,
            solid,
            masking,
            visible,
            orbiting,
            orbiting_center,
            orbiting_state: orbiting_list,
            mobility,
            connector: connector.clone()
        })
    }

    pub fn new(connector: &Arc<Connector>, _: &UniverseGroup, name: String, radius: f32,
               gravity: f32, position: Vector, movement: Vector, solid: bool, masking: bool,
               visible: bool, mobility: Mobility) -> UnitData {
        UnitData {
            connector: connector.clone(),
            name,
            radius,
            gravity,
            position,
            movement,
            solid,
            masking,
            visible,
            mobility,

            // hardcoded
            team: None,
            orbiting: false,
            orbiting_center: None,
            orbiting_state:  None,
        }
    }
}

impl<T: Borrow<UnitData> + BorrowMut<UnitData>> Unit for T {
    fn name(&self) -> &str {
        &self.borrow().name
    }

    fn position(&self) -> &Vector {
        &self.borrow().position
    }

    fn movement(&self) -> &Vector {
        &self.borrow().movement
    }

    fn radius(&self) -> f32 {
        self.borrow().radius
    }

    fn gravity(&self) -> f32 {
        self.borrow().gravity
    }

    fn team(&self) -> &Option<Arc<RwLock<Team>>> {
        &self.borrow().team
    }

    fn solid(&self) -> bool {
        self.borrow().solid
    }

    fn masking(&self) -> bool {
        self.borrow().masking
    }

    fn visible(&self) -> bool {
        self.borrow().visible
    }

    fn orbiting(&self) -> bool {
        self.borrow().orbiting
    }

    fn orbiting_center(&self) -> &Option<Vector> {
        &self.borrow().orbiting_center
    }

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>> {
        self.borrow().connector.register_task_quitely_if_unknown(Task::UsedOrbits);
        &self.borrow().orbiting_state
    }

    fn mobility(&self) -> Mobility {
        self.borrow().mobility
    }

    fn connector(&self) -> &Arc<Connector> {
        &self.borrow().connector
    }

    fn kind(&self) -> UnitKind {
        UnitKind::Unknown
    }
}