
use Error;
use Player;
use Connector;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;

use unit::ControllableInfo;
use unit::any_unit::prelude::*;

use controllable::SubDirection;

pub struct Shot {
    unit:   UnitData,
    player: Weak<Player>,
    info:   Weak<ControllableInfo>,
    originator_kind:    UnitKind,
    originator_name:    String,
    time:               u16,
    load:               f32,
    hull:               f32,
    hull_max:           f32,
    hull_armor:         f32,
    shield:             f32,
    shield_max:         f32,
    shield_armor:       f32,
    damage_hull:                f32,
    damage_hull_crit:           f32,
    damage_hull_crit_chance:    f32,
    damage_shield:              f32,
    damage_shield_crit:         f32,
    damage_shield_crit_chance:  f32,
    damage_energy:              f32,
    damage_energy_crit:         f32,
    damage_energy_crit_chance:  f32,
    sub_directions:             Vec<SubDirection>,
}

impl Shot {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Shot, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader)?;
        let kind;
        let name;
        let player;
        let info : Weak<ControllableInfo>;

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
                let i_strong = p_strong.controllable_info(id).ok_or(Error::InvalidControllableInfo(id))?;
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

        Ok(Shot {
            unit,
            originator_kind: kind,
            originator_name: name,
            player,
            info,
            time:           reader.read_u16()?,
            load:           reader.read_single()?,
            hull:           reader.read_single()?,
            hull_max:       reader.read_single()?,
            hull_armor:     reader.read_single()?,
            shield:         reader.read_single()?,
            shield_max:     reader.read_single()?,
            shield_armor:   reader.read_single()?,
            damage_hull:                reader.read_single()?,
            damage_hull_crit:           reader.read_single()?,
            damage_hull_crit_chance:    reader.read_single()?,
            damage_shield:              reader.read_single()?,
            damage_shield_crit:         reader.read_single()?,
            damage_shield_crit_chance:  reader.read_single()?,
            damage_energy:              reader.read_single()?,
            damage_energy_crit:         reader.read_single()?,
            damage_energy_crit_chance:  reader.read_single()?,
            sub_directions: {
                let count = reader.read_unsigned_byte()?;
                let mut vec = Vec::new();
                for _ in 0..count {
                    vec.push(SubDirection::from_reader(reader)?);
                }
                vec
            },
        })
    }

    /// The [Player] who fired the shot
    pub fn player(&self) -> &Weak<Player> {
        &self.player
    }

    /// The [ControllableInfo] the fired the shot
    pub fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.info
    }

    /// The [UnitKind] that fired the shot
    pub fn originator_kind(&self) -> UnitKind {
        self.originator_kind
    }

    /// The name of the [ControllableInfo] that fired the shot
    pub fn originator_name(&self) -> &str {
        &self.originator_name
    }

    pub fn time(&self) -> u16 {
        self.time
    }

    pub fn load(&self) -> f32 {
        self.load
    }

    pub fn hull(&self) -> f32 {
        self.hull
    }

    pub fn hull_max(&self) -> f32 {
        self.hull_max
    }

    pub fn hull_armor(&self) -> f32 {
        self.hull_armor
    }

    pub fn shield(&self) -> f32 {
        self.shield
    }

    pub fn shield_max(&self) -> f32 {
        self.shield_max
    }

    pub fn shield_armor(&self) -> f32 {
        self.shield_armor
    }

    pub fn damage_hull(&self) -> f32 {
        self.damage_hull
    }

    pub fn damage_hull_crit(&self) -> f32 {
        self.damage_hull_crit
    }

    pub fn damage_hull_crit_chance(&self) -> f32 {
        self.damage_hull_crit_chance
    }

    pub fn damage_shield(&self) -> f32 {
        self.damage_shield
    }

    pub fn damage_shield_crit(&self) -> f32 {
        self.damage_shield_crit
    }

    pub fn damage_shield_crit_chance(&self) -> f32 {
        self.damage_shield_crit_chance
    }

    pub fn damage_energy(&self) -> f32 {
        self.damage_energy
    }

    pub fn damage_energy_crit(&self) -> f32 {
        self.damage_energy_crit
    }

    pub fn damage_energy_crit_chance(&self) -> f32 {
        self.damage_energy_crit_chance
    }

    pub fn sub_directions(&self) -> &Vec<SubDirection> {
        &self.sub_directions
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Shot {
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
        UnitKind::Shot
    }
}