use std::fmt::Display;

/// Final state of one gate after a switch action.
#[derive(Debug, Clone)]
pub struct GateStateChange {
    pub(crate) gate_name: String,
    pub(crate) closed: bool,
}

impl GateStateChange {
    #[inline]
    pub fn gate_name(&self) -> &str {
        &self.gate_name
    }

    #[inline]
    pub fn closed(&self) -> bool {
        self.closed
    }
}

impl Display for GateStateChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={}",
            self.gate_name,
            if self.closed { "Closed" } else { "Open" }
        )
    }
}
