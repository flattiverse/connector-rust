
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;


use Downcast;

impl_downcast!(Controllable);
pub trait Controllable : Downcast + Debug + Display {

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
}