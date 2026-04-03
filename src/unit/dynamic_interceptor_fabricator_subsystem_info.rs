use crate::unit::DynamicShotFabricatorSubsystemInfo;
use std::ops::Deref;

/// Visible snapshot of an interceptor fabricator on a scanned player unit.
/// Its semantics are identical to [`DynamicShotFabricatorSubsystemInfo`], but it fabricates
/// interceptor ammunition instead of shot ammunition.
#[derive(Debug, Clone, Default)]
pub struct DynamicInterceptorFabricatorSubsystemInfo(DynamicShotFabricatorSubsystemInfo);

impl Deref for DynamicInterceptorFabricatorSubsystemInfo {
    type Target = DynamicShotFabricatorSubsystemInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
