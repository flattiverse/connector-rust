use crate::unit::DynamicShotMagazineSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of an interceptor magazine on a scanned player unit.
/// Its semantics are identical to [`DynamicShotMagazineSubsystemInfo`], but the ammunition consists
/// of interceptors instead of shots.
#[derive(Debug, Clone, Default)]
pub struct DynamicInterceptorMagazineSubsystemInfo(DynamicShotMagazineSubsystemInfo);

impl Deref for DynamicInterceptorMagazineSubsystemInfo {
    type Target = DynamicShotMagazineSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
