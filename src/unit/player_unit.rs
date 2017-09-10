
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;

use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use unit::ControllableInfo;
use unit::PlayerUnitTractorbeamInfo;

use net::Packet;
use net::BinaryReader;

impl_downcast!(PlayerUnit);
pub trait PlayerUnit : Unit {

    fn player(&self) -> &Weak<RwLock<Player>>;

    fn controllable_info(&self) -> &Weak<RwLock<ControllableInfo>>;

    fn tractorbam_info(&self) -> &Option<PlayerUnitTractorbeamInfo>;
}

pub struct PlayerUnitData {
    unit:   UnitData,
    player: Weak<RwLock<Player>>,
    c_info:   Option<ControllableInfo>,
    b_info:   Option<PlayerUnitTractorbeamInfo>,
}

impl PlayerUnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerUnitData, Error> {
        Ok(PlayerUnitData {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            player: connector.weak_player_for(reader.read_u16()?)?,
            c_info: {
                let player = connector.player_for(reader.read_u16()?)?;
                let player = player.read()?;
                let id = reader.read_unsigned_byte()?;
                let info = player.controllable_info(id).ok_or(Error::InvalidControllableInfo(id))?
                Arc::downgrade(info)
            },
            b_info:   {
                if reader.read_byte() == 1 {
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
    fn player(&self) -> &Weak<RwLock<Player>> {
        &self.borrow().player
    }

    fn controllable_info(&self) -> &Weak<RwLock<ControllableInfo>> {

    }

    fn tractorbam_info(&self) -> &Option<PlayerUnitTractorbeamInfo> {

    }
}