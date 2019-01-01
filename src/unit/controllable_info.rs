
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use crate::Error;
use crate::Player;
use crate::Scores;
use crate::UniversalEnumerable;

use crate::unit::UnitKind;

use crate::item::AnyCargoItem;
use crate::item::CrystalCargoItem;

use crate::net::Packet;
use crate::net::BinaryReader;
use crate::net::is_set_u8;

struct ControllableInfoMut {
    hull:           f32,
    shield:         f32,

    build_progress:     f32,
    pending_shutdown:   bool,


    is_building:    Weak<ControllableInfo>,
    is_built_by:    Weak<ControllableInfo>,

    has_power_up_haste:         bool,
    has_power_up_double_damage: bool,
    has_power_up_quad_damage:   bool,
    has_power_up_cloak:         bool,
    active:                     bool,
}

pub struct ControllableInfo {
    id: u8,
    revision: i64,
    class: String,
    name: String,
    level: i32,

    efficiency_tactical  : f32,
    efficiency_economical: f32,
    hull_max:     f32,
    hull_armor:   f32,
    shield_max:   f32,
    shield_armor: f32,
    radius:       f32,
    cargo_slots:   u8,
    crystal_slots: u8,
    has_tractor_beam: bool,

    kind:    UnitKind,
    player:  Weak<Player>,
    scores:  Arc<Scores>,
    mutable: RwLock<ControllableInfoMut>,

    items:    RwLock<Vec<AnyCargoItem>>,
    crystals: RwLock<Vec<Arc<CrystalCargoItem>>>,
}

impl ControllableInfo {
    pub fn from_packet(packet: &Packet, player: Weak<Player>) -> Result<ControllableInfo, Error> {
        let kind = match packet.path_sub() {
            0x00 => UnitKind::PlayerPlatform,
            0x01 => UnitKind::PlayerProbe,
            0x02 => UnitKind::PlayerDrone,
            0x03 => UnitKind::PlayerShip,
            0x04 => UnitKind::PlayerBase,
            id@_ => return Err(Error::InvalidControllableInfo(id))
        };

        Self::new(kind, &packet, player)
    }

