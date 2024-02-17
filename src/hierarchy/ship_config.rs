use crate::network::PacketWriter;
use crate::unit::Ship;

#[derive(Debug, Clone, Default)]
pub struct ShipConfig {
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
    pub shields_repair: f64,
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
    pub free_spawn: bool,
}

impl From<&Ship> for ShipConfig {
    fn from(ship: &Ship) -> Self {
        Self {
            name: ship.name().to_string(),
            cost_energy: ship.cost_energy(),
            cost_ion: ship.cost_ion(),
            cost_iron: ship.cost_iron(),
            cost_tungsten: ship.cost_tungsten(),
            cost_silicon: ship.cost_silicon(),
            cost_tritium: ship.cost_tritium(),
            cost_time: ship.cost_time(),
            hull: ship.hull(),
            hull_repair: ship.hull_repair(),
            shields: ship.shields(),
            shields_repair: ship.shields_repair(),
            size: ship.size(),
            weight: ship.weight(),
            energy_max: ship.energy_max(),
            energy_cells: ship.energy_cells(),
            energy_reactor: ship.energy_reactor(),
            energy_transfer: ship.energy_transfer(),
            ion_max: ship.ion_max(),
            ion_cells: ship.ion_cells(),
            ion_reactor: ship.ion_reactor(),
            ion_transfer: ship.ion_transfer(),
            thruster: ship.thruster(),
            nozzle: ship.nozzle(),
            speed: ship.speed(),
            turnrate: ship.turnrate(),
            cargo: ship.cargo(),
            extractor: ship.extractor(),
            weapon_speed: ship.weapon_speed(),
            weapon_time: ship.weapon_time(),
            weapon_load: ship.weapon_load(),
            free_spawn: ship.free_spawn(),
        }
    }
}

impl ShipConfig {
    pub(crate) fn write_to(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_2u(self.cost_energy, 1.0);
        writer.write_2u(self.cost_ion, 100.0);
        writer.write_2u(self.cost_iron, 1.0);
        writer.write_2u(self.cost_tungsten, 100.0);
        writer.write_2u(self.cost_silicon, 1.0);
        writer.write_2u(self.cost_tritium, 10.0);
        writer.write_2u(self.cost_time, 10.0);
        writer.write_2u(self.hull, 10.0);
        writer.write_2u(self.hull_repair, 100.0);
        writer.write_2u(self.shields, 10.0);
        writer.write_2u(self.shields_repair, 100.0);
        writer.write_2u(self.size, 10.0);
        writer.write_2s(self.weight, 10000.0);
        writer.write_2u(self.energy_max, 10.0);
        writer.write_4u(self.energy_cells, 100.0);
        writer.write_2u(self.energy_reactor, 100.0);
        writer.write_2u(self.energy_transfer, 100.0);
        writer.write_2u(self.ion_max, 100.0);
        writer.write_2u(self.ion_cells, 100.0);
        writer.write_2u(self.ion_reactor, 1000.0);
        writer.write_2u(self.ion_transfer, 1000.0);
        writer.write_2u(self.thruster, 10000.0);
        writer.write_2u(self.nozzle, 100.0);
        writer.write_2u(self.speed, 100.0);
        writer.write_2u(self.turnrate, 100.0);
        writer.write_4u(self.cargo, 1000.0);
        writer.write_2u(self.extractor, 100.0);
        writer.write_2u(self.weapon_speed, 10.0);
        writer.write_uint16(self.weapon_time as _);
        writer.write_2u(self.weapon_load, 10.0);
        writer.write_boolean(self.free_spawn);
    }
}
