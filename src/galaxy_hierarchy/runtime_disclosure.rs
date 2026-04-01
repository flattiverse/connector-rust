use crate::galaxy_hierarchy::{RuntimeDisclosureAspect, RuntimeDisclosureLevel};
use crate::network::PacketReader;
use num_enum::FromPrimitive;
use std::fmt::Display;
use std::ops::Index;

/// Session-level runtime self-disclosure.
#[derive(Debug, Clone)]
pub struct RuntimeDisclosure {
    /// Engine-control disclosure.
    pub engine_control: RuntimeDisclosureLevel,
    /// Navigation disclosure.
    pub navigation: RuntimeDisclosureLevel,
    /// Scanner-control disclosure.
    pub scanner_control: RuntimeDisclosureLevel,
    /// Weapon-aiming disclosure.
    pub weapon_aiming: RuntimeDisclosureLevel,
    /// Weapon-target-selection disclosure
    pub weapon_target_selection: RuntimeDisclosureLevel,
    /// Resource-control disclosure.
    pub resource_control: RuntimeDisclosureLevel,
    /// Fleet-control disclosure.
    pub fleet_control: RuntimeDisclosureLevel,
    /// Mission-control disclosure.
    pub mission_control: RuntimeDisclosureLevel,
    /// Loadout-control disclosure.
    pub loadout_control: RuntimeDisclosureLevel,
    /// Chat disclosure.
    pub chat: RuntimeDisclosureLevel,
}

impl RuntimeDisclosure {
    pub(crate) fn try_read(reader: &mut dyn PacketReader) -> Option<Self> {
        let packed0 = reader.read_byte();
        let packed1 = reader.read_byte();
        let packed2 = reader.read_byte();
        let packed3 = reader.read_byte();
        let packed4 = reader.read_byte();

        Some(Self {
            engine_control: RuntimeDisclosureLevel::from_primitive(packed0 >> 4).validated()?,
            navigation: RuntimeDisclosureLevel::from_primitive(packed0 & 0x0F).validated()?,
            scanner_control: RuntimeDisclosureLevel::from_primitive(packed1 >> 4).validated()?,
            weapon_aiming: RuntimeDisclosureLevel::from_primitive(packed1 & 0x0F).validated()?,
            weapon_target_selection: RuntimeDisclosureLevel::from_primitive(packed2 >> 4)
                .validated()?,
            resource_control: RuntimeDisclosureLevel::from_primitive(packed2 & 0x0F).validated()?,
            fleet_control: RuntimeDisclosureLevel::from_primitive(packed3 >> 4).validated()?,
            mission_control: RuntimeDisclosureLevel::from_primitive(packed3 & 0x0F).validated()?,
            loadout_control: RuntimeDisclosureLevel::from_primitive(packed4 >> 4).validated()?,
            chat: RuntimeDisclosureLevel::from_primitive(packed4 & 0x0F).validated()?,
        })
    }
}

impl Index<RuntimeDisclosureAspect> for RuntimeDisclosure {
    type Output = RuntimeDisclosureLevel;

    fn index(&self, index: RuntimeDisclosureAspect) -> &Self::Output {
        match index {
            RuntimeDisclosureAspect::EngineControl => &self.engine_control,
            RuntimeDisclosureAspect::Navigation => &self.navigation,
            RuntimeDisclosureAspect::ScannerControl => &self.scanner_control,
            RuntimeDisclosureAspect::WeaponAiming => &self.weapon_aiming,
            RuntimeDisclosureAspect::WeaponTargetSelection => &self.weapon_target_selection,
            RuntimeDisclosureAspect::ResourceControl => &self.resource_control,
            RuntimeDisclosureAspect::FleetControl => &self.fleet_control,
            RuntimeDisclosureAspect::MissionControl => &self.mission_control,
            RuntimeDisclosureAspect::LoadoutControl => &self.loadout_control,
            RuntimeDisclosureAspect::Chat => &self.chat,
            RuntimeDisclosureAspect::Unknown(_) => panic!("Unable to index for {index:?}"),
        }
    }
}

impl Display for RuntimeDisclosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}",
            nibble_to_hex(u8::from(self.engine_control)),
            nibble_to_hex(u8::from(self.navigation)),
            nibble_to_hex(u8::from(self.scanner_control)),
            nibble_to_hex(u8::from(self.weapon_aiming)),
            nibble_to_hex(u8::from(self.weapon_target_selection)),
            nibble_to_hex(u8::from(self.resource_control)),
            nibble_to_hex(u8::from(self.fleet_control)),
            nibble_to_hex(u8::from(self.mission_control)),
            nibble_to_hex(u8::from(self.loadout_control)),
            nibble_to_hex(u8::from(self.chat)),
        )
    }
}

fn nibble_to_hex(value: u8) -> char {
    if value < 10 {
        char::from(b'0' + value)
    } else {
        char::from(b'A' + value - 10)
    }
}
