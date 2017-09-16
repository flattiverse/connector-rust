
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::ControllableData;

pub struct Ship {
    controllable: ControllableData,
}

impl Ship {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Ship, Error>  {
        Ok(Ship {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }

    /// Allows your ship to continue playing after it was destroyed.
    /// This is only possible for this ship. Everything else has to be rebuilt!
    /// Note: In C# this is called 'Continue()'
    pub fn proceed(&self) -> Result<(), Error> {
        let connector = self.controllable.connector.upgrade().ok_or(Error::ConnectorNotAvailable)?;
        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        let _ = player.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;

        if self.controllable.mutable.read()?.pending_shutdown {
            return Err(Error::PendingShutdown);
        }

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        let mut packet = Packet::new();

        packet.set_command(0x81);
        packet.set_session(block.id());
        packet.set_path_ship(self.controllable.id);

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }
}

impl AsRef<ControllableData> for Ship {
    fn as_ref(&self) -> &ControllableData {
        &self.controllable
    }
}