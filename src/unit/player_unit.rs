use crate::hierarchy::{ClusterId, ControllableInfoId, ShipDesignId, ShipUpgradeId};
use crate::network::PacketReader;
use crate::unit::{Unit, UnitKind};
use crate::{ControllableId, Indexer, PlayerId, Vector};
use std::any::Any;

#[derive(Debug)]
pub struct PlayerUnit {
    name: String,
    cluster: ClusterId,

    player: PlayerId,
    controllable_info: ControllableInfoId,

    size: f64,
    weight: f64,
    thruster: f64,
    nozzle: f64,
    turnrate: f64,
    weapon_ammo: f64,

    direction: f64,
    position: Vector,
    movement: Vector,

    active: bool,
}

impl PlayerUnit {
    pub fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            name: reader.read_string(),

            player: PlayerId(reader.read_byte()),
            controllable_info: ControllableInfoId(reader.read_byte()),

            size: reader.read_double(),
            weight: reader.read_double(),
            thruster: reader.read_double(),
            nozzle: reader.read_double(),
            turnrate: reader.read_double(),
            weapon_ammo: reader.read_double(),

            direction: reader.read_double(),
            position: Vector::default().with_read(reader),
            movement: Vector::default().with_read(reader),

            active: true,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn cluster(&self) -> ClusterId {
        self.cluster
    }

    #[inline]
    pub fn player(&self) -> PlayerId {
        self.player
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.size
    }

    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    #[inline]
    pub fn thruster(&self) -> f64 {
        self.thruster
    }

    #[inline]
    pub fn nozzle(&self) -> f64 {
        self.nozzle
    }

    #[inline]
    pub fn turnrate(&self) -> f64 {
        self.turnrate
    }

    #[inline]
    pub fn weapon_ammo(&self) -> f64 {
        self.weapon_ammo
    }

    #[inline]
    pub fn direction(&self) -> f64 {
        self.direction
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position
    }

    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active
    }
}

impl Unit for PlayerUnit {
    #[inline]
    fn active(&self) -> bool {
        self.active
    }

    #[inline]
    fn name(&self) -> &str {
        PlayerUnit::name(self)
    }

    #[inline]
    fn cluster(&self) -> ClusterId {
        PlayerUnit::cluster(self)
    }

    fn movement(&self) -> Vector {
        self.movement
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.weight
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.size
    }

    fn update(&mut self, reader: &mut dyn PacketReader) {
        let _ = reader.read_string(); // 'jump over string'

        self.thruster = reader.read_double();
        self.nozzle = reader.read_double();
        self.turnrate = reader.read_double();
        self.weapon_ammo = reader.read_double();

        self.direction = reader.read_double();
        self.position.read(reader);
        self.movement.read(reader);
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Ship
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        &*self
    }
}
