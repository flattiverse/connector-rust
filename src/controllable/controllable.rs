
use std::mem;

use std::fmt;
use std::fmt::Display;

use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Downcast;


use Error;
use Scores;
use Vector;
use Universe;
use Connector;
use UniversalEnumerable;

use unit::Unit;
use unit::UnitKind;
use unit::ScanInfo;

use item::CargoItem;
use item::CrystalCargoItem;

use controllable::EnergyCost;
use controllable::ScanEnergyCost;
use controllable::WeaponEnergyCost;
use controllable::SubDirection;


use net::Packet;
use net::BinaryReader;
use net::BinaryWriter;
use net::is_set_u8;

impl_downcast!(Controllable);
pub trait Controllable : Downcast {

    fn id(&self) -> u8;

    fn revision(&self) -> i64;

    fn class(&self) -> &str;

    fn name(&self) -> &str;

    /// The level of the best component
    fn level(&self) -> u8;

    fn radius(&self) -> f32;

    fn gravity(&self) -> f32;

    fn efficiency_tactical(&self) -> f32;

    fn efficiency_economical(&self) -> f32;

    fn visible_range_multiplier(&self) -> f32;

    fn energy_max(&self) -> f32;

    fn particles_max(&self) -> f32;

    fn ions_max(&self) -> f32;

    fn energy_cells(&self) -> f32;

    fn particles_cells(&self) -> f32;

    fn ions_cells(&self) -> f32;

    fn energy_reactor(&self) -> f32;

    fn particles_reactor(&self) -> f32;

    fn ions_reactor(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;

    fn hull_repair(&self) -> &EnergyCost;

    fn shield_max(&self) -> f32;

    fn shield_armor(&self) -> f32;

    fn shield_load(&self) -> &EnergyCost;

    fn engine_speed(&self) -> f32;

    fn engine_acceleration(&self) -> &EnergyCost;

    fn scanner_degree_per_scan(&self) -> f32;

    fn scanner_count(&self) -> u8;

    fn scanner_area(&self) -> &ScanEnergyCost;

    fn weapon_shot(&self) -> &WeaponEnergyCost;

    fn weapon_hull(&self) -> f32;

    fn weapon_hull_armor(&self) -> f32;

    fn weapon_shield(&self) -> f32;

    fn weapon_shield_armor(&self) -> f32;

    fn weapon_visible_range_multiplier(&self) -> f32;

    fn weapon_gravity(&self) -> f32;

    fn weapon_size(&self) -> f32;

    fn weapon_production(&self) -> f32;

    fn weapon_production_load(&self) -> f32;

    fn weapon_sub_directions(&self) -> u8;

    fn weapon_sub_directions_length(&self) -> f32;

    fn builder_time(&self) -> f32;

    fn builder_time_factory_energy(&self) -> f32;

    fn builder_time_factory_particles(&self) -> f32;

    fn builder_time_factory_ions(&self) -> f32;

    fn builder_capabilities(&self) -> &Vec<UnitKind>;

    fn energy_transfer_energy(&self) -> &EnergyCost;

    fn energy_transfer_particles(&self) -> &EnergyCost;

    fn energy_transfer_ions(&self) -> &EnergyCost;

    fn cargo_slots(&self) -> u8;

    fn cargo_amount(&self) -> f32;

    fn crystal_converter(&self) -> &EnergyCost;

    fn crystal_slots(&self) -> u8;

    fn tractor_beam(&self) -> &EnergyCost;

    fn tractor_beam_range(&self) -> f32;

    fn scores(&self) -> &Arc<Scores>;

    fn energy(&self) -> f32;

    fn particles(&self) -> f32;

    fn ions(&self) -> f32;

    fn hull(&self) -> f32;

    fn shield(&self) -> f32;

    fn build_position(&self) -> &Option<Vector>;

    fn build_progress(&self) -> f32;

    fn is_building(&self) -> &Option<Weak<RwLock<Controllable>>>;

    fn is_built_by(&self) -> &Option<Weak<RwLock<Controllable>>>;

    fn weapon_production_status(&self) -> f32;

    fn crystals(&self) -> Arc<Vec<Box<CrystalCargoItem>>>;

    fn set_crystals(&self, crystals: Arc<Vec<Box<CrystalCargoItem>>>);

    fn cargo_items(&self) -> Arc<Vec<Box<CargoItem>>>;

    fn set_cargo_items(&self, items: Arc<Vec<Box<CargoItem>>>);

    fn universe(&self) -> &Weak<RwLock<Universe>>;

    fn haste_time(&self) -> u16;

    fn double_damage_time(&self) -> u16;

    fn quad_damage_time(&self) -> u16;

    fn cloak_time(&self) -> u16;

    fn connector(&self) -> &Weak<Connector>;

    fn active(&self) -> bool;

    fn pending_shutdown(&self) -> bool;

    fn close(&self) -> Result<(), Error> {
        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector,
        };

        let group = match connector.player().upgrade() {
            None => return Err(Error::PlayerNotAvailable),
            Some(player) => {
                match player.read()?.universe_group().upgrade() {
                    None => return Err(Error::PlayerNotInUniverseGroup),
                    Some(group) => group
                }
            }
        };

        let block = connector.block_manager().block()?;
        let mut packet = Packet::new();

        {
            let block = block.lock()?;
            packet.set_command(0x88);
            packet.set_session(block.id());
            packet.set_path_sub(self.id());
        }

        connector.send(&packet)?;
        block.lock()?.wait()?;
        Ok(())
    }

