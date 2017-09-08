
use std::sync::Weak;

use Color;
use Error;
use Connector;
use UniversalEnumerable;
use item::CargoItem;
use item::CrystalKind;
use item::CargoItemData;
use item::CargoItemKind;
use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;

impl_downcast!(CrystalCargoItem);
pub trait CrystalCargoItem : CargoItem + UniversalEnumerable {
    fn color(&self) -> &Color;

    fn red(&self) -> f32 {
        self.color().red()
    }

    fn green(&self) -> f32 {
        self.color().green()
    }

    fn blue(&self) -> f32 {
        self.color().blue()
    }

    fn alpha(&self) -> f32 {
        self.color().alpha()
    }

    fn crystal_kind(&self) -> CrystalKind;

    fn level(&self) -> i32;

    /// Chance offset for a critical strike on enemies energy system
    fn energy_critical_strike_chance_offset(&self) -> f32;

    /// Chance offset for a critical strike on enemies shield
    fn shield_critical_strike_chance_offset(&self) -> f32;

    /// Chance offset for a critical strike on enemies ship hull
    fn hull_critical_strike_chance_offset(&self) -> f32;

    /// Damage offset for a critical strike on enemies energy system
    fn energy_critical_strike_damage_offset(&self) -> f32;

    /// Damage offset for a critical strike on enemies shields
    fn shield_critical_strike_damage_offset(&self) -> f32;

    /// Damage offset for a critical strike on enemies ship hull
    fn hull_critical_strike_damage_offset(&self) -> f32;

    /// Offset on shot refresh rate
    fn shots_refresh_rate_offset(&self) -> f32;

    /// Offset on maximum shots
    fn shots_maximum_offset(&self) -> f32;

    /// Offset on maximum speed
    fn maximum_speed_offset(&self) -> f32;

    /// multiplier of visible range
    fn visible_range_multiplier(&self) -> f32;

    /// weight multiplier
    fn weight_multiplier(&self) -> f32;

    /// maximum energy offset
    fn energy_offset(&self) -> f32;

    /// maximum particles offset
    fn particles_offset(&self) -> f32;

    /// maximum ions offset
    fn ions_offset(&self) -> f32;

    /// maximum hull offset
    fn hull_offset(&self) -> f32;

    /// maximum shield offset
    fn shield_offset(&self) -> f32;

    /// hull armor offset
    fn hull_armor_offset(&self) -> f32;

    /// shield armor offset
    fn shield_armor_offset(&self) -> f32;

    /// A vessel equipped with this crystal will automatically produce
    /// energy. This is a special feature offered only by special crystals.
    fn special_produced_energy(&self) -> f32;

    /// A vessel equipped with this crystal will automatically produce
    /// particles. This is a special feature offered only by special crystals.
    fn special_produced_particles(&self) -> f32;

    /// A vessel equipped with this crystal will automatically produce
    /// ions. This is a special feature offered only by special crystals.
    fn special_produced_ions(&self) -> f32;

    /// A vessel equipped with this crystal will automatically repair
    /// itself. This is a special feature offered only by special crystals.
    fn special_autoregenerating_hull(&self) -> f32;

    /// A vessel equipped with this crystal will automatically self-load
    /// its shield. This is a special feature offered only by special crystals.
    fn special_autoregenerating_shield(&self) -> f32;

    /// Renames this crystal
    fn rename(&mut self, new_name: &str) -> Result<(), Error>;

    /// Destroys this crystal. The crystal must not be in use.
    /// After this action, the crystal will be deleted from the
    /// players' account.
    fn destroy(&mut self) -> Result<(), Error>;
}

pub(crate) struct CrystalCargoItemData  {
    pub(crate) cargo_item_data: CargoItemData,
    pub(crate) color:           Color,
    pub(crate) name:            String,
    pub(crate) crystal_kind:    CrystalKind,
    pub(crate) level:           i32,

    pub(crate) energy_critical_strike_chance_offset : f32,
    pub(crate) shield_critical_strike_chance_offset : f32,
    pub(crate) hull_critical_strike_chance_offset   : f32,
    pub(crate) energy_critical_strike_damage_offset : f32,
    pub(crate) shield_critical_strike_damage_offset : f32,
    pub(crate) hull_critical_strike_damage_offset   : f32,

    pub(crate) shots_refresh_rate_offset            : f32,
    pub(crate) shots_maximum_offset                 : f32,

    pub(crate) maximum_speed_offset                 : f32,
    pub(crate) visible_range_multiplier             : f32,
    pub(crate) weight_multiplier                    : f32,
    pub(crate) energy_offset                        : f32,
    pub(crate) particles_offset                     : f32,
    pub(crate) ions_offset                          : f32,
    pub(crate) hull_offset                          : f32,
    pub(crate) shield_offset                        : f32,
    pub(crate) hull_armor_offset                    : f32,
    pub(crate) shield_armor_offset                  : f32,
    pub(crate) special_produced_energy              : f32,
    pub(crate) special_produced_particles           : f32,
    pub(crate) special_produced_ions                : f32,
    pub(crate) special_autoregenerating_hull        : f32,
    pub(crate) special_autoregenerating_shield      : f32,
}

