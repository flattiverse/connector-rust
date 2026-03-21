use crate::galaxy_hierarchy::Controllable;
use crate::{SubsystemSlot, SubsystemStatus};
use std::sync::Arc;

pub trait SubsystemExt {
    /// The [Controllable] this subsystem belongs to.
    fn controllable(&self) -> Arc<Controllable>;

    /// A human-readable subsystem name.
    fn name(&self) -> &str;

    /// Whether the controllable actually provides this subsystem.
    fn exists(&self) -> bool;

    /// The concrete slot this subsystem occupies.
    fn slot(&self) -> SubsystemSlot;

    /// The latest status reported by the server.
    fn status(&self) -> SubsystemStatus;
}
