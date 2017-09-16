
use std::sync::Arc;
use std::ops::Deref;

use Error;
use Vector;
use Connector;
use controllable::EnergyCost;
use controllable::Controllable;

use net::BinaryReader;



#[derive(Clone)]
pub struct WeaponEnergyCost {
    load:           EnergyCost,
    speed:          EnergyCost,
    time:           EnergyCost,
    damage_hull:    EnergyCost,

    damage_hull_crit:       f32,
    damage_hull_crit_chance:f32,

    damage_shield:  EnergyCost,

    damage_shield_crit:         f32,
    damage_shield_crit_chance:  f32,

    damage_energy:  EnergyCost,
    damage_energy_crit:         f32,
    damage_energy_crit_chance:  f32,
}

impl WeaponEnergyCost {
    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader) -> Result<WeaponEnergyCost, Error> {
        Ok(WeaponEnergyCost {
            load:                       EnergyCost::from_reader(connector, reader)?,
            speed:                      EnergyCost::from_reader(connector, reader)?,
            time:                       EnergyCost::from_reader(connector, reader)?,
            damage_hull:                EnergyCost::from_reader(connector, reader)?,
            damage_hull_crit:           reader.read_single()?,
            damage_hull_crit_chance:    reader.read_single()?,
            damage_shield:              EnergyCost::from_reader(connector, reader)?,
            damage_shield_crit:         reader.read_single()?,
            damage_shield_crit_chance:  reader.read_single()?,
            damage_energy:              EnergyCost::from_reader(connector, reader)?,
            damage_energy_crit:         reader.read_single()?,
            damage_energy_crit_chance:  reader.read_single()?,
        })
    }

    pub fn calculate_energy_usage(&self, value: f32, energy: &mut f32, particles: &mut f32, ions: &mut f32, controllable: &Controllable) -> Result<bool, Error> {

        if !value.is_finite() {
            return Err(Error::InvalidValue(value))
        }

        if value < 0_f32 || value > self.load.extended_limit() * 1.000_000_06_f32 {
            return Err(Error::InvalidValue(value));
        }

        *energy = self.damage_hull  .cost_energy(self.damage_hull.limit())
                + self.damage_shield.cost_energy(self.damage_shield.limit())
                + self.damage_energy.cost_energy(self.damage_energy.limit())
                + self.speed        .cost_energy(self.speed.limit())
                + self.time         .cost_energy(self.time.limit());

        *particles
                = self.damage_hull  .cost_particles(self.damage_hull.limit())
                + self.damage_shield.cost_particles(self.damage_hull.limit())
                + self.damage_energy.cost_particles(self.damage_energy.limit())
                + self.speed        .cost_particles(self.speed.limit())
                + self.time         .cost_particles(self.time.limit());

        *ions   = self.damage_hull  .cost_ions(self.damage_hull.limit())
                + self.damage_shield.cost_ions(self.damage_shield.limit())
                + self.damage_energy.cost_ions(self.damage_energy.limit())
                + self.speed        .cost_ions(self.speed.limit())
                + self.time         .cost_ions(self.time.limit());

        if value <= self.load.limit() {
            *energy     += self.load.cost_energy(value);
            *particles  += self.load.cost_particles(value);
            *ions       += self.load.cost_ions(value);

        } else {
            *energy     += self.load.extended_cost_energy(value);
            *particles  += self.load.extended_cost_particles(value);
            *ions       += self.load.extended_cost_ions(value);
        }

        Ok(
            *energy     <= controllable.energy() &&
            *particles  <= controllable.particles() &&
            *ions       <= controllable.ions()
        )
    }

    pub fn calculate_energy_usage_for_pos(&self, pos: &Vector, load: f32, time: f32, damage_hull: f32,
                                          damage_shield: f32, damage_energy: f32, energy: &mut f32,
                                          particles: &mut f32, ions: &mut f32, controllable: &Controllable) -> Result<bool, Error> {
        if pos.damaged() {
            return Err(Error::VectorIsDamaged);
        }

        self.calculate_energy_usage_for_len(
            pos.length(),
            load,
            time,
            damage_hull,
            damage_shield,
            damage_energy,
            energy,
            particles,
            ions,
            controllable
        )
    }

    pub fn calculate_energy_usage_for_len(&self, len: f32, load: f32, time: f32, damage_hull: f32,
                                          damage_shield: f32, damage_energy: f32, energy: &mut f32,
                                          particles: &mut f32, ions: &mut f32, controllable: &Controllable) -> Result<bool, Error> {

        *energy = self.damage_hull  .cost_energy(damage_hull)
                + self.damage_shield.cost_energy(damage_shield)
                + self.damage_energy.cost_energy(damage_energy)
                + self.load         .cost_energy(load)
                + self.speed        .cost_energy(len)
                + self.time         .cost_energy(time);

        *particles
                = self.damage_hull  .cost_particles(damage_hull)
                + self.damage_shield.cost_particles(damage_shield)
                + self.damage_energy.cost_particles(damage_energy)
                + self.load         .cost_particles(load)
                + self.speed        .cost_particles(len)
                + self.time         .cost_particles(time);

        *ions   = self.damage_hull  .cost_ions(damage_hull)
                + self.damage_shield.cost_ions(damage_shield)
                + self.damage_energy.cost_ions(damage_energy)
                + self.load         .cost_ions(load)
                + self.speed        .cost_ions(len)
                + self.time         .cost_ions(time);


        Ok(
            *energy     <= controllable.energy() &&
            *particles  <= controllable.particles() &&
            *ions       <= controllable.ions()
        )
    }

    /// The [EnergyCost] for the shots' load
    pub fn load(&self) -> &EnergyCost {
        &self.load
    }

    /// The [EnergyCost] for the shots' speed
    pub fn speed(&self) -> &EnergyCost {
        &self.speed
    }

    /// The [EnergyCost] for the shots' time
    pub fn time(&self) -> &EnergyCost {
        &self.time
    }

    /// The [EnergyCost] for the shots' hull damage
    pub fn damage_hull(&self) -> &EnergyCost {
        &self.damage_hull
    }

    /// -1 = No damage
    ///  0 = No critical damage
    ///  3 = 300% critical damage (= 400% total damage)
    ///
    /// Returns the amount of additional damage if
    /// the chance of a critical strike succeeded
    pub fn damage_hull_crit(&self) -> f32 {
        self.damage_hull_crit
    }

    ///  0 = No chance
    ///  1 = 100% chance
    ///
    /// Returns the chance for an critical strike
    pub fn damage_hull_crit_chance(&self) -> f32 {
        self.damage_hull_crit_chance
    }

    /// The [EnergyCost] for the shots' shield damage
    pub fn damage_shield(&self) -> &EnergyCost {
        &self.damage_shield
    }

    /// -1 = No damage
    ///  0 = No critical damage
    ///  3 = 300% critical damage (= 400% total damage)
    ///
    /// Returns the amount of additional damage if
    /// the chance of a critical strike succeeded
    pub fn damage_shield_crit(&self) -> f32 {
        self.damage_shield_crit
    }

    ///  0 = No chance
    ///  1 = 100% chance
    ///
    /// Returns the chance for critical strike
    pub fn damage_shield_crit_chance(&self) -> f32 {
        self.damage_shield_crit_chance
    }

    /// The [EnergyCost] for the shots' energy damage
    pub fn damage_energy(&self) -> &EnergyCost {
        &self.damage_energy
    }

    /// -1 = No damage
    ///  0 = No critical damage
    ///  3 = 300% critical damage (= 400% total damage)
    ///
    /// Returns the amount of additional damage if
    /// the chance of a critical strike succeeded
    pub fn damage_energy_crit(&self) -> f32 {
        self.damage_energy_crit
    }

    ///  0 = No chance
    ///  1 = 100% chance
    ///
    /// Returns the chance for a critical strike
    pub fn damage_energy_crit_chance(&self) -> f32 {
        self.damage_energy_crit_chance
    }

}

// WeaponEnergyCost 'extends' EnergyCost - kinda
impl Deref for WeaponEnergyCost {
    type Target = EnergyCost;

    fn deref(&self) -> &Self::Target {
        &self.load
    }
}