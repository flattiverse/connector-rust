
use Task;
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;
use net::is_set_u8;

use unit::any_unit::prelude::*;

pub trait Unit : Send + Sync {
    fn name(&self) -> &str;

    fn position(&self) -> &Vector;

    fn movement(&self) -> &Vector;

    fn radius(&self) -> f32;

    fn gravity(&self) -> f32;

    fn team(&self) -> &Weak<Team>;

    fn is_solid(&self) -> bool;

    fn is_masking(&self) -> bool;

    fn is_visible(&self) -> bool;

    fn is_orbiting(&self) -> bool;

    fn orbiting_center(&self) -> &Option<Vector>;

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>>;

    fn mobility(&self) -> Mobility;

    fn connector(&self) -> &Weak<Connector>;

    fn kind(&self) -> UnitKind;
}


pub(crate) struct UnitData {
    name: String,
    position: Vector,
    movement: Vector,
    radius: f32,
    gravity: f32,
    team: Weak<Team>,
    solid: bool,
    masking: bool,
    visible: bool,
    orbiting: bool,
    orbiting_center: Option<Vector>,
    orbiting_state: Option<Vec<OrbitingState>>,
    mobility: Mobility,
    connector: Weak<Connector>,
}

impl UnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<UnitData, Error> {
        let team = universe_group.team_weak(packet.path_sub());

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
            connector: Arc::downgrade(connector),
        })
    }

    pub fn new(connector: &Arc<Connector>, _: &UniverseGroup, name: String, radius: f32,
               gravity: f32, position: Vector, movement: Vector, solid: bool, masking: bool,
               visible: bool, mobility: Mobility) -> UnitData {
        UnitData {
            connector: Arc::downgrade(connector),
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
            team: Weak::default(),
            orbiting: false,
            orbiting_center: None,
            orbiting_state:  None,
        }
    }
}

impl Unit for UnitData {
    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> &Vector {
        &self.position
    }

    fn movement(&self) -> &Vector {
        &self.movement
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn gravity(&self) -> f32 {
        self.gravity
    }

    fn team(&self) -> &Weak<Team> {
        &self.team
    }

    fn is_solid(&self) -> bool {
        self.solid
    }

    fn is_masking(&self) -> bool {
        self.masking
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn is_orbiting(&self) -> bool {
        self.orbiting
    }

    fn orbiting_center(&self) -> &Option<Vector> {
        &self.orbiting_center
    }

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>> {
        match self.connector.upgrade() {
            None => println!("Connector reference invalid"),
            Some(ref arc) => arc.register_task_quitely_if_unknown(Task::UsedOrbits),
        };
        &self.orbiting_state
    }

    fn mobility(&self) -> Mobility {
        self.mobility
    }

    fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    fn kind(&self) -> UnitKind {
        unimplemented!("Missing override!")
    }
}