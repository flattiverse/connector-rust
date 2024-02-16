use crate::network::PacketReader;
use crate::unit::Upgrade;

#[derive(Debug)]
pub struct Ship {
    pub galaxy: i32,
    pub id: u8,
    upgrades: [Option<Upgrade>; 256],
    upgrade_max: usize,
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
}

impl Ship {
    pub fn new(id: u8, galaxy: i32, reader: &mut dyn PacketReader) -> Self {
        Self {
            id,
            galaxy,
            upgrades: core::array::from_fn(|_| None),
            upgrade_max: 0,
            name: reader.read_string(),
            cost_energy: reader.read_4s(2.0),
            cost_ion: reader.read_4s(2.0),
            cost_iron: reader.read_4s(2.0),
            cost_tungsten: reader.read_4s(2.0),
            cost_silicon: reader.read_4s(2.0),
            cost_tritium: reader.read_4s(2.0),
            cost_time: reader.read_4s(2.0),
            hull: reader.read_4s(2.0),
            hull_repair: reader.read_4s(2.0),
            shields: reader.read_4s(2.0),
            shields_repair: reader.read_4s(2.0),
            size: reader.read_4s(2.0),
            weight: reader.read_4s(2.0),
            energy_max: reader.read_4s(2.0),
            energy_cells: reader.read_4s(2.0),
            energy_reactor: reader.read_4s(2.0),
            energy_transfer: reader.read_4s(2.0),
            ion_max: reader.read_4s(2.0),
            ion_cells: reader.read_4s(2.0),
            ion_reactor: reader.read_4s(2.0),
            ion_transfer: reader.read_4s(2.0),
            thruster: reader.read_4s(2.0),
            nozzle: reader.read_4s(2.0),
            speed: reader.read_4s(2.0),
            turnrate: reader.read_4s(2.0),
            cargo: reader.read_4s(2.0),
            extractor: reader.read_4s(2.0),
            weapon_speed: reader.read_4s(2.0),
            weapon_time: reader.read_4s(2.0),
            weapon_load: reader.read_4s(2.0),
        }
    }

    pub(crate) fn read_upgrade(&mut self, id: u8, reader: &mut dyn PacketReader) {
        let index = usize::from(id);
        self.upgrades[index] = Some(Upgrade::new(id, self.galaxy, self.id, reader));
        if self.upgrade_max < index + 1 {
            self.upgrade_max = index + 1;
        }
    }

    #[inline]
    pub fn get_upgrade(&self, id: u8) -> Option<&Upgrade> {
        self.upgrades[usize::from(id)].as_ref()
    }
}
