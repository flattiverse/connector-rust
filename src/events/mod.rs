mod player_unit_destroyed_reason;
pub use player_unit_destroyed_reason::*;

mod team_snapshot;
pub use team_snapshot::*;

mod cluster_snapshot;
pub use cluster_snapshot::*;

mod galaxy_settings_snapshot;
pub use galaxy_settings_snapshot::*;

use crate::galaxy_hierarchy::{
    Cluster, ClusterId, Controllable, ControllableInfo, Galaxy, Identifiable, Player, Score, Team,
};
use crate::unit::{Unit, UnitExt, UnitKind};
use crate::{SubsystemSlot, SubsystemStatus, Vector};
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
        macro_rules! log_change {
            ($append_state:expr, $current:ident, $before:ident, $name:ident) => {
                log_change!(
                    $append_state,
                    { $current.$name() },
                    { $before.$name },
                    $name
                );
            };
            ($append_state:expr, $current:expr, $before:expr, $name:ident) => {
                if $current != $before {
                    if $append_state {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}={}->{}", stringify!($name), $before, $current)?;
                    $append_state = true;
                }
            };
            ($append_state:expr, $current:ident, $before:ident, debug $name:ident) => {
                log_change!(
                    $append_state,
                    { $current.$name() },
                    { $before.$name },
                    debug $name
                );
            };
            ($append_state:expr, $current:expr, $before:expr, debug $name:ident) => {
                if $current != $before {
                    if $append_state {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}={:?}->{:?}", stringify!($name), $before, $current)?;
                    $append_state = true;
                }
            };
        }

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

                log_change!(appended_at_least_one_change, &*team.name(), before.name, name);
                log_change!(appended_at_least_one_change, team, before, red);
                log_change!(appended_at_least_one_change, team, before, green);
                log_change!(appended_at_least_one_change, team, before, blue);

                if !appended_at_least_one_change {
                    write!(f, ", without effective field changes.")?;
                }

                Ok(())
            }
            FlattiverseEventKind::TeamScoreUpdated { team, before } => {
                write!(
                    f,
                    "Team score updated: {:?}, kills={}-{}, deaths={}-{}, mission={}-{}.",
                    team.id(),
                    before.kills(),
                    team.score().kills(),
                    before.deaths(),
                    team.score().deaths(),
                    before.mission(),
                    team.score().mission(),
                )
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

            FlattiverseEventKind::ClusterCreated { cluster } => write!(
                f,
                "Cluster created: {:?}, name={:?}, active={}, start={}, respawn={}",
                cluster.id(),
                &*cluster.name(),
                cluster.active(),
                cluster.start(),
                cluster.respawn(),
            ),
            FlattiverseEventKind::ClusterUpdated { cluster, before } => {
                write!(f, "Cluster updated: id={:?}", cluster.id())?;
                let mut appended_at_least_one_change = false;

                log_change!(appended_at_least_one_change, &*cluster.name(), before.name, name);
                log_change!(appended_at_least_one_change, cluster, before, active);
                log_change!(appended_at_least_one_change, cluster, before, start);
                log_change!(appended_at_least_one_change, cluster, before, respawn);

                if !appended_at_least_one_change {
                    write!(f, ", without effective field changes.")?;
                }

                Ok(())
            }
            FlattiverseEventKind::ClusterRemoved { cluster } => write!(
                f,
                "Cluster removed: {:?}, name={:?}, active={}, start={}, respawn={}",
                cluster.id(),
                &*cluster.name(),
                cluster.active(),
                cluster.start(),
                cluster.respawn(),
            ),

            FlattiverseEventKind::GalaxySettingsUpdated { galaxy, before } => {
                if let Some(before) = before {
                    write!(f, "Galaxy settings updated: ")?;
                    let mut appended_at_least_one_change = false;

                    log_change!(appended_at_least_one_change, galaxy, before, debug game_mode);
                    log_change!(appended_at_least_one_change, &*galaxy.name(), before.name, name);
                    log_change!(appended_at_least_one_change, &*galaxy.description(), before.description, description);
                    log_change!(appended_at_least_one_change, galaxy, before, max_players);
                    log_change!(appended_at_least_one_change, galaxy, before, max_spectators);
                    log_change!(appended_at_least_one_change, galaxy, before, galaxy_max_total_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, galaxy_max_classic_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, galaxy_max_new_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, galaxy_max_bases);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_total_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_classic_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_new_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_bases);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_total_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_classic_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_new_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_bases);
                    log_change!(appended_at_least_one_change, galaxy, before, maintenance);

                    if !appended_at_least_one_change {
                        write!(f, ", without effective field changes.")?;
                    }
                } else {
                    write!(
                        f,
                        "Galaxy settings initialized: game_mode={:?} name={:?},  description={:?},  max_players={:?},  max_spectators={:?},  galaxy_max_total_ships={:?},  galaxy_max_classic_ships={:?},  galaxy_max_new_ships={:?},  galaxy_max_bases={:?},  team_max_total_ships={:?},  team_max_classic_ships={:?},  team_max_new_ships={:?},  team_max_bases={:?},  player_max_total_ships={:?},  player_max_classic_ships={:?},  player_max_new_ships={:?},  player_max_bases={:?}",
                        galaxy.game_mode(),
                        &*galaxy.name(),
                        &*galaxy.description(),
                        galaxy.max_players(),
                        galaxy.max_spectators(),
                        galaxy.galaxy_max_total_ships(),
                        galaxy.galaxy_max_classic_ships(),
                        galaxy.galaxy_max_new_ships(),
                        galaxy.galaxy_max_bases(),
                        galaxy.team_max_total_ships(),
                        galaxy.team_max_classic_ships(),
                        galaxy.team_max_new_ships(),
                        galaxy.team_max_bases(),
                        galaxy.player_max_total_ships(),
                        galaxy.player_max_classic_ships(),
                        galaxy.player_max_new_ships(),
                        galaxy.player_max_bases(),
                    )?;
                }

                Ok(())
            }

            FlattiverseEventKind::RespondedToPingMeasurement { challenge } => write!(
                f,
                "Responded to Ping measurement: {challenge:?}"
            ),
            FlattiverseEventKind::PlayerUpdated { player } => write!(
                f,
                "Updated player: {:?}", &*player.name()
            ),
            FlattiverseEventKind::PlayerJoined { player } => write!(
                f,
                "{:?} joined the galaxy with team {:?} as {:?}",
                &*player.name(),
                &*player.team().name(),
                player.kind()
            ),
            FlattiverseEventKind::PlayerParted { player } => write!(
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
                    PlayerUnitDestroyedReason::ByClusterRemoval => "got destroyed because its cluster was removed",
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
            FlattiverseEventKind::UnitAdded { unit } => {
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
                    }
                }
            }
            FlattiverseEventKind::UnitUpdated { unit } => {
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
                    }
                }
            }
            FlattiverseEventKind::UnitRemoved { unit } => {
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
                    }
                }
            }
            FlattiverseEventKind::UnitAlteredByAdmin { cluster, name } => write!(f, "Unit altered by admin: {cluster:?}, name={name:?}"),


            FlattiverseEventKind::BatterySubsystem { controllable, slot, status, current, consumed_this_tick } => {
                write!(f, "Battery subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current:?}, consumed={consumed_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::EnergyCellSubsystem { controllable, slot, status, collected_this_tick } => {
                write!(f, "Battery subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, collected={collected_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::ScannerSubsystem { controllable, slot, status, active, current_width, current_length, current_angle, target_width, target_length, target_angle, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Engine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, active={active:?}, current_width={current_width:?}, current_length={current_length:?}, current_angle={current_angle:?}, target_width={target_width:?}, target_length={target_length:?}, target_angle={target_angle:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::ClassicShipEngineSubsystem { controllable, slot, status, current, target, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Engine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current:?}, target={target:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::ShotWeaponSubsystem { controllable, slot, status, relative_movement, ticks, load, damage, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Engine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, , relative_movement={relative_movement:?}, ticks={ticks:?}, load={load:?}, damage={damage:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }

            FlattiverseEventKind::PlayerScoreUpdated { player, before } => {
                write!(
                    f,
                    "Player score updated: {:?}, name={:?}, kills={}->{}, deaths={}->{}, mission={}->{}.",
                    player.id(),
                    player.name(),
                    before.kills(),
                    player.score().kills(),
                    before.deaths(),
                    player.score().deaths(),
                    before.mission(),
                    player.score().mission(),
                )
            }

            FlattiverseEventKind::CompiledWithMessage { message, .. } => write!(f, "{message}"),
        }
    }
}

