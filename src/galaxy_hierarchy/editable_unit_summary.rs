use crate::unit::UnitKind;

/// Summary entry of one editable unit in a cluster.
#[derive(Debug, Clone)]
pub struct EditableUnitSummary {
    name: String,
    kind: UnitKind,
}

impl EditableUnitSummary {
    pub fn new(name: String, kind: UnitKind) -> Self {
        Self { name, kind }
    }

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
