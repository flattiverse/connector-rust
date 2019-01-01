
use std::sync::Arc;

use crate::Color;
use crate::Error;
use crate::Connector;
use crate::UniversalEnumerable;
use crate::item::CargoItem;
use crate::item::CrystalKind;
use crate::item::CargoItemData;
use crate::item::CargoItemKind;
use crate::net::Packet;
use crate::net::BinaryReader;
use crate::net::BinaryWriter;

pub struct CrystalCargoItem {
    cargo:  CargoItemData,
    color:  Color,
    name:   String,
    kind:   CrystalKind,
    level:  i32,

    energy_critical_strike_chance_offset:   f32,
    shield_critical_strike_chance_offset:   f32,
    hull_critical_strike_chance_offset:     f32,
    energy_critical_strike_damage_offset:   f32,
    shield_critical_strike_damage_offset:   f32,
    hull_critical_strike_damage_offset:     f32,

    shots_refresh_rate_offset:  f32,
    shots_maximum_offset:       f32,

    maximum_speed_offset:       f32,
    visible_range_multiplier:   f32,
    weight_multiplier:          f32,
    energy_offset:              f32,
    particles_offset:           f32,
    ions_offset:                f32,
    hull_offset:                f32,
    shield_offset:              f32,
    hull_armor_offset:          f32,
    shield_armor_offset:        f32,

    special_produced_energy:        f32,
    special_produced_particles:     f32,
    special_produced_ions:          f32,
    special_autoregenerating_hull:  f32,
    special_autoregenerating_shield:f32,
}