    fn scan_list(&self) -> &RwLock<Vec<Box<Unit>>>;

    fn scan_area(&self, degree: f32, range: f32) -> Result<Vec<Box<Unit>>, Error> {
        self.scan_areas(&[ScanInfo::new(
            degree - (self.scanner_degree_per_scan() / 2f32),
            degree + (self.scanner_degree_per_scan() / 2f32),
            range
        )?])
    }

    fn scan_areas(&self, info: &[ScanInfo]) -> Result<Vec<Box<Unit>>, Error> {
        if info.len() > self.scanner_count() as usize {
            return Err(Error::ScanRequestExceedsScannerCount {
                got: info.len() as u8,
                max: self.scanner_count()
            });
        }

        let mut packet = Packet::new();
        packet.set_command(0x90);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_u8(info.len() as u8)?;
            for &i in info.iter() {
                i.write(writer)?;
            }
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector,
        };

        // TODO no scan sync - any issues with that?
        let block = connector.block_manager().block()?;

        {
            let mut block = block.lock()?;
            packet.set_session(block.id());

            connector.send(&packet);
            block.wait()?;

            let mut vec = Vec::new(); // replacement list
            mem::swap(&mut *self.scan_list().write()?, &mut vec);

            Ok(vec)
        }
    }

    fn build(&self, class: &str, name: &str, direction: f32, crystals: &[Box<CrystalCargoItem>]) -> Result<Arc<RwLock<Controllable>>, Error> {
        if !Connector::check_name(class) {
            return Err(Error::InvalidName);
        }

        if !"Ship".eq(class) && !Connector::check_name(class) {
            return Err(Error::InvalidClass);
        }

        if !direction.is_finite() {
            return Err(Error::InvalidDirection);
        }

        let mut packet = Packet::new();

        packet.set_command(0x85);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(class)?;
            writer.write_string(name)?;
            writer.write_f32(direction)?;

            if crystals.len() == 0 {
                writer.write_byte(0x00)?;

            } else {
                writer.write_u8(crystals.len() as u8)?;
                for crystal in crystals {
                    writer.write_string(crystal.name())?;
                }
            }
        }


        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector,
        };

        let id = {
            let block = connector.block_manager().block()?;
            let mut block = block.lock()?;

            packet.set_session(block.id());
            connector.send(&packet);
            block.wait()?.path_ship()
        };

        match connector.controllable(id) {
            None => Err(Error::InvalidControllable(id)),
            Some(arc) => Ok(arc)
        }
    }

    fn kill(&self) -> Result<(), Error> {
        let mut packet = Packet::new();

        packet.set_command(0x82);
        packet.set_path_ship(self.id());

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        Ok(())
    }

    fn repair_hull(&self, hull: f32) -> Result<(), Error> {
        let mut packet = Packet::new();

        packet.set_command(0x83);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_f32(hull)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        Ok(())
    }

    fn load_shields(&self, amount: f32) -> Result<(), Error> {
        let mut packet = Packet::new();

        packet.set_command(0x84);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_f32(amount)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        Ok(())
    }

    fn harvest_nebula(&self, amount: f32) -> Result<(), Error> {
        let mut packet = Packet::new();

        packet.set_command(0x86);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_f32(amount)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        Ok(())
    }

    fn flush_cargo(&self) -> Result<(), Error> {
        let mut packet = Packet::new();

        packet.set_command(0x87);
        packet.set_path_ship(self.id());

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        Ok(())
    }

    fn produce_crystal(&self, name: &str) -> Result<Arc<CrystalCargoItem>, Error> {
        if !Connector::check_name(name) {
            return Err(Error::InvalidName);
        }

        let mut packet = Packet::new();
        packet.set_command(0x89);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(name)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        block.wait()?;
        match connector.crystals(name) {
            None => Err(Error::InvalidCrystalName(name.to_string())),
            Some(arc) => Ok(arc)
        }
    }

    fn shoot_full_load(&self, direction: &Vector, time: u16) -> Result<String, Error> {
        self.shoot(
            direction,
            direction.angle(),
            time,
            self.weapon_shot().load().limit(),
            self.weapon_shot().damage_hull().limit(),
            0f32,
            0f32,
            &[]
        )
    }

    fn shoot_with_load(&self, direction: &Vector, time: u16, load: f32, damage: f32) -> Result<String, Error> {
        self.shoot(
            direction,
            direction.angle(),
            time,
            load,
            damage,
            0f32,
            0f32,
            &[]
        )
    }

    fn shoot(&self, direction: &Vector, launch_angle: f32, time: u16, load: f32, damage_hull: f32,
             damage_shields: f32, damage_energy: f32, sub_directions: &[SubDirection]) -> Result<String, Error> {
        if sub_directions.len() > 255 {
            return Err(Error::TooManySubDirections(sub_directions.len()));
        }

        let mut packet = Packet::new();

        packet.set_command(0xA0);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            direction.write(writer)?;
            writer.write_f32(launch_angle)?;
            writer.write_u16(time)?;
            writer.write_f32(load)?;
            writer.write_f32(damage_hull)?;
            writer.write_f32(damage_shields)?;
            writer.write_f32(damage_energy)?;
            writer.write_u8(sub_directions.len() as u8)?;

            for sub in sub_directions.iter() {
                sub.write(writer)?;
            }
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        let response = block.wait()?;
        let reader = &mut response.read() as &mut BinaryReader;
        Ok(reader.read_string()?)
    }

    /// Transfer energy to another ship in range of 200
    fn transfer_energy(&self, destination: &str, energy: f32, particles: f32, ions: f32) -> Result<(), Error> {
        if !Connector::check_name(destination) {
            return Err(Error::InvalidDestination);
        }

        if !energy.is_finite() || energy.is_sign_negative() {
            return Err(Error::InvalidEnergyValue(energy));
        }

        if !particles.is_finite() || energy.is_sign_negative() {
            return Err(Error::InvalidParticlesValue(particles));
        }

        if !ions.is_finite() || ions.is_sign_negative() {
            return Err(Error::InvalidIonsValue(ions));
        }

        let mut packet = Packet::new();

        packet.set_command(0xA1);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_string(destination)?;
            writer.write_f32(energy)?;
            writer.write_f32(particles)?;
            writer.write_f32(ions)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet);

        block.wait()?;
        Ok(())
    }

    /// Engages the tractorbeam of the ship into the given direction, range and with
    /// the specified force. The tractorbeam changes the movement-[Vector] of
    /// objects towards or away from you. When you affect a [Mobility#Still]
    /// or [Mobility#Steady] [Unit] your movement gets affected in the opposite way.
    fn tractorbeam(&self, mut direction: f32, range: f32, force: f32) -> Result<bool, Error> {
        if !direction.is_finite() {
            return Err(Error::InvalidDirectionValue(direction));
        }

        if !range.is_finite() || range < 0_f32 || range > 2_000_f32 {
            return Err(Error::InvalidRangeValue(range));
        }

        if !force.is_finite() || force > 20_f32 || force < -20_f32 {
            return Err(Error::InvalidForceValue(force));
        }

        while direction < 0f32 {
            direction += 3_600_f32;
        }

        direction %= 360_f32;

        let mut packet = Packet::new();

        packet.set_command(0xA2);
        packet.set_path_ship(self.id());

        {
            let writer = &mut packet.write() as &mut BinaryWriter;
            writer.write_f32(direction)?;
            writer.write_f32(range)?;
            writer.write_f32(force)?;
        }

        let connector = match self.connector().upgrade() {
            None => return Err(Error::ConnectorNotAvailable),
            Some(connector) => connector
        };

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        packet.set_session(block.id());
        connector.send(&packet)?;

        let response = block.wait()?;
        Ok(response.path_sub() == 1)
    }
}

