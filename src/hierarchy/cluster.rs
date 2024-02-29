use crate::atomics::Atomic;
use crate::hierarchy::{ClusterConfig, Galaxy, Region, RegionConfig, RegionId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::configurations::{
    BlackHoleConfiguration, BuoyConfiguration, MeteoroidConfiguration, MoonConfiguration,
    PlanetConfiguration, SunConfiguration,
};
use crate::unit::{Unit, UnitKind};
use crate::{FlattiverseEvent, GameError, Identifiable, Indexer, UniversalArcHolder};
use arc_swap::ArcSwap;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ClusterId(pub(crate) u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Cluster {
    active: Atomic<bool>,
    id: ClusterId,
    galaxy: Arc<Galaxy>,
    config: ArcSwap<ClusterConfig>,
    units: UniversalArcHolder<(), Arc<dyn Unit>>,
    regions: UniversalArcHolder<RegionId, Region>,
}

impl Cluster {
    #[inline]
    pub fn new(
        galaxy: Arc<Galaxy>,
        id: impl Into<ClusterId>,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            active: Atomic::from(true),
            id: id.into(),
            galaxy,
            config: ArcSwap::new(Arc::new(ClusterConfig::from(reader))),
            units: UniversalArcHolder::with_capacity(256),
            regions: UniversalArcHolder::with_capacity(256),
        }
    }

    #[inline]
    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.config.store(Arc::new(ClusterConfig::from(reader)));
    }

    #[inline]
    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    /// Sets the given values for this [`Cluster`].
    /// See also [`ConnectionHandle::configure_cluster`].
    #[inline]
    pub async fn configure(&self, config: &ClusterConfig) -> Result<(), GameError> {
        self.connection().configure_cluster(self.id, config).await
    }

    /// Removes this [`Cluster`].
    /// See also [`ConnectionHandle::remove_cluster`].
    #[inline]
    pub async fn remove(&self) -> Result<(), GameError> {
        self.connection().remove_cluster(self.id).await
    }

    /// Creates a [`Region`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_region`].
    #[inline]
    pub async fn create_region(&self, config: &RegionConfig) -> Result<Arc<Region>, GameError> {
        let region = self.connection().create_region(self.id, config).await?;
        Ok(self.regions.get(region))
    }

    /// Creates a [`crate::unit::Sun`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_sun`].
    #[inline]
    pub async fn create_sun(
        &self,
        config: &SunConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.connection().create_sun(self.id, config).await?;
        Ok(self.get_unit(config.name()))
    }

    /// Creates a [`crate::unit::BlackHole`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_black_hole`].
    #[inline]
    pub async fn create_black_hole(
        &self,
        config: &BlackHoleConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.connection().create_black_hole(self.id, config).await?;
        Ok(self.get_unit(config.name()))
    }

    /// Creates a [`crate::unit::Planet`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_planet`].
    #[inline]
    pub async fn create_planet(
        &self,
        config: &PlanetConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.connection().create_planet(self.id, config).await?;
        Ok(self.get_unit(config.name()))
    }

    /// Creates a [`crate::unit::Moon`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_moon`].
    #[inline]
    pub async fn create_moon(
        &self,
        config: &MoonConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.galaxy
            .connection()
            .create_moon(self.id, config)
            .await?;
        Ok(self.get_unit(config.name()))
    }

    /// Creates a [`crate::unit::Meteoroid`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_meteoroid`].
    #[inline]
    pub async fn create_meteoroid(
        &self,
        config: &MeteoroidConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.galaxy
            .connection()
            .create_meteoroid(self.id, config)
            .await?;
        Ok(self.get_unit(config.name()))
    }

    /// Creates a [`crate::unit::Buoy`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_buoy`].
    #[inline]
    pub async fn create_buoy(
        &self,
        config: &BuoyConfiguration,
    ) -> Result<Arc<dyn Unit + 'static>, GameError> {
        self.galaxy
            .connection()
            .create_buoy(self.id, config)
            .await?;
        Ok(self.get_unit(config.name()))
    }

    pub(crate) fn see_new_unit(
        self: &Arc<Cluster>,
        kind: UnitKind,
        reader: &mut dyn PacketReader,
    ) -> Result<FlattiverseEvent, GameError> {
        let unit = crate::unit::from_packet(Arc::clone(self), kind, reader)?;
        self.units.push(Arc::new(Arc::clone(&unit)));
        Ok(FlattiverseEvent::SeeingNewUnit { unit })
    }

    pub(crate) fn see_update_unit(
        self: &Arc<Self>,
        reader: &mut dyn PacketReader,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        let name = reader.peek_string();
        match self.units.get_by_name_opt(&name) {
            Some(unit) => {
                unit.update(reader);
                Ok(Some(FlattiverseEvent::SeeingUnitUpdated {
                    unit: Arc::clone(&*unit),
                }))
            }
            None => {
                warn!("Requested unit '{name}' should be known but isn't in the units dictionary.");
                Ok(None)
            }
        }
    }

    pub(crate) fn see_unit_no_more(
        self: &Arc<Self>,
        name: String,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        if let Some(unit) = self.units.remove_by_name_opt(&name) {
            unit.deactivate();
            Ok(Some(FlattiverseEvent::SeeingUnitNoMore {
                unit: Arc::clone(&*unit),
            }))
        } else {
            warn!("Requested unit '{name}' should be known but isn't in the units dictionary.");
            Ok(None)
        }
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> &Arc<Galaxy> {
        &self.galaxy
    }

    #[inline]
    pub fn config(&self) -> impl Deref<Target = Arc<ClusterConfig>> {
        self.config.load()
    }

    #[inline]
    pub fn get_region(&self, id: RegionId) -> Arc<Region> {
        self.regions.get(id)
    }

    #[inline]
    pub fn regions(&self) -> &UniversalArcHolder<RegionId, Region> {
        &self.regions
    }

    #[inline]
    pub fn get_unit(&self, name: &str) -> Arc<dyn Unit> {
        Arc::clone(&self.units.get_by_name(name))
    }

    #[inline]
    pub fn iter_units(&self) -> impl Iterator<Item = Arc<dyn Unit + 'static>> + '_ {
        self.units.iter().map(|unit| Arc::clone(&*unit))
    }

    #[inline]
    pub fn units(&self) -> &UniversalArcHolder<(), Arc<dyn Unit>> {
        &self.units
    }

    #[inline]
    pub fn connection(&self) -> &ConnectionHandle {
        self.galaxy.connection()
    }
}

impl Identifiable<ClusterId> for Cluster {
    #[inline]
    fn id(&self) -> ClusterId {
        self.id
    }
}
