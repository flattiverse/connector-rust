
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use unit::ControllableInfo;
use controllable::SubDirection;

use net::Packet;
use net::BinaryReader;

impl_downcast!(Shot);
pub trait Shot : Unit {

    /// The [Player] who fired the shot
    fn player(&self) -> &Weak<RwLock<Player>>;

    /// The [ControllableInfo] the fired the shot
    fn controllable_info(&self) -> &Weak<RwLock<ControllableInfo>>;

    /// The [UnitKind] that fired the shot
    fn originator_kind(&self) -> UnitKind;

    /// The name of the [ControllableInfo] that fired the shot
    fn originator_name(&self) -> &str;

    fn time(&self) -> u16;

    fn load(&self) -> f32;

    fn hull(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;

    fn shield(&self) -> f32;

    fn shield_max(&self) -> f32;

    fn shield_armor(&self) -> f32;

    fn damage_hull(&self) -> f32;

    fn damage_hull_crit(&self) -> f32;

    fn damage_hull_crit_chance(&self) -> f32;

    fn damage_shield(&self) -> f32;

    fn damage_shield_crit(&self) -> f32;

    fn damage_shield_crit_chance(&self) -> f32;

    fn damage_energy(&self) -> f32;

    fn damage_energy_crit(&self) -> f32;

    fn damage_energy_crit_chance(&self) -> f32;

    fn sub_directions(&self) -> &Vec<SubDirection>;

    fn kind(&self) -> UnitKind {
        UnitKind::Shot
    }
}

pub struct ShotData {
    unit:   UnitData,
    player: Weak<RwLock<Player>>,
    info:   Weak<RwLock<ControllableInfo>>,
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

impl ShotData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<ShotData, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader)?;
        let kind;
        let name;
        let player;
        let info : Weak<RwLock<ControllableInfo>>;

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
                let i_strong = p_strong.read()?.controllable_info(id).ok_or(Error::InvalidControllableInfo(id))?;
                info = Arc::downgrade(&i_strong);
                let i_read = i_strong.read()?;
                kind    = i_read.kind();
                name    = String::from(i_read.name());
            },
            _ => {
                kind    = UnitKind::Unknown;
                name    = String::new();
                player  = Weak::default();
                info    = Weak::new();
            },
        }

        Ok(ShotData {
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
}


// implicitly implement Unit
impl Borrow<UnitData> for ShotData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for ShotData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<ShotData> + BorrowMut<ShotData> + Unit> Shot for  T {
    fn player(&self) -> &Weak<RwLock<Player>> {
        &self.borrow().player
    }

    fn controllable_info(&self) -> &Weak<RwLock<ControllableInfo>> {
        &self.borrow().info
    }

    fn originator_kind(&self) -> UnitKind {
        self.borrow().originator_kind
    }

    fn originator_name(&self) -> &str {
        &self.borrow().originator_name
    }

    fn time(&self) -> u16 {
        self.borrow().time
    }

    fn load(&self) -> f32 {
        self.borrow().load
    }

    fn hull(&self) -> f32 {
        self.borrow().hull
    }

    fn hull_max(&self) -> f32 {
        self.borrow().hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.borrow().hull_armor
    }

    fn shield(&self) -> f32 {
        self.borrow().shield
    }

    fn shield_max(&self) -> f32 {
        self.borrow().shield_max
    }

    fn shield_armor(&self) -> f32 {
        self.borrow().shield_armor
    }

    fn damage_hull(&self) -> f32 {
        self.borrow().damage_hull
    }

    fn damage_hull_crit(&self) -> f32 {
        self.borrow().damage_hull_crit
    }

    fn damage_hull_crit_chance(&self) -> f32 {
        self.borrow().damage_hull_crit_chance
    }

    fn damage_shield(&self) -> f32 {
        self.borrow().damage_shield
    }

    fn damage_shield_crit(&self) -> f32 {
        self.borrow().damage_shield_crit
    }

    fn damage_shield_crit_chance(&self) -> f32 {
        self.borrow().damage_shield_crit_chance
    }

    fn damage_energy(&self) -> f32 {
        self.borrow().damage_energy
    }

    fn damage_energy_crit(&self) -> f32 {
        self.borrow().damage_energy_crit
    }

    fn damage_energy_crit_chance(&self) -> f32 {
        self.borrow().damage_energy_crit_chance
    }

    fn sub_directions(&self) -> &Vec<SubDirection> {
        &self.borrow().sub_directions
    }
}