use crate::galaxy_hierarchy::{EditableUnitSummary, Galaxy, Identifiable, Indexer};
use crate::unit::Unit;
use crate::utils::Atomic;
use crate::utils::GuardedArcStringDeref;
use crate::{GameError, ProgressState};
use arc_swap::ArcSwap;
use crossbeam_skiplist::SkipMap;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ClusterId(pub(crate) u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// This is a subset of the galaxy. Each cluster is a separate map.
pub struct Cluster {
    id: ClusterId,
    galaxy: Weak<Galaxy>,
    name: ArcSwap<String>,
    start: Atomic<bool>,
    respawn: Atomic<bool>,
    active: Atomic<bool>,
    units: SkipMap<String, Arc<dyn Unit>>,
}

impl Debug for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cluster")
            .field("id", &self.id)
            .field("galaxy", &self.galaxy)
            .field("name", &self.name)
            .field("active", &self.active)
            .finish_non_exhaustive()
    }
}

impl Cluster {
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: ClusterId,
        name: impl Into<String>,
        start: bool,
        respawn: bool,
    ) -> Self {
        Self {
            galaxy,
            id,
            name: ArcSwap::new(Arc::new(name.into())),
            start: Atomic::from(start),
            respawn: Atomic::from(respawn),
            active: Atomic::from(true),
            units: SkipMap::new(),
        }
    }

    pub(crate) fn update(&self, name: String, start: bool, respawn: bool) {
        self.name.store(Arc::new(name));
        self.start.store(start);
        self.respawn.store(respawn);
        self.active.store(true);
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    pub(crate) fn add_unit(&self, unit: Arc<dyn Unit>) {
        self.units.insert(unit.name().to_string(), unit);
    }

    pub(crate) fn remove_unit_(&self, name: &str) -> Option<Arc<dyn Unit>> {
        self.units.remove(name).map(|e| Arc::clone(e.value()))
    }

    #[inline]
    pub fn get_unit(&self, unit: &str) -> Option<Arc<dyn Unit>> {
        self.units.get(unit).map(|e| Arc::clone(e.value()))
    }

    #[inline]
    pub fn iter_units(&self) -> impl Iterator<Item = Arc<dyn Unit>> + '_ {
        self.units.iter().map(|e| Arc::clone(e.value()))
    }

    /// The id within the galaxy of the cluster.
    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    /// The name of the cluster.
    #[inline]
    pub fn name(&self) -> impl Deref<Target = str> + '_ {
        GuardedArcStringDeref(self.name.load())
    }

    /// If true, freshly registered ships spawn in this cluster.
    #[inline]
    pub fn start(&self) -> bool {
        self.start.load()
    }

    /// If true, [`Controllable::r#continue`] spawns in this cluster.
    ///
    /// [`Controllable::r#continue`]: crate::galaxy_hierarchy::Controllable::r#continue
    #[inline]
    pub fn respawn(&self) -> bool {
        self.respawn.load()
    }

    /// If false, you have been disconnected or the cluster has been removed and therefore disabled.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.galaxy.upgrade().unwrap()
    }

    /// Creates or updates a region within this cluster:
    ///
    /// ```xml
    /// <Region Id="66" Name="Spawn A" Left="-150" Top="-300" Right="150" Bottom="0">
    ///   <Team Id="0" />
    /// </Region>
    /// ```
    #[inline]
    pub async fn set_region(&self, xml: impl AsRef<str>) -> Result<(), GameError> {
        self.galaxy()
            .connection()
            .set_cluster_region(self.id, xml)
            .await
    }

    /// Removes a region by id from this cluster.
    #[inline]
    pub async fn remove_region(&self, region: u8) -> Result<(), GameError> {
        self.galaxy()
            .connection()
            .remove_cluster_region(self.id, region)
            .await
    }

    /// Queries all regions of this cluster as XML.
    #[inline]
    pub async fn query_regions(&self) -> Result<String, GameError> {
        self.galaxy()
            .connection()
            .query_cluster_regions(self.id)
            .await
    }

    /// Queries all editable units of this cluster, including currently invisible ones like inactive power-ups.
    #[inline]
    pub async fn query_editable_units(
        &self,
        progress_state: impl Into<Option<Arc<ProgressState>>>,
    ) -> Result<Vec<EditableUnitSummary>, GameError> {
        self.galaxy()
            .connection()
            .query_cluster_editable_units(self.id, progress_state.into())
            .await
    }

    /// Creates or updates a single editable map unit in this cluster.
    /// Root node must be the unit type itself, for example `<Sun />`.
    /// For `<Buoy />` an optional message attribute is supported (max 384 characters).
    /// For `<MissionTarget />` the team is required, Achievement is optional and child nodes
    /// `<Vector X="..." Y="..." />` are supported.
    #[inline]
    pub async fn set_unit(&self, xml: impl AsRef<str>) -> Result<(), GameError> {
        self.galaxy()
            .connection()
            .set_cluster_unit(self.id, xml)
            .await
    }

    /// Removes a single editable map unit by name.
    #[inline]
    pub async fn remove_unit(&self, name: impl AsRef<str>) -> Result<(), GameError> {
        self.galaxy()
            .connection()
            .remove_cluster_unit(self.id, name)
            .await
    }

    /// Queries the XML of one specific editable map unit by name.
    #[inline]
    pub async fn query_unit_xml(&self, name: impl AsRef<str>) -> Result<String, GameError> {
        self.galaxy()
            .connection()
            .query_cluster_unit_xml(self.id, name)
            .await
    }
}

impl Identifiable<ClusterId> for Cluster {
    #[inline]
    fn id(&self) -> ClusterId {
        self.id
    }
}

impl Hash for dyn Unit {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl Eq for dyn Unit {}

impl PartialEq<Self> for dyn Unit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regions(pub Vec<Region>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    pub id: u8,
    pub name: Option<String>,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub teams: Vec<RegionTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegionTeam {
    pub id: u8,
}
