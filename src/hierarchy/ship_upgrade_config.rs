use crate::hierarchy::ShipUpgradeId;
use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_64;
use crate::GameError;

#[derive(Debug, Clone, Default)]
pub struct ShipUpgradeConfig {
    pub name: String,
    /// The id of the previous [`crate::Upgrade`], which can be found on the orresponding
    /// [`crate::unit::ShipDesign`] of this [`crate::Upgrade`].
    pub previous_upgrade: Option<ShipUpgradeId>,
    pub cost_energy: f64,
    pub cost_ion: f64,
    pub cost_iron: f64,
    pub cost_tungsten: f64,
    pub cost_silicon: f64,
    pub cost_tritium: f64,
    pub cost_time: f64,
    pub hull: f64,
    pub hull_repair: f64,
    pub shields: f64,
    pub shields_repair: f64,
    pub radius: f64,
    pub gravity: f64,
    pub energy_max: f64,
    pub energy_cells: f64,
    pub energy_reactor: f64,
    pub energy_transfer: f64,
    pub ion_max: f64,
    pub ion_cells: f64,
    pub ion_reactor: f64,
    pub ion_transfer: f64,
    pub thruster: f64,
    pub nozzle: f64,
    pub speed: f64,
    pub turnrate: f64,
    pub cargo: f64,
    pub extractor: f64,
    pub weapon_speed: f64,
    pub weapon_time: f64,
    pub weapon_load: f64,
    pub weapon_ammo: f64,
    pub weapon_ammo_production: f64,
    pub free_spawn: bool,
    pub nozzle_energy_consumption: f64,
    pub thruster_energy_consumption: f64,
    pub hull_repair_energy_consumption: f64,
    pub hull_repair_iron_consumption: f64,
    pub shields_ion_consumption: f64,
    pub extractor_energy_consumption: f64,
    pub weapon_energy_consumption: f64,
    pub scanner_energy_consumption: f64,
    pub scanner_range: f64,
    pub scanner_width: f64,
}

impl From<&mut dyn PacketReader> for ShipUpgradeConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
        this.read(reader);
        this
    }
}

impl ShipUpgradeConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.previous_upgrade = reader.read_nullable_byte().map(ShipUpgradeId);
        self.cost_energy = reader.read_double();
        self.cost_ion = reader.read_double();
        self.cost_iron = reader.read_double();
        self.cost_tungsten = reader.read_double();
        self.cost_silicon = reader.read_double();
        self.cost_tritium = reader.read_double();
        self.cost_time = reader.read_uint16() as f64;
        self.hull = reader.read_double();
        self.hull_repair = reader.read_double();
        self.shields = reader.read_double();
        self.shields_repair = reader.read_double();
        self.radius = reader.read_double();
        self.gravity = reader.read_double();
        self.energy_max = reader.read_double();
        self.energy_cells = reader.read_double();
        self.energy_reactor = reader.read_double();
        self.energy_transfer = reader.read_double();
        self.ion_max = reader.read_double();
        self.ion_cells = reader.read_double();
        self.ion_reactor = reader.read_double();
        self.ion_transfer = reader.read_double();
        self.thruster = reader.read_double();
        self.nozzle = reader.read_double();
        self.speed = reader.read_double();
        self.turnrate = reader.read_double();
        self.cargo = reader.read_double();
        self.extractor = reader.read_double();
        self.weapon_speed = reader.read_double();
        self.weapon_time = reader.read_uint16() as f64;
        self.weapon_load = reader.read_double();
        self.weapon_ammo = reader.read_double();
        self.weapon_ammo_production = reader.read_double();
        self.free_spawn = reader.read_boolean();
        self.nozzle_energy_consumption = reader.read_double();
        self.thruster_energy_consumption = reader.read_double();
        self.hull_repair_energy_consumption = reader.read_double();
        self.hull_repair_iron_consumption = reader.read_double();
        self.shields_ion_consumption = reader.read_double();
        self.extractor_energy_consumption = reader.read_double();
        self.weapon_energy_consumption = reader.read_double();
        self.scanner_energy_consumption = reader.read_double();
        self.scanner_range = reader.read_double();
        self.scanner_width = reader.read_double();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_nullable_byte(self.previous_upgrade.map(|id| id.0));
        writer.write_double(self.cost_energy);
        writer.write_double(self.cost_ion);
        writer.write_double(self.cost_iron);
        writer.write_double(self.cost_tungsten);
        writer.write_double(self.cost_silicon);
        writer.write_double(self.cost_tritium);
        writer.write_double(self.cost_time);
        writer.write_double(self.hull);
        writer.write_double(self.hull_repair);
        writer.write_double(self.shields);
        writer.write_double(self.shields_repair);
        writer.write_double(self.radius);
        writer.write_double(self.gravity);
        writer.write_double(self.energy_max);
        writer.write_double(self.energy_cells);
        writer.write_double(self.energy_reactor);
        writer.write_double(self.energy_transfer);
        writer.write_double(self.ion_max);
        writer.write_double(self.ion_cells);
        writer.write_double(self.ion_reactor);
        writer.write_double(self.ion_transfer);
        writer.write_double(self.thruster);
        writer.write_double(self.nozzle);
        writer.write_double(self.speed);
        writer.write_double(self.turnrate);
        writer.write_double(self.cargo);
        writer.write_double(self.extractor);
        writer.write_double(self.weapon_speed);
        writer.write_uint16(self.weapon_time as _);
        writer.write_double(self.weapon_load);
        writer.write_double(self.weapon_ammo);
        writer.write_double(self.weapon_ammo_production);
        writer.write_boolean(self.free_spawn);
        writer.write_double(self.nozzle_energy_consumption);
        writer.write_double(self.thruster_energy_consumption);
        writer.write_double(self.hull_repair_energy_consumption);
        writer.write_double(self.hull_repair_iron_consumption);
        writer.write_double(self.shields_ion_consumption);
        writer.write_double(self.extractor_energy_consumption);
        writer.write_double(self.weapon_energy_consumption);
        writer.write_double(self.scanner_energy_consumption);
        writer.write_double(self.scanner_range);
        writer.write_double(self.scanner_width);
    }

    /// The name of the configured [`crate::hierarchy::ShipUpgrade`].
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), GameError> {
        let name = name.into();
        self.name = check_name_or_err_64(name)?;
        Ok(())
    }

    #[inline]
    pub fn name_valid(&self) -> bool {
        check_name_or_err_64(&self.name).is_ok()
    }
}
