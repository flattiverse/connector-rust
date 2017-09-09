
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use unit::Unit;
use net::Packet;
use net::BinaryReader;
use event::UniverseEvent;


/// This implementation does not provide administrative
/// functionality due to time limitation while porting.
pub struct Universe {
    universe_group: Weak<RwLock<UniverseGroup>>,
    connector:      Weak<Connector>,

    id:         u8,
    name:       String,
    description:String,
    start:      bool,
    respawn:    bool,
}

impl Universe {
    pub fn new(universe_group: &Arc<RwLock<UniverseGroup>>, packet: &Packet) -> Result<Universe, Error> {
        let reader = &mut packet.read() as &mut BinaryReader;
        Ok(Universe {
            universe_group: Arc::downgrade(universe_group),
            connector:      universe_group.read()?.connector().clone(),
            id:             packet.path_universe(),
            name:           reader.read_string()?,
            description:    reader.read_string()?,
            start:          reader.read_bool()?,
            respawn:        reader.read_bool()?,
        })
    }

    pub fn universe_group(&self) -> &Weak<RwLock<UniverseGroup>> {
        &self.universe_group
    }

    pub fn connector(&self) -> &Weak<Connector> {
        &self.connector
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    /// Whether this is the start [Universe] of
    /// its [UniverseGroup]
    pub fn start(&self) -> bool {
        self.start
    }

    /// Whether this [Universe] allows respawning
    pub fn respawn(&self) -> bool {
        self.respawn
    }
}

impl UniversalEnumerable for Universe {
    fn name(&self) -> &str {
        &self.name
    }
}