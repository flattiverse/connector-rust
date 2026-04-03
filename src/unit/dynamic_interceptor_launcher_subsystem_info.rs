use crate::unit::DynamicShotLauncherSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of a configurable interceptor launcher on a scanned player unit.
/// Its semantics are identical to [`DynamicShotLauncherSubsystemInfo`], but the launched projectile
/// type is an interceptor instead of a shot.
#[derive(Debug, Clone, Default)]
pub struct DynamicInterceptorLauncherSubsystemInfo(DynamicShotLauncherSubsystemInfo);

impl Deref for DynamicInterceptorLauncherSubsystemInfo {
    type Target = DynamicShotLauncherSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
