
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use crate::Error;
use crate::Connector;
use crate::UniverseGroup;
use crate::UniversalEnumerable;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::event::AnyUniverseEvent;


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

    events:     RwLock<Vec<AnyUniverseEvent>>,
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

            events:         RwLock::new(Vec::new()),
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

    /// Administrative functionality
    pub fn query_events(&self) -> Result<Vec<AnyUniverseEvent>, Error> {
        let connector = self.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;
        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        let other_group = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;
        let self_group = self.universe_group.upgrade().ok_or(Error::UniverseNotInUniverseGroup)?;

        if other_group.id() != self_group.id() {
            return Err(Error::PlayerAlreadyInAnotherUniverseGroup(other_group.id()))
        }

        let mut block = connector.block_manager().block()?;
        let mut packet = Packet::default();

        packet.set_command(0x42);
        packet.set_session(block.id());
        packet.set_path_universe_group(self_group.id());
        packet.set_path_universe(self.id);

        connector.send(&packet)?;
        block.wait()?;

        let mut vec = Vec::new();
        ::std::mem::swap(&mut *self.events.write()?, &mut vec);

        Ok(vec)
    }

    pub(crate) fn events(&self) -> &RwLock<Vec<AnyUniverseEvent>> {
        &self.events
    }
}

impl UniversalEnumerable for Universe {
    fn name(&self) -> &str {
        &self.name
    }
}