pub struct ControllableData {
    id: u8,
    revision: i64,
    class: String,
    name: String,
    level: u8,
    radius: f32,
    gravity: f32,
    efficiency_tactical: f32,
    efficiency_economical: f32,
    visible_range_multiplier: f32,
    energy_max: f32,
    particles_max: f32,
    ions_max: f32,
    energy_cells: f32,
    particles_cells: f32,
    ions_cells: f32,
    energy_reactor: f32,
    particles_reactor: f32,
    ions_reactor: f32,
    hull_max: f32,
    hull_armor: f32,
    hull_repair: EnergyCost,
    shield_max: f32,
    shield_armor: f32,
    shield_load: EnergyCost,
    engine_speed: f32,
    engine_acceleration: EnergyCost,
    scanner_degree_per_scan: f32,
    scanner_count: u8,
    scanner_area: ScanEnergyCost,
    weapon_shot: WeaponEnergyCost,
    weapon_hull: f32,
    weapon_hull_armor: f32,
    weapon_shield: f32,
    weapon_shield_armor: f32,
    weapon_visible_range_multiplier: f32,
    weapon_gravity: f32,
    weapon_size: f32,
    weapon_production: f32,
    weapon_production_load: f32,
    weapon_sub_directions: u8,
    weapon_sub_directions_length: f32,
    builder_time: f32,
    builder_time_factory_energy: f32,
    builder_time_factory_particles: f32,
    builder_time_factory_ions: f32,
    builder_capabilities: Vec<UnitKind>,
    energy_transfer_energy: EnergyCost,
    energy_transfer_particles: EnergyCost,
    energy_transfer_ions: EnergyCost,
    cargo_slots: u8,
    cargo_amount: f32,
    crystal_converter: EnergyCost,
    crystal_slots: u8,
    tractor_beam: EnergyCost,
    tractor_beam_range: f32,
    scores: Arc<Scores>,
    energy: f32,
    particles: f32,
    ions: f32,
    hull: f32,
    shield: f32,
    build_position: Option<Vector>,
    build_progress: f32,
    is_building: Option<Weak<RwLock<Controllable>>>,
    is_built_by: Option<Weak<RwLock<Controllable>>>,
    weapon_production_status: f32,
    crystals:       RwLock<Arc<Vec<Box<CrystalCargoItem>>>>,
    cargo_items:    RwLock<Arc<Vec<Box<CargoItem>>>>,
    universe: Weak<RwLock<Universe>>,
    haste_time: u16,
    double_damage_time: u16,
    quad_damage_time: u16,
    cloak_time: u16,
    connector: Weak<Connector>,
    active: bool,
    pending_shutdown: bool,
    scan_list: RwLock<Vec<Box<Unit>>>
}

