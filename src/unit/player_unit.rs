
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_player_unit::prelude::*;

pub trait PlayerUnit : Unit {

    fn player(&self) -> &Weak<Player>;

    fn controllable_info(&self) -> &Weak<ControllableInfo>;

    fn tractorbeam_info(&self) -> &Option<PlayerUnitTractorbeamInfo>;
}

pub(crate) struct PlayerUnitData {
    unit:   UnitData,
    player: Weak<Player>,
    c_info: Weak<ControllableInfo>,
    b_info: Option<PlayerUnitTractorbeamInfo>,
}

impl PlayerUnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitData, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitData {
            unit,
            player: Arc::downgrade(&player),
            c_info: {
                let id = reader.read_unsigned_byte()?;
                let info = player.controllable_info(id).ok_or(Error::InvalidControllableInfo(id))?;
                Arc::downgrade(&info)
            },
            b_info:   {
                if reader.read_byte()? == 1 {
                    Some(PlayerUnitTractorbeamInfo::for_reader(reader)?)
                } else {
                    None
                }
            },
        })
    }
}

impl PlayerUnit for PlayerUnitData {
    fn player(&self) -> &Weak<Player> {
        &self.player
    }

    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.c_info
    }

    fn tractorbeam_info(&self) -> &Option<PlayerUnitTractorbeamInfo> {
        &self.b_info
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for PlayerUnitData {
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