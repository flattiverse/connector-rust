
use std::sync::Arc;
use std::sync::Weak;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Player;
use Connector;
use UniverseGroup;

use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use unit::ControllableInfo;
use unit::PlayerUnitTractorbeamInfo;

use net::Packet;
use net::BinaryReader;

downcast!(PlayerUnit);
pub trait PlayerUnit : Unit {

    fn player(&self) -> &Weak<Player>;

    fn controllable_info(&self) -> &Weak<ControllableInfo>;

    fn tractorbam_info(&self) -> &Option<PlayerUnitTractorbeamInfo>;
}

pub struct PlayerUnitData {
    unit:   UnitData,
    player: Weak<Player>,
    c_info:   Weak<ControllableInfo>,
    b_info:   Option<PlayerUnitTractorbeamInfo>,
}

impl PlayerUnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader, kind: UnitKind) -> Result<PlayerUnitData, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader, kind)?;
        let player = connector.player_for(reader.read_u16()?)?;
        Ok(PlayerUnitData {
            unit,
            player: Arc::downgrade(&player),
            c_info: {
                let id = reader.read_unsigned_byte()?;
                let info = player.controllable_info(id).ok_or(Error::InvalidControllableInfo(id))?;
                Arc::downgrade(&info)
            },
            b_info:   {
                if reader.read_byte()? == 1 {
                    Some(PlayerUnitTractorbeamInfo::for_reader(reader)?)
                } else {
                    None
                }
            },
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for PlayerUnitData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for PlayerUnitData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<PlayerUnitData> + BorrowMut<PlayerUnitData> + Unit> PlayerUnit for  T {
    fn player(&self) -> &Weak<Player> {
        &self.borrow().player
    }

    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.borrow().c_info
    }

    fn tractorbam_info(&self) -> &Option<PlayerUnitTractorbeamInfo> {
        &self.borrow().b_info
    }
}