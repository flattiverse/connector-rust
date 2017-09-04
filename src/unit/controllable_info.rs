
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Error;
use Player;
use Scores;
use UniversalEnumerable;

use unit::UnitKind;

use item::CrystalCargoItem;

use net::Packet;
use net::BinaryReader;
use net::is_set_u8;

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

    crystals: Vec<Weak<RwLock<Box<CrystalCargoItem>>>>,
    scores:   Scores,

    hull:           f32,
    shield:         f32,
    build_progress: f32,
    is_building:    Option<Weak<RwLock<Box<ControllableInfo>>>>,
    is_built_by:    Option<Weak<RwLock<Box<ControllableInfo>>>>,

    active:             bool,
    pending_shutdown:   bool,
    player:             Weak<RwLock<Player>>,

    has_power_up_haste:         bool,
    has_power_up_double_damage: bool,
    has_power_up_quad_damage:   bool,
    has_power_up_cloak:         bool,

    kind: UnitKind

}

impl ControllableInfo {
    pub fn from_packet(packet: &Packet, player: Weak<RwLock<Player>>) -> Result<ControllableInfo, Error> {
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

    pub fn new(kind: UnitKind, packet: &Packet, player: Weak<RwLock<Player>>) -> Result<ControllableInfo, Error> {
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

            crystals:               Vec::new(),
            scores:                 Scores::default(),
            hull:                   0f32,
            shield:                 0f32,
            build_progress:         0f32,
            is_building:            None,
            is_built_by:            None,
            active:                 false,
            pending_shutdown:       false,
            player:                 player,
            has_power_up_haste:         false,
            has_power_up_double_damage: false,
            has_power_up_quad_damage:   false,
            has_power_up_cloak:         false,
            kind:                   kind
        })
    }

    pub(crate) fn update(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;

        self.hull               = reader.read_single()?;
        self.shield             = reader.read_single()?;
        self.pending_shutdown   = reader.read_bool()?;

        let header = reader.read_byte()?;

        if !is_set_u8(header, 0x03) {
            self.build_progress = 0f32;
        }

        if is_set_u8(header, 0x01) {
            let player = self.player.upgrade().unwrap();
            let player = player.read().unwrap();

            self.build_progress = reader.read_single()?;
            self.is_building    = match player.controllable_info(reader.read_unsigned_byte()?) {
                Some(ref arc) => Some(Arc::downgrade(arc)),
                None => None
            }

        } else {
            self.is_building    = None;
        }

        if is_set_u8(header, 0x02) {
            let player = self.player.upgrade().unwrap();
            let player = player.read().unwrap();

            self.build_progress = reader.read_single()?;
            self.is_built_by    = match player.controllable_info(reader.read_unsigned_byte()?) {
                Some(ref arc) => Some(Arc::downgrade(arc)),
                None => None
            }
        } else {
            self.is_built_by    = None;
        }


        self.has_power_up_haste         = is_set_u8(header, 0x10);
        self.has_power_up_double_damage = is_set_u8(header, 0x20);
        self.has_power_up_quad_damage   = is_set_u8(header, 0x40);
        self.has_power_up_cloak         = is_set_u8(header, 0x80);

        Ok(())
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub(crate) fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn alive(&self) -> bool {
        self.hull > 0f32
    }

    /// Whether this [ControllableInfo] is
    /// building another [ControllableInfo]
    pub fn building(&self) -> bool {
        self.is_building.is_some()
    }

    /// Whether this [ControllableInfo] is
    /// currently built by another [ControllableInfo]
    pub fn built(&self) -> bool {
        self.is_built_by.is_some()
    }

    /// The [ControllableInfo] currently built
    /// by this [ControllableInfo]
    pub fn build_target(&self) -> &Option<Weak<RwLock<Box<ControllableInfo>>>> {
        &self.is_building
    }

    /// The [ControllableInfo] currently
    /// building this [ControllableInfo]
    pub fn built_by(&self) -> &Option<Weak<RwLock<Box<ControllableInfo>>>> {
        &self.is_built_by
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

    pub fn scores(&self) -> &Scores {
        &self.scores
    }

    pub fn hash_pending_shutdown(&self) -> bool {
        self.pending_shutdown
    }

    pub fn has_power_up_haste(&self) -> bool {
        self.has_power_up_haste
    }

    pub fn has_power_up_double_damage(&self) -> bool {
        self.has_power_up_double_damage
    }

    pub fn has_power_up_quad_damage(&self) -> bool {
        self.has_power_up_quad_damage
    }

    pub fn has_power_up_cloak(&self) -> bool {
        self.has_power_up_cloak
    }

    pub fn build_progress(&self) -> f32 {
        self.build_progress
    }

    pub fn hull(&self) -> f32 {
        self.hull
    }

    pub fn shield(&self) -> f32 {
        self.shield
    }

    pub fn kind(&self) -> UnitKind {
        self.kind
    }

    pub fn crystals(&self) -> &Vec<Weak<RwLock<Box<CrystalCargoItem>>>> {
        &self.crystals
    }

    pub(crate) fn set_crystals(&mut self, crystals: Vec<Weak<RwLock<Box<CrystalCargoItem>>>>) {
        self.crystals = crystals;
    }
}

impl UniversalEnumerable for ControllableInfo {
    fn name(&self) -> &str {
        &self.name
    }
}