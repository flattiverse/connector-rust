use crate::atomics::Atomic;
use crate::hierarchy::{Cluster, ControllableInfo, ControllableInfoId, Galaxy};
use crate::network::PacketReader;
use crate::unit::{Unit, UnitKind};
use crate::{Player, PlayerId, Team, Vector};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug)]
pub struct PlayerUnit {
    name: String,
    cluster: Arc<Cluster>,

    player: Arc<Player>,
    controllable_info: Arc<ControllableInfo>,

    radius: f64,
    gravity: f64,
    thruster: Atomic<f64>,
    nozzle: Atomic<f64>,
    turnrate: Atomic<f64>,
    weapon_ammo: Atomic<f64>,

    direction: Atomic<f64>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,

    active: Atomic<bool>,
}

impl PlayerUnit {
    pub fn new(galaxy: &Galaxy, cluster: Arc<Cluster>, reader: &mut dyn PacketReader) -> Self {
        let name = reader.read_string();
        let player = galaxy.get_player(PlayerId(reader.read_byte()));
        let controllable_info = player
            .controllable_info()
            .get(ControllableInfoId(reader.read_byte()));

        Self {
            cluster,
            name,
            player,
            controllable_info,

            radius: reader.read_double(),
            gravity: reader.read_double(),
            thruster: Atomic::from_reader(reader),
            nozzle: Atomic::from_reader(reader),
            turnrate: Atomic::from_reader(reader),
            weapon_ammo: Atomic::from_reader(reader),

            direction: Atomic::from_reader(reader),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),

            active: Atomic::from(true),
        }
    }

    #[inline]
    pub fn player(&self) -> &Arc<Player> {
        &self.player
    }

    #[inline]
    pub fn controllable_info(&self) -> &Arc<ControllableInfo> {
        &self.controllable_info
    }

    #[inline]
    pub fn thruster(&self) -> f64 {
        self.thruster.load()
    }

    #[inline]
    pub fn nozzle(&self) -> f64 {
        self.nozzle.load()
    }

    #[inline]
    pub fn turnrate(&self) -> f64 {
        self.turnrate.load()
    }
    #[inline]
    pub fn weapon_ammo(&self) -> f64 {
        self.weapon_ammo.load()
    }
}

impl Unit for PlayerUnit {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    fn deactivate(&self) {
        self.active.store(false);
    }

    #[inline]
    fn cluster(&self) -> &Arc<Cluster> {
        &self.cluster
    }

    #[inline]
    fn direction(&self) -> f64 {
        self.direction.load()
    }

    fn movement(&self) -> Vector {
        self.movement.load()
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    fn gravity(&self) -> f64 {
        self.gravity
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.radius
    }

    #[inline]
    fn team(&self) -> Option<&Arc<Team>> {
        Some(&self.player.team())
    }

    fn update(&self, reader: &mut dyn PacketReader) {
        let _ = reader.read_string(); // 'jump over string'

        self.thruster.read(reader);
        self.nozzle.read(reader);
        self.turnrate.read(reader);
        self.weapon_ammo.read(reader);

        self.direction.read(reader);
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
