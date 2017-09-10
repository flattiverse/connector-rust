
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
use net::Packet;
use net::BinaryReader;

impl_downcast!(Explosion);
pub trait Explosion : Unit {

    /// The cause of the explosion
    fn player(&self) -> &Weak<RwLock<Player>>;

    /// The [ControllableInfo] that caused the explosion
    fn controllable_info(&self) -> &Weak<RwLock<ControllableInfo>>;

    /// The [UnitKind] that caused the explosion
    fn originator_kind(&self) -> UnitKind;

    /// The name of the [ControllableInfo] that caused the explosion
    fn originator_name(&self) -> &str;

    /// The hull damage dealt by the explosion
    fn damage_hull(&self) -> f32;

    /// The additional damage dealt by the explosion
    fn damage_hull_crit(&self) -> f32;

    /// The shield damage dealt by the explosion
    fn damage_shield(&self) -> f32;

    // The additional damage dealt by the explosion
    fn damage_shield_crit(&self) -> f32;

    /// The energy damage dealt by the explosion
    fn damage_energy(&self) -> f32;

    // The additional damage dealt by the explosion
    fn damage_energy_crit(&self) -> f32;

    fn kind(&self) -> UnitKind {
        UnitKind::Explosion
    }
}

pub struct ExplosionData {
    unit:   UnitData,
    player: Weak<RwLock<Player>>,
    info:   Weak<RwLock<ControllableInfo>>,
    originator_kind:    UnitKind,
    originator_name:    String,
    damage_hull:        f32,
    damage_hull_crit:   f32,
    damage_shield:      f32,
    damage_shield_crit: f32,
    damage_energy:      f32,
    damage_energy_crit: f32,
}

impl ExplosionData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<ExplosionData, Error> {
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


        Ok(ExplosionData {
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
}


// implicitly implement Unit
impl Borrow<UnitData> for ExplosionData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for ExplosionData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<ExplosionData> + BorrowMut<ExplosionData> + Unit> Explosion for  T {
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

    fn damage_hull(&self) -> f32 {
        self.borrow().damage_hull
    }

    fn damage_hull_crit(&self) -> f32 {
        self.borrow().damage_hull_crit
    }

    fn damage_shield(&self) -> f32 {
        self.borrow().damage_shield
    }

    fn damage_shield_crit(&self) -> f32 {
        self.borrow().damage_shield_crit
    }

    fn damage_energy(&self) -> f32 {
        self.borrow().damage_energy
    }

    fn damage_energy_crit(&self) -> f32 {
        self.borrow().damage_energy_crit
    }
}