impl ControllableData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<ControllableData, Error>  {
        let id  = packet.path_ship();
        let universe = match connector.player().upgrade() {
            None => return Err(Error::PlayerNotAvailable),
            Some(player) => {
                match player.read()?.universe_group().upgrade() {
                    None => return Err(Error::PlayerNotInUniverseGroup),
                    Some(universe_group) => {
                        universe_group.read()?.universe(packet.path_universe())
                    }
                }
            }
        };

        Ok(ControllableData {
            id,
            connector: Arc::downgrade(connector),
            universe,

            revision:                       reader.read_i64()?,
            class:                          reader.read_string()?,
            name:                           reader.read_string()?,
            level:                          reader.read_unsigned_byte()?,
            radius:                         reader.read_single()?,
            gravity:                        reader.read_single()?,
            efficiency_tactical:            reader.read_single()?,
            efficiency_economical:          reader.read_single()?,
            visible_range_multiplier:       reader.read_single()?,
            energy_max:                     reader.read_single()?,
            particles_max:                  reader.read_single()?,
            ions_max:                       reader.read_single()?,
            energy_cells:                   reader.read_single()?,
            particles_cells:                reader.read_single()?,
            ions_cells:                     reader.read_single()?,
            energy_reactor:                 reader.read_single()?,
            particles_reactor:              reader.read_single()?,
            ions_reactor:                   reader.read_single()?,
            hull_max:                       reader.read_single()?,
            hull_armor:                     reader.read_single()?,
            hull_repair:                    EnergyCost::from_reader(connector, reader)?,
            shield_max:                     reader.read_single()?,
            shield_armor:                   reader.read_single()?,
            shield_load:                    EnergyCost::from_reader(connector, reader)?,
            engine_speed:                   reader.read_single()?,
            engine_acceleration:            EnergyCost::from_reader(connector, reader)?,
            scanner_degree_per_scan:        reader.read_single()?,
            scanner_count:                  reader.read_unsigned_byte()?,
            scanner_area:                   ScanEnergyCost::from_reader(connector, reader)?,
            weapon_shot:                    WeaponEnergyCost::from_reader(connector, reader)?,
            weapon_hull:                    reader.read_single()?,
            weapon_hull_armor:              reader.read_single()?,
            weapon_shield:                  reader.read_single()?,
            weapon_shield_armor:            reader.read_single()?,
            weapon_visible_range_multiplier:reader.read_single()?,
            weapon_gravity:                 reader.read_single()?,
            weapon_size:                    reader.read_single()?,
            weapon_production:              reader.read_single()?,
            weapon_production_load:         reader.read_single()?,
            weapon_sub_directions:          reader.read_byte()?,
            weapon_sub_directions_length:   reader.read_single()?,
            builder_time:                   reader.read_single()?,
            builder_time_factory_energy:    reader.read_single()?,
            builder_time_factory_particles: reader.read_single()?,
            builder_time_factory_ions:      reader.read_single()?,
            builder_capabilities: {
                let mut vec = Vec::new();
                let count = reader.read_byte()?;
                for _ in 0..count {
                    vec.push(UnitKind::from_id(reader.read_byte()?));
                }
                vec
            },
            energy_transfer_energy:         EnergyCost::from_reader(connector, reader)?,
            energy_transfer_particles:      EnergyCost::from_reader(connector, reader)?,
            energy_transfer_ions:           EnergyCost::from_reader(connector, reader)?,
            cargo_slots:                    reader.read_unsigned_byte()?,
            cargo_amount:                   reader.read_single()?,
            crystal_converter:              EnergyCost::from_reader(connector, reader)?,
            crystal_slots:                  reader.read_byte()?,
            tractor_beam:                   EnergyCost::from_reader(connector, reader)?,
            tractor_beam_range:             reader.read_single()?,

            scores: {
                match connector.player().upgrade() {
                    None => return Err(Error::PlayerNotAvailable),
                    Some(player) => {
                        match player.read()?.controllable_info(packet.path_ship()) {
                            None => return Err(Error::ControllableInfoNotAvailable),
                            Some(info) => info.read()?.scores().clone()
                        }
                    }
                }
            },

            active:                 true,
            haste_time:             0u16,
            double_damage_time:     0u16,
            quad_damage_time:       0u16,
            energy:                 0f32,
            particles:              0f32,
            ions:                   0f32,
            hull:                   0f32,
            shield:                 0f32,
            build_position:         None,
            build_progress:         0f32,
            is_building:            None,
            is_built_by:            None,
            weapon_production_status:0f32,
            crystals:               RwLock::new(Arc::new(Vec::new())),
            cargo_items:            RwLock::new(Arc::new(Vec::new())),
            cloak_time:             0u16,
            pending_shutdown:       false,
            scan_list:              RwLock::new(Vec::new())
        })
    }

    pub fn update(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;

        self.energy             = reader.read_single()?;
        self.particles          = reader.read_single()?;
        self.ions               = reader.read_single()?;
        self.hull               = reader.read_single()?;
        self.shield             = reader.read_single()?;
        self.pending_shutdown   = reader.read_bool()?;
        Ok(())
    }

    pub fn update_extended(&mut self, packet: &Packet) -> Result<(), Error> {
        let reader = &mut packet.read() as &mut BinaryReader;

        self.weapon_production_status   = reader.read_single()?;
        let header                      = reader.read_byte()?;

        if is_set_u8(header, 0x03) {
            self.build_progress = 0f32;
        }

        if is_set_u8(header, 0x01) {
            match self.connector.upgrade() {
                None => return Err(Error::ConnectorNotAvailable),
                Some(connector) => {
                    self.build_progress = reader.read_single()?;
                    self.is_building    = connector.controllable_weak(reader.read_unsigned_byte()?);
                    self.build_position = Some(Vector::from_reader_with_connector(reader, &connector)?);
                }
            };
        } else {
            self.build_position = None;
            self.is_building    = None;
        }

        if is_set_u8(header, 0x02) {
            self.build_progress = reader.read_single()?;
            self.is_built_by    = match self.connector.upgrade() {
                None => return Err(Error::ConnectorNotAvailable),
                Some(connector) => {
                    connector.controllable_weak(reader.read_unsigned_byte()?)
                }
            };
        } else {
            self.is_built_by = None;
        }

        if is_set_u8(header, 0x10) {
            self.haste_time = reader.read_u16()?;
        } else {
            self.haste_time = 0u16;
        }

        if is_set_u8(header, 0x40) {
            self.quad_damage_time = reader.read_u16()?;
        } else {
            self.haste_time = 0u16;
        }

        if is_set_u8(header, 0x80) {
            self.cloak_time = reader.read_u16()?;
        } else {
            self.cloak_time = 0u16;
        }

        Ok(())
    }
}

