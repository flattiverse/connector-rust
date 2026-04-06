use crate::unit::ClassicRailgunSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of a modern railgun subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ModernRailgunSubsystemInfo(ClassicRailgunSubsystemInfo);

impl Deref for ModernRailgunSubsystemInfo {
    type Target = ClassicRailgunSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
