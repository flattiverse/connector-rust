
use std::sync::Arc;

use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;

use controllable::Controllable;
use controllable::ControllableData;

use unit::UnitKind;

use net::Packet;
use net::BinaryReader;

impl_downcast!(Ship);
pub trait Ship : Controllable {

    fn kind(&self) -> UnitKind {
        UnitKind::PlayerShip
    }

    /// Allows your ship to continue playing after it was destroyed.
    /// This is only possible for this ship. Everything else has to be rebuilt!
    /// Note: In C# this is called 'Continue()'
    fn proceed(&self) -> Result<(), Error> {
        let connector = self.connector().upgrade().ok_or(Error::ConnectorNotAvailable)?;
        let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
        let _ = player.read()?.universe_group().upgrade().ok_or(Error::PlayerNotInUniverseGroup)?;

        if self.pending_shutdown() {
            return Err(Error::PendingShutdown);
        }

        let block = connector.block_manager().block()?;
        let mut block = block.lock()?;

        let mut packet = Packet::new();

        packet.set_command(0x81);
        packet.set_session(block.id());
        packet.set_path_ship(self.id());

        connector.send(&packet)?;
        block.wait()?;
        Ok(())
    }
}

pub struct ShipData {
    data: ControllableData
}

impl ShipData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<ShipData, Error>  {
        Ok(ShipData {
            data: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// implicitly 'extend' Controllable
impl Borrow<ControllableData> for ShipData {
    fn borrow(&self) -> &ControllableData {
        &self.data
    }
}
impl BorrowMut<ControllableData> for ShipData {
    fn borrow_mut(&mut self) -> &mut ControllableData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<ShipData> + BorrowMut<ShipData> + Controllable> Ship for T {

}