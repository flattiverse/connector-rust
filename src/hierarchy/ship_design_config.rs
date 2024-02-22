use crate::network::{PacketReader, PacketWriter};

#[derive(Debug, Clone, Default)]
pub struct ShipDesignConfig {
    pub name: String,
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
    pub shields_load: f64,
    pub size: f64,
    pub weight: f64,
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
    pub weapon_ammo: u16,
    pub weapon_ammo_production: f64,
    pub free_spawn: bool,
}

impl From<&mut dyn PacketReader> for ShipDesignConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
        this.read(reader);
        this
    }
}

impl ShipDesignConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.cost_energy = reader.read_4u(1_000.0);
        self.cost_ion = reader.read_4u(1_000.0);
        self.cost_iron = reader.read_4u(1_000.0);
        self.cost_tungsten = reader.read_4u(1_000.0);
        self.cost_silicon = reader.read_4u(1_000.0);
        self.cost_tritium = reader.read_4u(1_000.0);
        self.cost_time = reader.read_2u(10.0);
        self.hull = reader.read_3u(10_000.0);
        self.hull_repair = reader.read_3u(10_000.0);
        self.shields = reader.read_3u(10_000.0);
        self.shields_load = reader.read_3u(10_000.0);
        self.size = reader.read_3u(1_000.0);
        self.weight = reader.read_2s(10_000.0);
        self.energy_max = reader.read_4u(1_000.0);
        self.energy_cells = reader.read_4u(1_000.0);
        self.energy_reactor = reader.read_4u(1_000.0);
        self.energy_transfer = reader.read_4u(1_000.0);
        self.ion_max = reader.read_4u(1_000.0);
        self.ion_cells = reader.read_4u(1_000.0);
        self.ion_reactor = reader.read_4u(1_000.0);
        self.ion_transfer = reader.read_4u(1_000.0);
        self.thruster = reader.read_2u(10_000.0);
        self.nozzle = reader.read_2u(100.0);
        self.speed = reader.read_2u(1_000.0);
        self.turnrate = reader.read_2u(100.0);
        self.cargo = reader.read_4u(1_000.0);
        self.extractor = reader.read_4u(1_000.0);
        self.weapon_speed = reader.read_2u(1_000.0);
        self.weapon_time = reader.read_uint16() as f64;
        self.weapon_load = reader.read_3u(1_000.0);
        self.weapon_ammo = reader.read_uint16();
        self.weapon_ammo_production = reader.read_4u(1_000.0);
        self.free_spawn = reader.read_boolean();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_4u(self.cost_energy, 1_000.0);
        writer.write_4u(self.cost_ion, 1_000.0);
        writer.write_4u(self.cost_iron, 1_000.0);
        writer.write_4u(self.cost_tungsten, 1_000.0);
        writer.write_4u(self.cost_silicon, 1_000.0);
        writer.write_4u(self.cost_tritium, 1_000.0);
        writer.write_2u(self.cost_time, 10.0);
        writer.write_3u(self.hull, 10_000.0);
        writer.write_3u(self.hull_repair, 10_000.0);
        writer.write_3u(self.shields, 1_000.0);
        writer.write_3u(self.shields_load, 10_000.0);
        writer.write_3u(self.size, 1_000.0);
        writer.write_2s(self.weight, 10_000.0);
        writer.write_4u(self.energy_max, 1_000.0);
        writer.write_4u(self.energy_cells, 1_000.0);
        writer.write_4u(self.energy_reactor, 1_000.0);
        writer.write_4u(self.energy_transfer, 1_000.0);
        writer.write_4u(self.ion_max, 1_000.0);
        writer.write_4u(self.ion_cells, 1_000.0);
        writer.write_4u(self.ion_reactor, 1_000.0);
        writer.write_4u(self.ion_transfer, 1_000.0);
        writer.write_2u(self.thruster, 10_000.0);
        writer.write_2u(self.nozzle, 100.0);
        writer.write_2u(self.speed, 1_000.0);
        writer.write_2u(self.turnrate, 100.0);
        writer.write_4u(self.cargo, 1_000.0);
        writer.write_4u(self.extractor, 1_000.0);
        writer.write_2u(self.weapon_speed, 1_000.0);
        writer.write_uint16(self.weapon_time as u16);
        writer.write_3u(self.weapon_load, 1_000.0);
        writer.write_uint16(self.weapon_ammo);
        writer.write_4u(self.weapon_ammo_production, 1_000.0);
        writer.write_boolean(self.free_spawn);
    }
}
