use crate::galaxy_hierarchy::{BuildDisclosureAspect, BuildDisclosureLevel};
use crate::network::PacketReader;
use num_enum::FromPrimitive;
use std::fmt::{Display, Formatter};
use std::ops::Index;

/// Session-level build-assistance self-disclosure.
#[derive(Debug, Clone)]
pub struct BuildDisclosure {
    /// Software-design disclosure.
    pub software_design: BuildDisclosureLevel,
    /// UI disclosure.
    pub ui: BuildDisclosureLevel,
    /// Universe-rendering disclosure.
    pub universe_rendering: BuildDisclosureLevel,
    /// Input disclosure.
    pub input: BuildDisclosureLevel,
    /// Engine-control disclosure.
    pub engine_control: BuildDisclosureLevel,
    /// Navigation disclosure.
    pub navigation: BuildDisclosureLevel,
    /// Scanner-control disclosure.
    pub scanner_control: BuildDisclosureLevel,
    /// Weapon-systems disclosure.
    pub weapon_systems: BuildDisclosureLevel,
    /// Resource-control disclosure.
    pub resource_control: BuildDisclosureLevel,
    /// Fleet-control disclosure.
    pub fleet_control: BuildDisclosureLevel,
    /// Mission-control disclosure.
    pub mission_control: BuildDisclosureLevel,
    /// Chat disclosure.
    pub chat: BuildDisclosureLevel,
}

impl BuildDisclosure {
    pub(crate) fn try_read(reader: &mut dyn PacketReader) -> Option<Self> {
        let packed0 = reader.read_byte();
        let packed1 = reader.read_byte();
        let packed2 = reader.read_byte();
        let packed3 = reader.read_byte();
        let packed4 = reader.read_byte();
        let packed5 = reader.read_byte();

        Some(Self {
            software_design: BuildDisclosureLevel::from_primitive(packed0 >> 4).validated()?,
            ui: BuildDisclosureLevel::from_primitive(packed0 & 0x0F).validated()?,
            universe_rendering: BuildDisclosureLevel::from_primitive(packed1 >> 4).validated()?,
            input: BuildDisclosureLevel::from_primitive(packed1 & 0x0F).validated()?,
            engine_control: BuildDisclosureLevel::from_primitive(packed2 >> 4).validated()?,
            navigation: BuildDisclosureLevel::from_primitive(packed2 & 0x0F).validated()?,
            scanner_control: BuildDisclosureLevel::from_primitive(packed3 >> 4).validated()?,
            weapon_systems: BuildDisclosureLevel::from_primitive(packed3 & 0x0F).validated()?,
            resource_control: BuildDisclosureLevel::from_primitive(packed4 >> 4).validated()?,
            fleet_control: BuildDisclosureLevel::from_primitive(packed4 & 0x0F).validated()?,
            mission_control: BuildDisclosureLevel::from_primitive(packed5 >> 4).validated()?,
            chat: BuildDisclosureLevel::from_primitive(packed5 & 0x0F).validated()?,
        })
    }
}

impl Index<BuildDisclosureAspect> for BuildDisclosure {
    type Output = BuildDisclosureLevel;

    fn index(&self, index: BuildDisclosureAspect) -> &Self::Output {
        match index {
            BuildDisclosureAspect::SoftwareDesign => &self.software_design,
            BuildDisclosureAspect::UI => &self.ui,
            BuildDisclosureAspect::UniverseRendering => &self.universe_rendering,
            BuildDisclosureAspect::Input => &self.input,
            BuildDisclosureAspect::EngineControl => &self.engine_control,
            BuildDisclosureAspect::Navigation => &self.navigation,
            BuildDisclosureAspect::ScannerControl => &self.scanner_control,
            BuildDisclosureAspect::WeaponSystems => &self.weapon_systems,
            BuildDisclosureAspect::ResourceControl => &self.resource_control,
            BuildDisclosureAspect::FleetControl => &self.fleet_control,
            BuildDisclosureAspect::MissionControl => &self.mission_control,
            BuildDisclosureAspect::Chat => &self.chat,
            BuildDisclosureAspect::Unknown(_) => panic!("Unable to index for {index:?}"),
        }
    }
}

impl Display for BuildDisclosure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            nibble_to_hex(u8::from(self.software_design)),
            nibble_to_hex(u8::from(self.ui)),
            nibble_to_hex(u8::from(self.universe_rendering)),
            nibble_to_hex(u8::from(self.input)),
            nibble_to_hex(u8::from(self.engine_control)),
            nibble_to_hex(u8::from(self.navigation)),
            nibble_to_hex(u8::from(self.scanner_control)),
            nibble_to_hex(u8::from(self.weapon_systems)),
            nibble_to_hex(u8::from(self.resource_control)),
            nibble_to_hex(u8::from(self.fleet_control)),
            nibble_to_hex(u8::from(self.mission_control)),
            nibble_to_hex(u8::from(self.chat)),
        )
    }
}

fn nibble_to_hex(value: u8) -> char {
    if value < 10 {
        char::from('0' as u8 + value)
    } else {
        char::from('A' as u8 + value - 10)
    }
}