impl Controllable for ControllableData {
    fn id(&self) -> u8 {
        self.id
    }

    fn revision(&self) -> i64 {
        self.revision
    }
    
    fn class(&self) -> &str {
        &self.class
    }
    
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> u8 {
        self.level
    }

    fn radius(&self) -> f32 {
        self.radius
    }


    fn gravity(&self) -> f32 {
        self.gravity
    }

    fn efficiency_tactical(&self) -> f32 {
        self.efficiency_tactical
    }

    fn efficiency_economical(&self) -> f32 {
        self.efficiency_economical
    }

    fn visible_range_multiplier(&self) -> f32 {
        self.visible_range_multiplier
    }

    fn energy_max(&self) -> f32 {
        self.energy_max
    }

    fn particles_max(&self) -> f32 {
        self.particles_max
    }

    fn ions_max(&self) -> f32 {
        self.ions_max
    }

    fn energy_cells(&self) -> f32 {
        self.energy_cells
    }

    fn particles_cells(&self) -> f32 {
        self.particles_cells
    }

    fn ions_cells(&self) -> f32 {
        self.ions_cells
    }

    fn energy_reactor(&self) -> f32 {
        self.energy_reactor
    }

    fn particles_reactor(&self) -> f32 {
        self.particles_reactor
    }

