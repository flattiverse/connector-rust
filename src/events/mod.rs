mod player_unit_destroyed_reason;
pub use player_unit_destroyed_reason::*;

mod team_snapshot;
pub use team_snapshot::*;

use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, Galaxy, Identifiable, NamedUnit, Player, Team,
};
use crate::unit::{Unit, UnitKind};
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

struct Inner {
    stamp: SystemTime,
    kind: FlattiverseEventKind,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct FlattiverseEvent(Arc<Inner>);

impl FlattiverseEvent {
    #[inline]
    pub fn timestamp(&self) -> SystemTime {
        self.0.stamp
    }

    #[inline]
    pub fn kind(&self) -> &FlattiverseEventKind {
        &self.0.kind
    }
}

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
        Self(Arc::new(Inner {
            stamp: crate::runtime::now(),
            kind,
        }))
    }
}

impl Display for FlattiverseEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", crate::runtime::format_date_time(self.0.stamp))?;
        match &self.0.kind {
            FlattiverseEventKind::ConnectionTerminated { message } => match message.as_ref() {
                None => write!(f, "Connection terminated."),
                Some(message) => return write!(f, "Connection terminated: {}", message),
            },
            FlattiverseEventKind::GalaxyTick { tick } => write!(
                f,
                "Tick/Tack #{}", tick
            ),
            FlattiverseEventKind::PingMeasured(ping) => write!(
                f,
                "Ping measured: {ping:?}"
            ),
            FlattiverseEventKind::TeamCreated { team } => write!(
                f,
                "Team created: {:?}, name={:?}, red={}, green={}, blue={}",
                team.id(),
                &*team.name(),
                team.red(),
                team.green(),
                team.blue()
            ),
            FlattiverseEventKind::TeamUpdated { team, before } => {
                write!(f, "Team updated: id={:?}", team.id)?;
                let mut appended_at_least_one_change = false;

                if *team.name() != before.name {
                    write!(f, "name={:?}->{:?}", before.name, &*team.name())?;
                    appended_at_least_one_change = true;
                }

                if team.red() != before.red {
                    if appended_at_least_one_change {
                        write!(f, ", ")?;
                    }
                    write!(f, "red={}->{}", before.red, team.red())?;
                    appended_at_least_one_change = true;
                }

                if team.green() != before.green {
                    if appended_at_least_one_change {
                        write!(f, ", ")?;
                    }
                    write!(f, "green={}->{}", before.green, team.green())?;
                    appended_at_least_one_change = true;
                }

                if team.blue() != before.blue {
                    if appended_at_least_one_change {
                        write!(f, ", ")?;
                    }
                    write!(f, "blue={}->{}", before.blue, team.blue())?;
                    appended_at_least_one_change = true;
                }

                if !appended_at_least_one_change {
                    write!(f, ", without effective field changes.")?;
                }

                Ok(())
            }
            FlattiverseEventKind::TeamRemoved { team } => write!(
                f,
                "Team removed: {:?}, name={:?}, red={}, green={}, blue={}",
                team.id(),
                &*team.name(),
                team.red(),
                team.green(),
                team.blue()
            ),

            FlattiverseEventKind::RespondedToPingMeasurement { challenge } => write!(
                f,
                "Responded to Ping measurement: {challenge:?}"
            ),
            FlattiverseEventKind::UpdatedGalaxy { galaxy } => write!(
                f,
                "Updated galaxy: {:?}", &*galaxy.name()
            ),
            FlattiverseEventKind::UpdatedCluster { cluster } => write!(
                f,
                "Updated cluster: {:?}", &*cluster.name()
            ),
            FlattiverseEventKind::DeactivatedCluster { cluster } => write!(
                f,
                "Deactivated cluster: {:?}", &*cluster.name()
            ),
            FlattiverseEventKind::UpdatedPlayer { player } => write!(
                f,
                "Updated player: {:?}", &*player.name()
            ),
            FlattiverseEventKind::JoinedPlayer { player } => write!(
                f,
                "{:?} joined the galaxy with team {:?} as {:?}",
                &*player.name(),
                &*player.team().name(),
                player.kind()
            ),
            FlattiverseEventKind::PartedPlayer { player } => write!(
                f,
                "{:?} parted the galaxy with team {:?} as {:?}",
                &*player.name(),
                &*player.team().name(),
                player.kind()
            ),
            FlattiverseEventKind::GalaxyChat {
                player,
                destination: _,
                message,
            } => write!(
                f,
                "<[{}]{}> {}",
                &*player.team().name(),
                player.name(),
                message
            ),
            FlattiverseEventKind::TeamChat {
                player,
                destination,
                message,
            } => write!(
                f,
                "<[{}]{}->{}> {}",
                &*player.team().name(),
                player.name(),
                destination.name(),
                message
            ),
            FlattiverseEventKind::PlayerChat {
                player,
                destination,
                message,
            } => write!(
                f,
                "<[{}]{}->{}> {}",
                &*player.team().name(),
                player.name(),
                destination.name(),
                message
            ),
            FlattiverseEventKind::ControllableInfoRegistered {
                player,
                controllable,
            } => write!(
                f,
                "Player {:?} of Team {:?} registered controllable {:?} of type {:?}",
                player.name(),
                &*player.team().name(),
                controllable.name(),
                controllable.kind()
            ),
            FlattiverseEventKind::ControllableInfoContinued {
                player,
                controllable,
            } => write!(
                f,
                "Player {:?} of Team {:?} continued controllable {:?} of type {:?}",
                player.name(),
                &*player.team().name(),
                controllable.name(),
                controllable.kind()
            ),
            FlattiverseEventKind::ControllableInfoDestroyed {
                player,
                controllable,
                reason,
            } => write!(
                f,
                "Player {:?} of Team {:?} controllable {:?} of type {:?} {}.",
                player.name(),
                &*player.team().name(),
                controllable.name(),
                controllable.kind(),
                match reason {
                    PlayerUnitDestroyedReason::ByRules => "got destroyed due to applied rules",
                    PlayerUnitDestroyedReason::Suicided => "suicided",
                    _ => "got destroyed",
                }
            ),
            FlattiverseEventKind::ControllableInfoDestroyedByNeutralCollision {
                player,
                controllable,
                reason: _,
                colliders_kind,
                colliders_name,
            } => write!(
                f,
                "Player {:?} of Team {:?} controllable {:?} of type {:?} collided with a {:?} named {:?}.",
                player.name(),
                &*player.team().name(),
                controllable.name(),
                controllable.kind(),
                colliders_kind,
                colliders_name,
            ),
            FlattiverseEventKind::ControllableInfoDestroyedByPlayerUnit {
                player,
                controllable,
                reason,
                destroyed_unit,
                destroyer_player,
            } => match reason {
                PlayerUnitDestroyedReason::CollidedWithEnemyPlayerUnit => write!(
                    f,
                    "Player {:?} of Team {:?}, controllable {:?} of type {:?}, got destroyed by colliding with enemy player {:?} of Team {:?}, unit {:?} of type {:?}.",
                    player.name(),
                    &*player.team().name(),
                    controllable.name(),
                    controllable.kind(),
                    destroyer_player.name(),
                    &*destroyer_player.team().name(),
                    destroyed_unit.name(),
                    destroyed_unit.kind()
                ),
                PlayerUnitDestroyedReason::CollidedWithFriendlyPlayerUnit => write!(
                    f,
                    "Player {:?} of Team {:?}, controllable {:?} of type {:?}, got destroyed by colliding with friendly player {:?}, unit {:?} of type {:?}.",
                    player.name(),
                    &*player.team().name(),
                    controllable.name(),
                    controllable.kind(),
                    destroyer_player.name(),
                    destroyed_unit.name(),
                    destroyed_unit.kind()
                ),
                PlayerUnitDestroyedReason::ShotByEnemyPlayerUnit => write!(
                    f,
                    "Player {:?} of Team {:?}, controllable {:?} of type {:?}, wa shot by enemy player {:?} of Team {:?}, unit {:?} of type {:?}.",
                    player.name(),
                    &*player.team().name(),
                    controllable.name(),
                    controllable.kind(),
                    destroyer_player.name(),
                    &*destroyer_player.team().name(),
                    destroyed_unit.name(),
                    destroyed_unit.kind()
                ),
                PlayerUnitDestroyedReason::ShotByFriendlyPlayerUnit => write!(
                    f,
                    "Player {:?} of Team {:?}, controllable {:?} of type {:?}, wa shot by enemy player {:?}, unit {:?} of type {:?}.",
                    player.name(),
                    &*player.team().name(),
                    controllable.name(),
                    controllable.kind(),
                    destroyer_player.name(),
                    destroyed_unit.name(),
                    destroyed_unit.kind()
                ),
                _ => write!(
                    f,
                    "Player {:?} of Team {:?}, controllable {:?} of type {:?} got destroyed.",
                    player.name(),
                    &*player.team().name(),
                    controllable.name(),
                    controllable.kind(),
                )
            }
            FlattiverseEventKind::ControllableInfoClosed {
                player,
                controllable,
            } => write!(
                f,
                "Player {:?} of Team {:?} closed/disposed controllable {:?} of type {:?}",
                player.name(),
                &*player.team().name(),
                controllable.name(),
                controllable.kind()
            ),
            FlattiverseEventKind::NewUnit { unit } => {
                let cluster = unit.cluster();
                let cluster = &*cluster.name();
                let kind = unit.kind();
                let name = unit.name();
                let position = unit.position();
                let radius = unit.radius();
                let gravity = unit.gravity();
                match unit.team().upgrade() {
                    None => write!(f, "New Unit in cluster {cluster:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}."),
                    Some(team) => {
                        let team = &*team.name();
                        write!(f, "New Unit in cluster {cluster:?} and with team {team:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}.")
                    },
                }
            }
            FlattiverseEventKind::UpdatedUnit { unit } => {
                let cluster = unit.cluster();
                let cluster = &*cluster.name();
                let kind = unit.kind();
                let name = unit.name();
                let position = unit.position();
                let radius = unit.radius();
                let gravity = unit.gravity();
                match unit.team().upgrade() {
                    None => write!(f, "Updated Unit in cluster {cluster:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}."),
                    Some(team) => {
                        let team = &*team.name();
                        write!(f, "Updated Unit in cluster {cluster:?} and with team {team:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}.")
                    },
                }
            }
            FlattiverseEventKind::RemovedUnit { unit } => {
                let cluster = unit.cluster();
                let cluster = &*cluster.name();
                let kind = unit.kind();
                let name = unit.name();
                let position = unit.position();
                let radius = unit.radius();
                let gravity = unit.gravity();
                match unit.team().upgrade() {
                    None => write!(f, "Removed Unit in cluster {cluster:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}."),
                    Some(team) => {
                        let team = &*team.name();
                        write!(f, "Removed Unit in cluster {cluster:?} and with team {team:?} of Kind {kind:?} with name {name:?} on position {position:?} and with radius {radius} and gravity {gravity:.3}.")
                    },
                }
            }
        }
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
    /// A PlayerUnit has been registered
    ControllableInfoRegistered {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// A PlayerUnit did continue the game.
    ControllableInfoContinued {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// A PlayerUnit was destroyed.
    ControllableInfoDestroyed {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        reason: PlayerUnitDestroyedReason,
    },
    /// A PlayerUnit got destroyed by collision with a neutral unit.
    ControllableInfoDestroyedByNeutralCollision {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        reason: PlayerUnitDestroyedReason,
        /// The UnitKind of the unit the PlayerUnit collided with.
        colliders_kind: UnitKind,
        /// The name of the unit the PlayerUnit collided with.
        colliders_name: String,
    },
    /// A PlayerUnit got destroyed by collision with a neutral unit.
    ControllableInfoDestroyedByPlayerUnit {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        reason: PlayerUnitDestroyedReason,
        /// The PlayerUnit which destroyed the PlayerUnit in question.
        destroyed_unit: Arc<ControllableInfo>,
        /// The Player of the unit which destroyed the PlayerUnit in question.
        destroyer_player: Arc<Player>,
    },
    /// A PlayerUnit was unregistered.
    ControllableInfoClosed {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// You see a new unit.
    NewUnit {
        unit: Arc<Unit>,
    },
    /// An existing unit has been updated.
    UpdatedUnit {
        unit: Arc<Unit>,
    },
    /// You don't see the unit anymore.
    RemovedUnit {
        unit: Arc<Unit>,
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
        tick: u32,
    },

    /// Is raised when a team has been created.
    TeamCreated {
        /// The new [Team].
        team: Arc<Team>,
    },
    /// Is raised when a team has been updated.
    TeamUpdated {
        /// The updated [Team].
        team: Arc<Team>,
        /// Team snapshot before the update.
        before: TeamSnapshot,
    },
    /// Is raised when a team has been removed.
    TeamRemoved {
        /// The removed [Team].
        team: Arc<Team>,
    },

    // ---------- local events below
    PingMeasured(Duration),
    RespondedToPingMeasurement {
        challenge: u16,
    },
    UpdatedGalaxy {
        galaxy: Arc<Galaxy>,
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
