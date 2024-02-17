use crate::network::PacketWriter;
use crate::unit::Ship;

#[derive(Debug, Clone, Default)]
pub struct ShipConfig {
    name: String,
    cost_energy: f64,
    cost_ion: f64,
    cost_iron: f64,
    cost_tungsten: f64,
    cost_silicon: f64,
    cost_tritium: f64,
    cost_time: f64,
    hull: f64,
    hull_repair: f64,
    shields: f64,
    shields_repair: f64,
    size: f64,
    weight: f64,
    energy_max: f64,
    energy_cells: f64,
    energy_reactor: f64,
    energy_transfer: f64,
    ion_max: f64,
    ion_cells: f64,
    ion_reactor: f64,
    ion_transfer: f64,
    thruster: f64,
    nozzle: f64,
    speed: f64,
    turnrate: f64,
    cargo: f64,
    extractor: f64,
    weapon_speed: f64,
    weapon_time: f64,
    weapon_load: f64,
    free_spawn: bool,
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