    fn ions_reactor(&self) -> f32 {
        self.ions_reactor
    }

    fn hull_max(&self) -> f32 {
        self.hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.hull_armor
    }

    fn hull_repair(&self) -> &EnergyCost {
        &self.hull_repair
    }

    fn shield_max(&self) -> f32 {
        self.shield_max
    }

    fn shield_armor(&self) -> f32 {
        self.shield_armor
    }

    fn shield_load(&self) -> &EnergyCost {
        &self.shield_load
    }

    fn engine_speed(&self) -> f32 {
        self.engine_speed
    }

    fn engine_acceleration(&self) -> &EnergyCost {
        &self.engine_acceleration
    }

    fn scanner_degree_per_scan(&self) -> f32 {
        self.scanner_degree_per_scan
    }

    fn scanner_count(&self) -> u8 {
        self.scanner_count
    }

    fn scanner_area(&self) -> &ScanEnergyCost {
        &self.scanner_area
    }

    fn weapon_shot(&self) -> &WeaponEnergyCost {
        &self.weapon_shot
    }

    fn weapon_hull(&self) -> f32 {
        self.weapon_hull
    }

    fn weapon_hull_armor(&self) -> f32 {
        self.weapon_hull_armor
    }

    fn weapon_shield(&self) -> f32 {
        self.weapon_shield
    }

    fn weapon_shield_armor(&self) -> f32 {
        self.weapon_shield_armor
    }

    fn weapon_visible_range_multiplier(&self) -> f32 {
        self.weapon_visible_range_multiplier
    }

