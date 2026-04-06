use crate::unit::DynamicInterceptorMagazineSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of a static interceptor magazine subsystem on a scanned modern ship.
#[derive(Debug, Clone, Default)]
pub struct StaticInterceptorMagazineSubsystemInfo(DynamicInterceptorMagazineSubsystemInfo);

impl Deref for StaticInterceptorMagazineSubsystemInfo {
    type Target = DynamicInterceptorMagazineSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