/// Specifies the various event kinds for a better match experience.
#[derive(Debug)]
pub enum FlattiverseEventKind {
    /// A player has joined the galaxy
    PlayerJoined {
        /// The player this event handles.
        player: Arc<Player>,
    },
    /// Is raised when a player score has been updated.
    PlayerScoreUpdated {
        /// The player this event handles.
        player: Arc<Player>,
        /// The player score before the update.
        before: Score,
    },
    /// A player has parted the galaxy.
    PlayerParted {
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
        /// Reason the referenced controllable was destroyed.
        reason: PlayerUnitDestroyedReason,
    },
    /// A PlayerUnit got destroyed by collision with a neutral unit.
    ControllableInfoDestroyedByNeutralCollision {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        /// Reason the referenced controllable was destroyed.
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
        /// Reason the referenced controllable was destroyed.
        reason: PlayerUnitDestroyedReason,
        /// The PlayerUnit which destroyed the PlayerUnit in question.
        destroyed_unit: Arc<ControllableInfo>,
        /// The Player of the unit which destroyed the PlayerUnit in question.
        destroyer_player: Arc<Player>,
    },
    /// Signals that the player has closed or disposed a controllable.
    ControllableInfoClosed {
        /// The player this event handles.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// You see a new unit.
    UnitAdded {
        unit: Arc<Unit>,
    },
    /// An existing unit has been updated.
    UnitUpdated {
        unit: Arc<Unit>,
    },
    /// You don't see the unit anymore.
    UnitRemoved {
        unit: Arc<Unit>,
    },
    /// This event informs about a unit that has been altered by an admin through map editing.
    UnitAlteredByAdmin {
        /// The cluster id of the altered unit.
        cluster: ClusterId,
        /// The name of the altered unit.
        name: String,
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

    /// The galaxy settings have been updated.
    GalaxySettingsUpdated {
        /// The updated [Galaxy].
        galaxy: Arc<Galaxy>,
        before: Option<GalaxySettingsSnapshot>,
    },

    /// A team has been created.
    TeamCreated {
        /// The new [Team].
        team: Arc<Team>,
    },
    /// A team has been updated.
    TeamUpdated {
        /// The updated [Team].
        team: Arc<Team>,
        /// Team snapshot before the update.
        before: TeamSnapshot,
    },
    TeamScoreUpdated {
        team: Arc<Team>,
        before: Score,
    },
    /// A team has been removed.
    TeamRemoved {
        /// The removed [Team].
        team: Arc<Team>,
    },

    /// A cluster has been created.
    ClusterCreated {
        /// The new [Cluster].
        cluster: Arc<Cluster>,
    },
    /// A cluster has been updated.
    ClusterUpdated {
        /// The updated [Cluster].
        cluster: Arc<Cluster>,
        /// Cluster snapshot before the update.
        before: ClusterSnapshot,
    },
    /// A cluster has been removed.
    ClusterRemoved {
        /// The removed [Cluster].
        cluster: Arc<Cluster>,
    },

    // ------------------- ControllableSubsystemEvents -------------------
    /// Update of a battery subsystem on your own controllable.
    BatterySubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// The status for the current server tick.
        status: SubsystemStatus,
        /// The current stored amount
        current: f32,
        /// The amount consumed during the current server tick.
        consumed_this_tick: f32,
    },
    /// Update of an energy-cell subsystem on your own controllable.
    EnergyCellSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// The status for the current server tick.
        status: SubsystemStatus,
        /// The amount collected during the current server tick.
        collected_this_tick: f32,
    },
    /// Update of a scanner subsystem on your own controllable.
    ScannerSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// The status for the current server tick.
        status: SubsystemStatus,
        /// Whether the scanner is active.
        active: bool,
        /// The current scanner width.
        current_width: f32,
        /// The current scanner length.
        current_length: f32,
        /// The current scanner angle.
        current_angle: f32,
        /// The target scanner width.
        target_width: f32,
        /// The target scanner length.
        target_length: f32,
        /// The target scanner angle.
        target_angle: f32,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Update of an engine subsystem on your own controllable.
    ClassicShipEngineSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// The status for the current server tick.
        status: SubsystemStatus,
        /// The current applied engine vector.
        current: Vector,
        /// The configured target engine vector.
        target: Vector,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Update of a shot launcher subsystem on your own controllable.
    ShotWeaponSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// The status for the current server tick.
        status: SubsystemStatus,
        /// The shot movement processed for the current server tick.
        relative_movement: Vector,
        /// The shot lifetime processed for the current server tick.
        ticks: u16,
        /// The shot load processed for the current server tick.
        load: f32,
        /// The shot damage processed for the current server tick.
        damage: f32,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    // ------------------- ControllableSubsystemEvents -------------------
    /// Is raised when the server announces the compile profile it was built with.
    CompiledWithMessage {
        /// The maximum amount of players supported by this server binary.
        max_players_supported: u8,
        /// The compile symbol that selected the server profile.
        symbol: Arc<String>,
        /// A user-facing message describing the compile profile.
        message: String,
    },

    // ---------- local events below
    PingMeasured(Duration),
    RespondedToPingMeasurement {
        challenge: u16,
    },
    PlayerUpdated {
        player: Arc<Player>,
    },
}
