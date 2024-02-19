use crate::hierarchy::{ClusterConfig, GlaxyId, Region, RegionConfig, RegionId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::configurations::SunConfiguration;
use crate::unit::{Unit, UnitKind};
use crate::{FlattiverseEvent, GameError, Indexer, NamedUnit, UniversalHolder};
use rustc_hash::FxHashMap;
use std::future::Future;

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
    id: ClusterId,
    galaxy: GlaxyId,
    config: ClusterConfig,
    units: FxHashMap<String, Box<dyn Unit>>,
    regions: UniversalHolder<RegionId, Region>,
    connection: ConnectionHandle,
}

impl Cluster {
    #[inline]
    pub fn new(
        id: impl Into<ClusterId>,
        galaxy: GlaxyId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            connection,
            config: ClusterConfig::from(reader),
            units: FxHashMap::default(),
            regions: UniversalHolder::with_capacity(256),
        }
    }

    #[inline]
    pub(crate) fn read_region(&mut self, id: RegionId, reader: &mut dyn PacketReader) {
        self.regions.set(
            id,
            Region::new(self.galaxy, self.id, id, self.connection.clone(), reader),
        );
    }

    /// Sets the given values for this [`Cluster`].
    /// See also [`ConnectionHandle::configure_cluster`].
    pub async fn configure(
        &self,
        config: &ClusterConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_cluster_split(self.id, config)
            .await
    }

    /// Removes this [`Cluster`].
    /// See also [`ConnectionHandle::remove_cluster`].
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_cluster_split(self.id).await
    }

    /// Creates a [`Region`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_region`].
    pub async fn create_region(
        &self,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<RegionId, GameError>>, GameError> {
        self.connection.create_region_split(self.id, config).await
    }

    /// Creates a [`Sun`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_sun`].
    pub async fn create_sun(
        &self,
        config: &SunConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.create_sun_split(self.id, config).await
    }

    // TODO pub async fn create_sun
    // TODO pub async fn create_blackhole
    // TODO pub async fn create_planet
    // TODO pub async fn create_moon
    // TODO pub async fn create_meteoroid
    // TODO pub async fn create_buoy

    pub(crate) fn see_new_unit(
        &mut self,
        kind: UnitKind,
        reader: &mut dyn PacketReader,
    ) -> Result<FlattiverseEvent, GameError> {
        // let unit_kind = UnitKind::try_from_primitive(packet.header().param0()).unwrap();
        let unit = crate::unit::from_packet(self.id, kind, reader)?;
        let name = unit.name().to_string();
        self.units.insert(name.clone(), unit);
        Ok(FlattiverseEvent::SeeingNewUnit {
            galaxy: self.galaxy,
            cluster: self.id,
            name,
        })
    }

    pub(crate) fn see_update_unit(
        &mut self,
        reader: &mut dyn PacketReader,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        let name = reader.peek_string();
        match self.units.get_mut(&name) {
            Some(unit) => {
                unit.update(reader);
                Ok(Some(FlattiverseEvent::SeeingUnitUpdated {
                    galaxy: self.galaxy,
                    cluster: self.id,
                    name,
                }))
            }
            None => {
                warn!("Requested unit '{name}' should be known but isn't in the units dictionary.");
                Ok(None)
            }
        }
    }

    pub(crate) fn see_unit_no_more(
        &mut self,
        name: String,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        if let Some(_) = self.units.remove(&name) {
            // unit.deactivate();
            Ok(Some(FlattiverseEvent::SeeingUnitNoMore {
                galaxy: self.galaxy,
                cluster: self.id,
                name,
            }))
        } else {
            warn!("Requested unit '{name}' should be known but isn't in the units dictionary.");
            Ok(None)
        }
    }

    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> GlaxyId {
        self.galaxy
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &ClusterConfig {
        &self.config
    }

    #[inline]
    pub fn regions(&self) -> &UniversalHolder<RegionId, Region> {
        &self.regions
    }

    #[inline]
    pub fn get_unit(&self, name: &str) -> Option<&dyn Unit> {
        self.units.get(name).map(|unit| &**unit)
    }

    #[inline]
    pub fn iter_units(&self) -> impl Iterator<Item = (&str, &dyn Unit)> {
        self.units
            .iter()
            .map(|(name, unit)| (name.as_str(), &**unit))
    }
}

impl NamedUnit for Cluster {
    #[inline]
    fn name(&self) -> &str {
        Cluster::name(self)
    }
}
