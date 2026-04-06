use crate::unit::DynamicShotMagazineSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of a static shot magazine subsystem on a scanned modern ship.
#[derive(Debug, Clone, Default)]
pub struct StaticShotMagazineSubsystemInfo(DynamicShotMagazineSubsystemInfo);

impl Deref for StaticShotMagazineSubsystemInfo {
    type Target = DynamicShotMagazineSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