    fn weapon_gravity(&self) -> f32 {
        self.weapon_gravity
    }

    fn weapon_size(&self) -> f32 {
        self.weapon_size
    }

    fn weapon_production(&self) -> f32 {
        self.weapon_production
    }

    fn weapon_production_load(&self) -> f32 {
        self.weapon_production_load
    }

    fn weapon_sub_directions(&self) -> u8 {
        self.weapon_sub_directions
    }

    fn weapon_sub_directions_length(&self) -> f32 {
        self.weapon_sub_directions_length
    }

    fn builder_time(&self) -> f32 {
        self.builder_time
    }

    fn builder_time_factory_energy(&self) -> f32 {
        self.builder_time_factory_energy
    }

    fn builder_time_factory_particles(&self) -> f32 {
        self.builder_time_factory_particles
    }

    fn builder_time_factory_ions(&self) -> f32 {
        self.builder_time_factory_ions
    }

    fn builder_capabilities(&self) -> &Vec<UnitKind> {
        &self.builder_capabilities
    }

    fn energy_transfer_energy(&self) -> &EnergyCost {
        &self.energy_transfer_energy
    }

    fn energy_transfer_particles(&self) -> &EnergyCost {
        &self.energy_transfer_particles
    }

    fn energy_transfer_ions(&self) -> &EnergyCost {
        &self.energy_transfer_ions
    }

    fn cargo_slots(&self) -> u8 {
        self.cargo_slots
    }

    fn cargo_amount(&self) -> f32 {
        self.cargo_amount
    }

    fn crystal_converter(&self) -> &EnergyCost {
        &self.crystal_converter
    }

    fn crystal_slots(&self) -> u8 {
        self.crystal_slots
    }

    fn tractor_beam(&self) -> &EnergyCost {
        &self.tractor_beam
    }

    fn tractor_beam_range(&self) -> f32 {
        self.tractor_beam_range
    }

    fn scores(&self) -> &Arc<Scores> {
        &self.scores
    }

    fn energy(&self) -> f32 {
        self.energy
    }

    fn particles(&self) -> f32 {
        self.particles
    }

    fn ions(&self) -> f32 {
        self.ions
    }

    fn hull(&self) -> f32 {
        self.hull
    }

    fn shield(&self) -> f32 {
        self.shield
    }

    fn build_position(&self) -> &Option<Vector> {
        &self.build_position
    }

    fn build_progress(&self) -> f32 {
        self.build_progress
    }

    fn is_building(&self) -> &Option<Weak<RwLock<Controllable>>> {
        &self.is_building
    }

    fn is_built_by(&self) -> &Option<Weak<RwLock<Controllable>>> {
        &self.is_built_by
    }

    fn weapon_production_status(&self) -> f32 {
        self.weapon_production_status
    }

    fn crystals(&self) -> Arc<Vec<Box<CrystalCargoItem>>> {
        self.crystals.read().unwrap().clone()
    }

    fn set_crystals(&self, crystals: Arc<Vec<Box<CrystalCargoItem>>>) {
        *self.crystals.write().unwrap() = crystals;
    }

    fn cargo_items(&self) -> Arc<Vec<Box<CargoItem>>> {
        self.cargo_items.read().unwrap().clone()
    }

    fn set_cargo_items(&self, items: Arc<Vec<Box<CargoItem>>>) {
        *self.cargo_items.write().unwrap() = items;
    }

    fn universe(&self) -> &Weak<RwLock<Universe>> {
        &self.universe
    }

    fn haste_time(&self) -> u16 {
        self.haste_time
    }

    fn double_damage_time(&self) -> u16 {
        self.double_damage_time
    }

    fn quad_damage_time(&self) -> u16 {
        self.quad_damage_time
    }

    fn cloak_time(&self) -> u16 {
        self.cloak_time
    }

    fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    fn active(&self) -> bool {
        self.active
    }

    fn pending_shutdown(&self) -> bool {
        self.pending_shutdown
    }

    fn scan_list(&self) -> &RwLock<Vec<Box<Unit>>> {
        &self.scan_list
    }
}

impl Display for Controllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}, rev: {}, id: {}", self.name(), self.class(), self.revision(), self.id())
    }
}

impl UniversalEnumerable for Controllable {
    fn name(&self) -> &str {
        self.name()
    }
}