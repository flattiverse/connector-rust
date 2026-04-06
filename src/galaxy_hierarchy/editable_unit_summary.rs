use crate::unit::UnitKind;

/// Summary entry of one editable unit in a cluster.
#[derive(Debug, Clone)]
pub struct EditableUnitSummary {
    pub(crate) name: String,
    pub(crate) kind: UnitKind,
}

impl EditableUnitSummary {
    /// Name of the editable unit inside its cluster.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Concrete unit kind of the editable unit.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        self.kind
    }
}
