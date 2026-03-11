use crate::galaxy_hierarchy::{Cluster, Team};
use crate::unit::{Mobility, UnitExt, UnitExtSealed, UnitKind};
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug, Clone)]
pub struct UnitBase {
    name: String,
    cluster: Weak<Cluster>,
}

impl UnitBase {
    pub(crate) fn new(cluster: Weak<Cluster>, name: String) -> Self {
        Self { cluster, name }
    }

    /// The name of the unit. A unit can't change her name after it has been set up.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The cluster the unit is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.upgrade().unwrap()
    }
}

impl<'a> UnitExtSealed<'a> for &'a UnitBase {
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        unreachable!()
    }
}

impl<'a> UnitExt<'a> for &'a UnitBase {
    #[inline]
    fn name(self) -> &'a str {
        &self.name
    }

    #[inline]
    fn radius(self) -> f32 {
        3.0
    }

    #[inline]
    fn position(self) -> Vector {
        Vector::default()
    }

    #[inline]
    fn movement(self) -> Vector {
        Vector::default()
    }

    #[inline]
    fn angle(self) -> f32 {
        0.0
    }

    #[inline]
    fn is_masking(self) -> bool {
        true
    }

    #[inline]
    fn is_solid(self) -> bool {
        true
    }

    #[inline]
    fn can_be_edited(self) -> bool {
        false
    }

    #[inline]
    fn gravity(self) -> f32 {
        0.0
    }

    #[inline]
    fn mobility(self) -> Mobility {
        Mobility::Still
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Sun
    }

    #[inline]
    fn cluster(self) -> Arc<Cluster> {
        UnitBase::cluster(self)
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        Weak::default()
    }
}