    pub fn new(kind: UnitKind, packet: &Packet, player: Weak<Player>) -> Result<ControllableInfo, Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        Ok(ControllableInfo {
            id:                     packet.path_sub(),
            revision:               reader.read_i64()?,
            class:                  reader.read_string()?,
            name:                   reader.read_string()?,
            level:                  reader.read_unsigned_byte()? as i32,
            efficiency_tactical:    reader.read_single()?,
            efficiency_economical:  reader.read_single()?,
            hull_max:               reader.read_single()?,
            hull_armor:             reader.read_single()?,
            shield_max:             reader.read_single()?,
            shield_armor:           reader.read_single()?,
            radius:                 reader.read_single()?,
            cargo_slots:            reader.read_unsigned_byte()?,
            crystal_slots:          reader.read_unsigned_byte()?,
            has_tractor_beam:       reader.read_bool()?,

            kind,
            player,
            scores:                 Arc::new(Scores::default()),
            crystals:               RwLock::new(Vec::new()),
            items:                  RwLock::new(Vec::new()),
            mutable: RwLock::new(ControllableInfoMut {
                hull:                   0f32,
                shield:                 0f32,
                build_progress:         0f32,
                is_building:            Weak::new(),
                is_built_by:            Weak::new(),
                active:                 false,
                pending_shutdown:       false,
                has_power_up_haste:         false,
                has_power_up_double_damage: false,
                has_power_up_quad_damage:   false,
                has_power_up_cloak:         false,
            })
        })
    }

    pub(crate) fn update(&self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        let mut mutable = self.mutable.write()?;

        mutable.hull               = reader.read_single()?;
        mutable.shield             = reader.read_single()?;
        mutable.pending_shutdown   = reader.read_bool()?;

        let header = reader.read_byte()?;

        if !is_set_u8(header, 0x03) {
            mutable.build_progress = 0f32;
        }

        if is_set_u8(header, 0x01) {
            let player = self.player.upgrade().unwrap();

            mutable.build_progress = reader.read_single()?;
            mutable.is_building    = match player.controllable_info(reader.read_unsigned_byte()?) {
                Some(ref arc) => Arc::downgrade(arc),
                None          => Weak::new()
            }

        } else {
            mutable.is_building    = Weak::new();
        }

        if is_set_u8(header, 0x02) {
            let player = self.player.upgrade().unwrap();

            mutable.build_progress = reader.read_single()?;
            mutable.is_built_by    = match player.controllable_info(reader.read_unsigned_byte()?) {
                Some(ref arc) => Arc::downgrade(arc),
                None          => Weak::new()
            }
        } else {
            mutable.is_built_by    = Weak::new();
        }


        mutable.has_power_up_haste         = is_set_u8(header, 0x10);
        mutable.has_power_up_double_damage = is_set_u8(header, 0x20);
        mutable.has_power_up_quad_damage   = is_set_u8(header, 0x40);
        mutable.has_power_up_cloak         = is_set_u8(header, 0x80);

        Ok(())
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn active(&self) -> bool {
        self.mutable.read().unwrap().active
    }

    pub(crate) fn set_active(&self, active: bool) -> Result<(), Error> {
        self.mutable.write()?.active = active;
        Ok(())
    }

    pub fn alive(&self) -> bool {
        self.hull() > 0f32
    }

    /// Whether this [ControllableInfo] is
    /// building another [ControllableInfo]
    pub fn building(&self) -> bool {
        self.mutable.read().unwrap().is_building.upgrade().is_some()
    }

    /// Whether this [ControllableInfo] is
    /// currently built by another [ControllableInfo]
    pub fn built(&self) -> bool {
        self.mutable.read().unwrap().is_built_by.upgrade().is_some()
    }

    /// The [ControllableInfo] currently built
    /// by this [ControllableInfo]
    pub fn build_target(&self) -> Weak<ControllableInfo> {
        self.mutable.read().unwrap().is_building.clone()
    }

    /// The [ControllableInfo] currently
    /// building this [ControllableInfo]
    pub fn built_by(&self) -> Weak<ControllableInfo> {
        self.mutable.read().unwrap().is_built_by.clone()
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn revision(&self) -> i64 {
        self.revision
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn efficiency_tactical(&self) -> f32 {
        self.efficiency_tactical
    }

    pub fn efficiency_economical(&self) -> f32 {
        self.efficiency_economical
    }

    pub fn hull_max(&self) -> f32 {
        self.hull_max
    }

    pub fn hull_armor(&self) -> f32 {
        self.hull_armor
    }

    pub fn shield_max(&self) -> f32 {
        self.shield_max
    }

    pub fn shield_armor(&self) -> f32 {
        self.shield_armor
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn cargo_slots(&self) -> u8 {
        self.cargo_slots
    }

    pub fn crystal_slots(&self) -> u8 {
        self.crystal_slots
    }

    pub fn has_tractor_beam(&self) -> bool {
        self.has_tractor_beam
    }

    pub fn scores(&self) -> &Arc<Scores> {
        &self.scores
    }

    pub fn has_pending_shutdown(&self) -> bool {
        self.mutable.read().unwrap().pending_shutdown
    }

    pub fn has_power_up_haste(&self) -> bool {
        self.mutable.read().unwrap().has_power_up_haste
    }

    pub fn has_power_up_double_damage(&self) -> bool {
        self.mutable.read().unwrap().has_power_up_double_damage
    }

    pub fn has_power_up_quad_damage(&self) -> bool {
        self.mutable.read().unwrap().has_power_up_quad_damage
    }

    pub fn has_power_up_cloak(&self) -> bool {
        self.mutable.read().unwrap().has_power_up_cloak
    }

    pub fn build_progress(&self) -> f32 {
        self.mutable.read().unwrap().build_progress
    }

    pub fn hull(&self) -> f32 {
        self.mutable.read().unwrap().hull
    }

    pub fn shield(&self) -> f32 {
        self.mutable.read().unwrap().shield
    }

    pub fn kind(&self) -> UnitKind {
        self.kind
    }

    pub fn crystals(&self) -> RwLockReadGuard<Vec<Arc<CrystalCargoItem>>> {
        self.crystals.read().unwrap()
    }

    pub(crate) fn set_crystals(&self, crystals: Vec<Arc<CrystalCargoItem>>) -> Result<(), Error> {
        *self.crystals.write()? = crystals;
        Ok(())
    }

    pub fn cargo_items(&self) -> RwLockReadGuard<Vec<AnyCargoItem>> {
        self.items.read().unwrap()
    }

    pub(crate) fn set_cargo_items(&self, items: Vec<AnyCargoItem>) -> Result<(), Error> {
        *self.items.write()? = items;
        Ok(())
    }
}

impl UniversalEnumerable for ControllableInfo {
    fn name(&self) -> &str {
        &self.name
    }
}