impl CrystalCargoItem {
    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<CrystalCargoItem, Error> {

        let mut data = CrystalCargoItem {
            cargo: CargoItemData::new(connector, reader, master)?,
            kind:  CrystalKind::from_id(reader.read_byte()?)?,
            name:  reader.read_string()?,
            color: Color::new_transparent(),

            // default values
            level: 0i32,
            energy_critical_strike_chance_offset: 0f32,
            shield_critical_strike_chance_offset: 0f32,
            hull_critical_strike_chance_offset: 0f32,
            energy_critical_strike_damage_offset: 0f32,
            shield_critical_strike_damage_offset: 0f32,
            hull_critical_strike_damage_offset: 0f32,
            shots_refresh_rate_offset: 0f32,
            shots_maximum_offset: 0f32,
            maximum_speed_offset: 0f32,
            visible_range_multiplier: 0f32,
            weight_multiplier: 0f32,
            energy_offset: 0f32,
            particles_offset: 0f32,
            ions_offset: 0f32,
            hull_offset: 0f32,
            shield_offset: 0f32,
            hull_armor_offset: 0f32,
            shield_armor_offset: 0f32,
            special_produced_energy: 0f32,
            special_produced_particles: 0f32,
            special_produced_ions: 0f32,
            special_autoregenerating_hull: 0f32,
            special_autoregenerating_shield: 0f32,
        };

        let hue;

        if data.kind == CrystalKind::Special {
            hue                                         = ::std::f32::NAN;
            data.level                                  = 1337;
            data.special_produced_energy                = reader.read_single()?;
            data.special_produced_particles             = reader.read_single()?;
            data.special_produced_ions                  = reader.read_single()?;
            data.special_autoregenerating_hull          = reader.read_single()?;
            data.special_autoregenerating_shield        = reader.read_single()?;

        } else {
            hue                                         = reader.read_single()?;
            data.level                                  = reader.read_byte()? as i32;
            data.energy_critical_strike_chance_offset   = reader.read_single()?;
            data.shield_critical_strike_chance_offset   = reader.read_single()?;
            data.hull_critical_strike_chance_offset     = reader.read_single()?;
            data.energy_critical_strike_damage_offset   = reader.read_single()?;
            data.shield_critical_strike_damage_offset   = reader.read_single()?;
            data.hull_critical_strike_damage_offset     = reader.read_single()?;
        }

        data.shots_refresh_rate_offset  = reader.read_single()?;
        data.shots_maximum_offset       = reader.read_single()?;

        data.maximum_speed_offset       = reader.read_single()?;
        data.visible_range_multiplier   = reader.read_single()?;
        data.weight_multiplier          = reader.read_single()?;
        data.energy_offset              = reader.read_single()?;
        data.particles_offset           = reader.read_single()?;
        data.ions_offset                = reader.read_single()?;

        data.hull_offset                = reader.read_single()?;
        data.shield_offset              = reader.read_single()?;
        data.hull_armor_offset          = reader.read_single()?;
        data.shield_armor_offset        = reader.read_single()?;

        if !hue.is_nan() {
            data.color = Color::from_hue(hue)?;
        }

        Ok(data)
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn red(&self) -> f32 {
        self.color.red
    }

    fn green(&self) -> f32 {
        self.color.green
    }

    fn blue(&self) -> f32 {
        self.color.blue
    }

    fn alpha(&self) -> f32 {
        self.color.alpha
    }

    fn crystal_kind(&self) -> CrystalKind {
        self.kind
    }

    fn level(&self) -> i32 {
        self.level
    }

    /// Chance offset for a critical strike on enemies energy system
    pub fn energy_critical_strike_chance_offset(&self) -> f32 {
        self.energy_critical_strike_chance_offset
    }

    /// Chance offset for a critical strike on enemies shield
    pub fn shield_critical_strike_chance_offset(&self) -> f32 {
        self.shield_critical_strike_chance_offset
    }

    /// Chance offset for a critical strike on enemies ship hull
    pub fn hull_critical_strike_chance_offset(&self) -> f32 {
        self.hull_critical_strike_chance_offset
    }

    /// Damage offset for a critical strike on enemies energy system
    pub fn energy_critical_strike_damage_offset(&self) -> f32 {
        self.energy_critical_strike_damage_offset
    }

    /// Damage offset for a critical strike on enemies shields
    pub fn shield_critical_strike_damage_offset(&self) -> f32 {
        self.shield_critical_strike_damage_offset
    }

    /// Damage offset for a critical strike on enemies ship hull
    pub fn hull_critical_strike_damage_offset(&self) -> f32 {
        self.hull_critical_strike_damage_offset
    }

    /// Offset on shot refresh rate
    pub fn shots_refresh_rate_offset(&self) -> f32 {
        self.shots_refresh_rate_offset
    }

    /// Offset on maximum shots
    pub fn shots_maximum_offset(&self) -> f32 {
        self.shots_maximum_offset
    }

    /// Offset on maximum speed
    pub fn maximum_speed_offset(&self) -> f32 {
        self.maximum_speed_offset
    }

    /// multiplier of visible range
    pub fn visible_range_multiplier(&self) -> f32 {
        self.visible_range_multiplier
    }

    /// weight multiplier
    pub fn weight_multiplier(&self) -> f32 {
        self.weight_multiplier
    }

    /// maximum energy offset
    pub fn energy_offset(&self) -> f32 {
        self.energy_offset
    }

    /// maximum particles offset
    pub fn particles_offset(&self) -> f32 {
        self.particles_offset
    }

    /// maximum ions offset
    pub fn ions_offset(&self) -> f32 {
        self.ions_offset
    }

    /// maximum hull offset
    pub fn hull_offset(&self) -> f32 {
        self.hull_offset
    }

    /// maximum shield offset
    pub fn shield_offset(&self) -> f32 {
        self.shield_offset
    }

    /// hull armor offset
    pub fn hull_armor_offset(&self) -> f32 {
        self.hull_armor_offset
    }

    /// shield armor offset
    pub fn shield_armor_offset(&self) -> f32 {
        self.shield_armor_offset
    }

    /// A vessel equipped with this crystal will automatically produce
    /// energy. This is a special feature offered only by special crystals.
    pub fn special_produced_energy(&self) -> f32 {
        self.special_produced_energy
    }

    /// A vessel equipped with this crystal will automatically produce
    /// particles. This is a special feature offered only by special crystals.
    pub fn special_produced_particles(&self) -> f32 {
        self.special_produced_particles
    }

    /// A vessel equipped with this crystal will automatically produce
    /// ions. This is a special feature offered only by special crystals.
    pub fn special_produced_ions(&self) -> f32 {
        self.special_produced_ions
    }

    /// A vessel equipped with this crystal will automatically repair
    /// itself. This is a special feature offered only by special crystals.
    pub fn special_autoregenerating_hull(&self) -> f32 {
        self.special_autoregenerating_hull
    }

    /// A vessel equipped with this crystal will automatically self-load
    /// its shield. This is a special feature offered only by special crystals.
    pub fn special_autoregenerating_shield(&self) -> f32 {
        self.special_autoregenerating_shield
    }

    /// Renames this crystal
    pub fn rename(&mut self, new_name: &str) -> Result<(), Error> {
        if self.kind == CrystalKind::Special {
            return Err(Error::CannotRenameCrystalKind(self.kind));
        }

        if !self.cargo.master {
            return Err(Error::YouCanOnlyRenameCrystalsNotInUse(self.name.clone()));
        }

        // lock account queries for the rest of this function
        let connector = self.cargo.connector.upgrade().unwrap();
        let _ = connector.sync_account_queries().lock().unwrap();
        let mut block = connector.block_manager().block()?;
        let mut packet = Packet::new();

        packet.set_command(0x71_u8);
        packet.set_session(block.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(new_name)?;
        }

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }


    /// Destroys this crystal. The crystal must not be in use.
    /// After this action, the crystal will be deleted from the
    /// players' account.
    pub fn destroy(&mut self) -> Result<(), Error> {
        if !self.cargo.master {
            return Err(Error::YouAreNotTheCrystalMaster(self.name.clone()));
        }

        // lock account queries for the rest of this function
        let connector = self.cargo.connector.upgrade().unwrap();
        let _ = connector.sync_account_queries().lock().unwrap();
        let mut block = connector.block_manager().block()?;

        let mut packet = Packet::new();

        packet.set_command(0x70_u8);
        packet.set_session(block.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(&self.name)?;
        }

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }
}

impl CargoItem for CrystalCargoItem {
    fn weight(&self) -> f32 {
        self.cargo.weight()
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::Crystal
    }
}

impl UniversalEnumerable for CrystalCargoItem {
    fn name(&self) -> &str {
        &self.name
    }
}