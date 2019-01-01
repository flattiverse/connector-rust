
use crate::Error;
use crate::Player;
use crate::Connector;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::ControllableInfo;
use crate::unit::any_unit::prelude::*;

pub struct Explosion {
    unit:   UnitData,
    player: Weak<Player>,
    info:   Weak<ControllableInfo>,
    originator_kind:    UnitKind,
    originator_name:    String,
    damage_hull:        f32,
    damage_hull_crit:   f32,
    damage_shield:      f32,
    damage_shield_crit: f32,
    damage_energy:      f32,
    damage_energy_crit: f32,
}

impl Explosion {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Explosion, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader)?;
        let kind;
        let name;
        let player;
        let info;

        match reader.read_unsigned_byte()? {
            1 => {
                kind    = UnitKind::from_id(reader.read_unsigned_byte()?);
                name    = reader.read_string()?;
                player  = Weak::default();
                info    = Weak::new();
            },
            2 => {
                let p_strong = connector.player_for(reader.read_u16()?)?;
                player  = Arc::downgrade(&p_strong);
                let id  = reader.read_unsigned_byte()?;
                let i_strong = p_strong.controllable_info(id).ok_or_else(|| Error::InvalidControllableInfo(id))?;
                info = Arc::downgrade(&i_strong);
                kind    = i_strong.kind();
                name    = String::from(i_strong.name());
            },
            _ => {
                kind    = UnitKind::Unknown;
                name    = String::new();
                player  = Weak::default();
                info    = Weak::new();
            },
        }


        Ok(Explosion {
            unit,
            originator_kind: kind,
            originator_name: name,
            player,
            info,
            damage_hull:        reader.read_single()?,
            damage_hull_crit:   reader.read_single()?,
            damage_shield:      reader.read_single()?,
            damage_shield_crit: reader.read_single()?,
            damage_energy:      reader.read_single()?,
            damage_energy_crit: reader.read_single()?,
        })
    }

    /// The cause of the explosion
    pub fn player(&self) -> &Weak<Player> {
        &self.player
    }

    /// The [ControllableInfo] that caused the explosion
    pub fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.info
    }

    /// The [UnitKind] that caused the explosion
    pub fn originator_kind(&self) -> UnitKind {
        self.originator_kind
    }

    /// The name of the [ControllableInfo] that caused the explosion
    pub fn originator_name(&self) -> &str {
        &self.originator_name
    }

    /// The hull damage dealt by the explosion
    pub fn damage_hull(&self) -> f32 {
        self.damage_hull
    }

    /// The additional damage dealt by the explosion
    pub fn damage_hull_crit(&self) -> f32 {
        self.damage_hull_crit
    }

    /// The shield damage dealt by the explosion
    pub fn damage_shield(&self) -> f32 {
        self.damage_shield
    }

    // The additional damage dealt by the explosion
    pub fn damage_shield_crit(&self) -> f32 {
        self.damage_shield_crit
    }

    /// The energy damage dealt by the explosion
    pub fn damage_energy(&self) -> f32 {
        self.damage_energy
    }

    // The additional damage dealt by the explosion
    pub fn damage_energy_crit(&self) -> f32 {
        self.damage_energy_crit
    }
}


// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Explosion {
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
        UnitKind::Explosion
    }
}