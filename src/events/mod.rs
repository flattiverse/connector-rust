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
        f.debug_struct("FlattiverseEvent")
            .field("stamp", &self.0.stamp)
            .field("kind", &self.0.kind)
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
                FlattiverseEventKind::GalaxyTick { tick } => {
                    return write!(f, "Tick/Tack #{}", tick);
                }
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
                FlattiverseEventKind::UpdatedPlayer { player } => {
                    return write!(f, "Updated player: {:?}", &*player.name());
                }
                FlattiverseEventKind::JoinedPlayer { player } => {
                    return write!(
                        f,
                        "{:?} joined the galaxy with team {:?} as {:?}",
                        &*player.name(),
                        &*player.team.name(),
                        player.kind
                    );
                }
                FlattiverseEventKind::PartedPlayer { player } => {
                    return write!(
                        f,
                        "{:?} parted the galaxy with team {:?} as {:?}",
                        &*player.name(),
                        &*player.team.name(),
                        player.kind
                    );
                }
                FlattiverseEventKind::GalaxyChat {
                    player,
                    destination: _,
                    message,
                } => {
                    return write!(f, "<[{}]{}> {}", &*player.team.name(), player.name, message);
                }
                FlattiverseEventKind::TeamChat {
                    player,
                    destination,
                    message,
                } => {
                    return write!(
                        f,
                        "<[{}]{}->{}> {}",
                        &*player.team.name(),
                        player.name,
                        destination.name,
                        message
                    );
                }
                FlattiverseEventKind::PlayerChat {
                    player,
                    destination,
                    message,
                } => {
                    return write!(
                        f,
                        "<[{}]{}->{}> {}",
                        &*player.team.name(),
                        player.name,
                        destination.name,
                        message
                    );
                }
            }
        )
    }
}

/// Specifies the various event kinds for a better match experience.
#[derive(Debug)]
pub enum FlattiverseEventKind {
    /// A player has joined the galaxy
    JoinedPlayer {
        /// The player this event handles.
        player: Arc<Player>,
    },
    /// A player has parted the galaxy.
    PartedPlayer {
        /// The player this event handles.
        player: Arc<Player>,
    },
    /// You received a galaxy chat message.
    GalaxyChat {
        /// The player this event handles.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Galaxy>,
        /// The message of the chat.
        message: String,
    },
    /// You received a team chat message.
    TeamChat {
        /// The player this event handles.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Player>,
        /// The message of the chat.
        message: String,
    },
    /// You received a private message of a team member.
    PlayerChat {
        /// The player this event handles.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Player>,
        /// The message of the chat.
        message: String,
    },
    /// The connection has been terminated.
    ConnectionTerminated {
        message: Option<String>,
    },
    /// A tick happened.
    GalaxyTick {
        tick: i32,
    },

    // ---------- local events below
    PingMeasured(Duration),
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
    UpdatedPlayer {
        player: Arc<Player>,
    },
}
