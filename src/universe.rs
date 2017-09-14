
use std::sync::Arc;
use std::sync::Weak;

use Error;
use Connector;
use UniverseGroup;
use UniversalEnumerable;

use net::Packet;
use net::BinaryReader;


/// This implementation does not provide administrative
/// functionality due to time limitation while porting.
pub struct Universe {
    universe_group: Weak<UniverseGroup>,
    connector:      Weak<Connector>,

    id:         u8,
    name:       String,
    description:String,
    start:      bool,
    respawn:    bool,
}

impl Universe {
    pub fn from_reader(universe_group: &Arc<UniverseGroup>, packet: &Packet, reader: &mut BinaryReader) -> Result<Universe, Error> {
        Ok(Universe {
            universe_group: Arc::downgrade(universe_group),
            connector:      universe_group.connector().clone(),
            id:             packet.path_universe(),
            name:           reader.read_string()?,
            description:    reader.read_string()?,
            start:          reader.read_bool()?,
            respawn:        reader.read_bool()?,
        })
    }

    pub fn universe_group(&self) -> &Weak<UniverseGroup> {
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