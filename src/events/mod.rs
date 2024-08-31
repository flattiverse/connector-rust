use crate::galaxy_hierarchy::{Cluster, Galaxy, NamedUnit, Player, Team};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

struct Inner {
    stamp: SystemTime,
    kind: FlattiverseEventKind,
}

#[repr(transparent)]
pub struct FlattiverseEvent(Box<Inner>);

impl Debug for FlattiverseEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FlattiverseEvent")
            .field(&self.0.stamp)
            .finish()
    }
}

impl From<FlattiverseEventKind> for FlattiverseEvent {
    #[inline]
    fn from(kind: FlattiverseEventKind) -> Self {
        Self(Box::new(Inner {
            stamp: crate::runtime::now(),
            kind,
        }))
    }
}

impl Display for FlattiverseEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", crate::runtime::format_date_time(self.0.stamp))?;
        write!(
            f,
            "{}",
            match &self.0.kind {
                FlattiverseEventKind::ConnectionTerminated { message } => match message.as_ref() {
                    None => "Connection terminated.",
                    Some(message) => return write!(f, "Connection terminated: {}", message),
                },
                FlattiverseEventKind::GalaxyTick => "Tick/Tack.",
                FlattiverseEventKind::PingMeasured(ping) =>
                    return write!(f, "Ping measured: {ping:?}"),

                // ---------- local events below
                FlattiverseEventKind::RespondedToPingMeasurement { challenge } => {
                    return write!(f, "Responded to Ping measurement: {challenge:?}");
                }
                FlattiverseEventKind::UpdatedGalaxy { galaxy } => {
                    return write!(f, "Updated galaxy: {:?}", galaxy.name());
                }
                FlattiverseEventKind::UpdatedTeam { team } => {
                    return write!(f, "Updated team: {:?}", &*team.name());
                }
                FlattiverseEventKind::DeactivatedTeam { team } => {
                    return write!(f, "Deactivated team: {:?}", &*team.name());
                }
                FlattiverseEventKind::UpdatedCluster { cluster } => {
                    return write!(f, "Updated cluster: {:?}", &*cluster.name());
                }
                FlattiverseEventKind::DeactivatedCluster { cluster } => {
                    return write!(f, "Deactivated cluster: {:?}", &*cluster.name());
                }
                FlattiverseEventKind::CreatedPlayer { player } => {
                    return write!(f, "Created player: {:?}", &*player.name());
                }
                FlattiverseEventKind::UpdatedPlayer { player } => {
                    return write!(f, "Updated player: {:?}", &*player.name());
                }
                FlattiverseEventKind::DeactivatedPlayer { player } => {
                    return write!(f, "Deactivated player: {:?}", &*player.name());
                }
            }
        )
    }
}

/// Specifies the various event kinds for a better match experience.
#[derive(Debug)]
pub enum FlattiverseEventKind {
    PingMeasured(Duration),
    /// Is fired when the connection to the flattiverse has been terminated
    ConnectionTerminated {
        message: Option<String>,
    },
    /// Event that is raised when the server has processed a tick.
    GalaxyTick,

    // ---------- local events below
    RespondedToPingMeasurement {
        challenge: u16,
    },
    UpdatedGalaxy {
        galaxy: Arc<Galaxy>,
    },
    UpdatedTeam {
        team: Arc<Team>,
    },
    DeactivatedTeam {
        team: Arc<Team>,
    },
    UpdatedCluster {
        cluster: Arc<Cluster>,
    },
    DeactivatedCluster {
        cluster: Arc<Cluster>,
    },
    CreatedPlayer {
        player: Arc<Player>,
    },
    UpdatedPlayer {
        player: Arc<Player>,
    },
    DeactivatedPlayer {
        player: Arc<Player>,
    },
}