impl CrystalCargoItemData {
    pub(crate) fn new(connector: Weak<Connector>, master: bool, reader: &mut BinaryReader) -> Result<CrystalCargoItemData, Error> {

        let mut data = CrystalCargoItemData {
            cargo_item_data: CargoItemData::new(connector, master, reader)?,
            crystal_kind:    CrystalKind::from_id(reader.read_byte()?)?,
            name:            reader.read_string()?,
            color:           Color::new_transparent(),

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

        if data.crystal_kind == CrystalKind::Special {
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
}

impl CargoItem for CrystalCargoItemData {
    fn weight(&self) -> f32 {
        self.cargo_item_data.weight
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::Crystal
    }
}

impl CrystalCargoItem for CrystalCargoItemData {
    fn color(&self) -> &Color {
        &self.color
    }

    fn crystal_kind(&self) -> CrystalKind {
        self.crystal_kind
    }

    fn level(&self) -> i32 {
        self.level
    }

    fn energy_critical_strike_chance_offset(&self) -> f32 {
        self.energy_critical_strike_chance_offset
    }

    fn shield_critical_strike_chance_offset(&self) -> f32 {
        self.shield_critical_strike_chance_offset
    }

    fn hull_critical_strike_chance_offset(&self) -> f32 {
        self.hull_critical_strike_chance_offset
    }

    fn energy_critical_strike_damage_offset(&self) -> f32 {
        self.energy_critical_strike_damage_offset
    }

    fn shield_critical_strike_damage_offset(&self) -> f32 {
        self.shield_critical_strike_damage_offset
    }

    fn hull_critical_strike_damage_offset(&self) -> f32 {
        self.hull_critical_strike_damage_offset
    }

    fn shots_refresh_rate_offset(&self) -> f32 {
        self.shots_refresh_rate_offset
    }

    fn shots_maximum_offset(&self) -> f32 {
        self.shots_maximum_offset
    }

    fn maximum_speed_offset(&self) -> f32 {
        self.maximum_speed_offset
    }

    fn visible_range_multiplier(&self) -> f32 {
        self.visible_range_multiplier
    }

    fn weight_multiplier(&self) -> f32 {
        self.weight_multiplier
    }

    fn energy_offset(&self) -> f32 {
        self.energy_offset
    }

    fn particles_offset(&self) -> f32 {
        self.particles_offset
    }

    fn ions_offset(&self) -> f32 {
        self.ions_offset
    }

    fn hull_offset(&self) -> f32 {
        self.hull_offset
    }

    fn shield_offset(&self) -> f32 {
        self.shield_offset
    }

    fn hull_armor_offset(&self) -> f32 {
        self.hull_armor_offset
    }

    fn shield_armor_offset(&self) -> f32 {
        self.shield_armor_offset
    }

    fn special_produced_energy(&self) -> f32 {
        self.special_produced_energy
    }

    fn special_produced_particles(&self) -> f32 {
        self.special_produced_particles
    }

    fn special_produced_ions(&self) -> f32 {
        self.special_produced_ions
    }

    fn special_autoregenerating_hull(&self) -> f32 {
        self.special_autoregenerating_hull
    }

    fn special_autoregenerating_shield(&self) -> f32 {
        self.special_autoregenerating_shield
    }

    fn rename(&mut self, new_name: &str) -> Result<(), Error> {
        if self.crystal_kind == CrystalKind::Special {
            return Err(Error::CannotRenameCrystalKind(self.crystal_kind));
        }

        if !self.cargo_item_data.master {
            return Err(Error::YouCanOnlyRenameCrystalsNotInUse(self.name.clone()));
        }

        // lock account queries for the rest of this function
        let connector = self.cargo_item_data.connector.upgrade().unwrap();
        let _ = connector.sync_account_queries().lock().unwrap();
        let manager = connector.block_manager().block()?;
        let mut block = manager.lock().unwrap();

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

    fn destroy(&mut self) -> Result<(), Error> {
        if !self.cargo_item_data.master {
            return Err(Error::YouAreNotTheCrystalMaster(self.name.clone()));
        }

        // lock account queries for the rest of this function
        let connector = self.cargo_item_data.connector.upgrade().unwrap();
        let _ = connector.sync_account_queries().lock().unwrap();
        let manager = connector.block_manager().block()?;
        let mut block = manager.lock().unwrap();

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

impl UniversalEnumerable for CrystalCargoItemData {
    fn name(&self) -> &str {
        &self.name
    }
}