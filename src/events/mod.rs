mod player_unit_destroyed_reason;
pub use player_unit_destroyed_reason::*;

mod team_snapshot;
pub use team_snapshot::*;

mod cluster_snapshot;
pub use cluster_snapshot::*;

mod galaxy_settings_snapshot;
pub use galaxy_settings_snapshot::*;

mod gate_state_change;
pub use gate_state_change::*;

use crate::galaxy_hierarchy::{
    Cluster, ClusterId, Controllable, ControllableInfo, Galaxy, Player, RailgunDirection, Score,
    Team, Tournament,
};
use crate::unit::{Unit, UnitKind};
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
    /// [`SystemTime`] timestamp when this event instance was created inside the connector.
    /// This is a local connector timestamp, not the authoritative server tick time.
    #[inline]
    pub fn timestamp(&self) -> SystemTime {
        self.0.stamp
    }

    /// Connector-side event classification used for event dispatch and switch statements.
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
                Some(message) => write!(f, "Connection terminated: {}", message),
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
                write!(f, "Team updated: id={:?}", team.id())?;
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
                    "Team score updated: {:?}, player_kills={}-{}, player_deaths={}-{}, friendly_kills={}-{}, friendly_deaths={}-{}, npc_kills={}-{}, npc_deaths={}-{}, neutral_deaths={}-{}, mission={}-{}.",
                    team.id(),
                    before.player_kills(), team.score().player_kills(),
                    before.player_deaths(), team.score().player_deaths(),
                    before.friendly_kills(), team.score().friendly_kills(),
                    before.friendly_deaths(), team.score().friendly_deaths(),
                    before.npc_kills(), team.score().npc_kills(),
                    before.npc_deaths(), team.score().npc_deaths(),
                    before.neutral_deaths(), team.score().neutral_deaths(),
                    before.mission(), team.score().mission(),
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

            FlattiverseEventKind::TournamentCreated { tournament } => write!(
                f,
                "Tournament created: {:?} in stage {:?}", tournament.mode(), tournament.stage(),
            ),
            FlattiverseEventKind::TournamentUpdated { old_tournament, new_tournament } => write!(
                f,
                "Tournament updated: {:?} => {:?}", old_tournament.stage(), new_tournament.stage(),
            ),
            FlattiverseEventKind::TournamentRemoved { tournament } => write!(
                f,
                "Tournament removed from stage {:?}", tournament.stage(),
            ),
            FlattiverseEventKind::TournamentMessage { message } => write!(f, "{message}"),

            FlattiverseEventKind::PowerUpCollected { controllable, power_up_kind, power_up_name, amount, applied_amount } => write!(
                f,
                "PowerUp collected: {:?}, kind={power_up_kind:?}, name={power_up_name}, amount={amount}, applied_amount={applied_amount}",
                controllable.name(),
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
                    log_change!(appended_at_least_one_change, galaxy, before, galaxy_max_modern_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_total_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_classic_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, team_max_modern_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_total_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_classic_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, player_max_modern_ships);
                    log_change!(appended_at_least_one_change, galaxy, before, maintenance);
                    log_change!(appended_at_least_one_change, galaxy, before, requires_self_disclosure);

                    if !appended_at_least_one_change {
                        write!(f, ", without effective field changes.")?;
                    }
                } else {
                    write!(
                        f,
                        "Galaxy settings initialized: game_mode={:?} name={:?}, description={:?}, max_players={:?}, max_spectators={:?}, galaxy_max_total_ships={:?}, galaxy_max_classic_ships={:?}, galaxy_max_new_ships={:?}, team_max_total_ships={:?}, team_max_classic_ships={:?}, team_max_new_ships={:?}, player_max_total_ships={:?}, player_max_classic_ships={:?}, player_max_new_ships={:?}.",
                        galaxy.game_mode(),
                        &*galaxy.name(),
                        &*galaxy.description(),
                        galaxy.max_players(),
                        galaxy.max_spectators(),
                        galaxy.galaxy_max_total_ships(),
                        galaxy.galaxy_max_classic_ships(),
                        galaxy.galaxy_max_modern_ships(),
                        galaxy.team_max_total_ships(),
                        galaxy.team_max_classic_ships(),
                        galaxy.team_max_modern_ships(),
                        galaxy.player_max_total_ships(),
                        galaxy.player_max_classic_ships(),
                        galaxy.player_max_modern_ships(),
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
                "Updated player: {:?}", player.name()
            ),
            FlattiverseEventKind::PlayerJoined { player } => write!(
                f,
                "{:?} joined the galaxy with team {:?} as {:?}",
                player.name(),
                &*player.team().name(),
                player.kind()
            ),
            FlattiverseEventKind::PlayerDisconnected { player } => write!(
                f,
                "{:?} disconnected from the galaxy while cleanup is still pending team.",
                player.name(),
            ),
            FlattiverseEventKind::PlayerParted { player } => write!(
                f,
                "{:?} parted the galaxy with team {:?} as {:?}",
                player.name(),
                &*player.team().name(),
                player.kind()
            ),
            FlattiverseEventKind::FlagScoredChat {
                player, controllable_info, flag_team, flag_name
            } => write!(
                f,
                "[SYSTEM] [{}] {} / {} scored flag {flag_name:?} of team {}.",
                &*player.team().name(),
                player.name(),
                controllable_info.name(),
                &*flag_team.name(),
            ),
            FlattiverseEventKind::DominationPointScoredChat {
                team, domination_point_name
            } => write!(
                f,
                "[SYSTEM] Team {} scored domination point {domination_point_name:?}.",
                &*team.name(),
            ),
            FlattiverseEventKind::OwnFlagHitChat {
                player, controllable_info, flag_team, flag_name
            } => write!(
                f,
                "[SYSTEM] [{}] {} / {} hit the own flag {flag_name:?} of team {}. The other teams gladly take the free point.",
                &*player.team().name(),
                player.name(),
                controllable_info.name(),
                &*flag_team.name(),
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
            FlattiverseEventKind::MissionTargetHitChat {
                player, controllable_info, mission_target_sequence
            } => write!(
                f,
                "[SYSTEM] [{}] Ship {} of player {} hit mission target of sequence #{mission_target_sequence}.",
                &*player.team().name(),
                controllable_info.name(),
                player.name(),
            ),
            FlattiverseEventKind::FlagReactivatedChat {
                flag_team, flag_name
            } => write!(
                f,
                "[SYSTEM] Flag {flag_name:?} of team {} is active again.",
                &*flag_team.name(),
            ),
            FlattiverseEventKind::GateSwitched {
                cluster, invoker_player, invoker_controllable_info, switch_name, gates
            } => {
                let gates = gates.iter().map(|g| g.to_string()).collect::<Vec<String>>().join(", ");
                write!(
                    f,
                    "Switch {switch_name} in cluster {} changed {}",
                    &*cluster.name(),
                    if gates.is_empty() {
                        "no linked gates"
                    } else {
                        &gates
                    }
                )?;
                if let (Some(player), Some(controllable)) = (invoker_player, invoker_controllable_info) {
                    write!(f, " by {} / {}.", player.name(), controllable.name())
                } else {
                    write!(f, ".")
                }
            },
            FlattiverseEventKind::GateRestored { cluster, gate_name, closed } => {
                write!(
                    f,
                    "Gate {gate_name} in cluster {} auto-restored to {}.",
                    &*cluster.name(),
                    if *closed {
                        "closed"
                    } else {
                        "open"
                    }
                )
            }
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
                    PlayerUnitDestroyedReason::Rebuilding => "went offline for a subsystem rebuild",
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
            FlattiverseEventKind::ControllableInfoScoreUpdated { player, controllable, before } => {
                write!(
                    f,
                    "Controllable score updated: {:?}, {:?}, player_kills={}-{}, player_deaths={}-{}, friendly_kills={}-{}, friendly_deaths={}-{}, npc_kills={}-{}, npc_deaths={}-{}, neutral_deaths={}-{}, mission={}-{}.",
                    player.id(),
                    controllable.id(),
                    before.player_kills(), controllable.score().player_kills(),
                    before.player_deaths(), controllable.score().player_deaths(),
                    before.friendly_kills(), controllable.score().friendly_kills(),
                    before.friendly_deaths(), controllable.score().friendly_deaths(),
                    before.npc_kills(), controllable.score().npc_kills(),
                    before.npc_deaths(), controllable.score().npc_deaths(),
                    before.neutral_deaths(), controllable.score().neutral_deaths(),
                    before.mission(), controllable.score().mission(),
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


            FlattiverseEventKind::ArmorSubsystem { controllable, slot, status, reduction, blocked_direct_damage_this_tick, blocked_radiation_damage_this_tick } => {
                write!(f, "Battery subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, reduction={reduction}, blocked_direct_damage_this_tick={blocked_direct_damage_this_tick}, blocked_radiation_damage_this_tick={blocked_radiation_damage_this_tick}", controllable.name())
            }
            FlattiverseEventKind::BatterySubsystem { controllable, slot, status, current, consumed_this_tick } => {
                write!(f, "Battery subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current:?}, consumed={consumed_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::CargoSubsystem { controllable, slot, status, current_metal, current_carbon, current_hydrogen, current_silicon, current_nebula, nebula_hue } => {
                write!(f, "Battery subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current_metal={current_metal}, current_carbon={current_carbon}, current_hydrogen={current_hydrogen}, current_silicon={current_silicon}, current_nebula={current_nebula}, nebula_hue={nebula_hue}", controllable.name())
            }
            FlattiverseEventKind::EnergyCellSubsystem { controllable, slot, status, collected_this_tick } => {
                write!(f, "Energy cell subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, collected={collected_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::DynamicScannerSubsystem { controllable, slot, status, active, current_width, current_length, current_angle, target_width, target_length, target_angle, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Dynamic scanner subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, active={active:?}, current_width={current_width:?}, current_length={current_length:?}, current_angle={current_angle:?}, target_width={target_width:?}, target_length={target_length:?}, target_angle={target_angle:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::ClassicShipEngineSubsystem { controllable, slot, status, current, target, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Classic ship engine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current:?}, target={target:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::HullSubsystem { controllable, slot, status, current } => {
                write!(f, "Hull subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current:?}", controllable.name())
            }
            FlattiverseEventKind::NebulaCollectorSubsystem { controllable, slot, status, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, collected_this_tick, collected_hue_this_tick } => {
                write!(f, "Railgun subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, rate={rate:?}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}, collected_this_tick={collected_this_tick}, collected_hue_this_tick={collected_hue_this_tick}", controllable.name())
            }
            FlattiverseEventKind::RailgunSubsystem { controllable, slot, status, direction, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Railgun subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, direction={direction:?}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}", controllable.name())
            }
            FlattiverseEventKind::RepairSubsystem { controllable, slot, status, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, repaired_hull_this_tick } => {
                write!(f, "Repair subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, rate={rate}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}, repaired_hull_this_tick={repaired_hull_this_tick}", controllable.name())
            }
            FlattiverseEventKind::ResourceMinerSubsystem { controllable, slot, status, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, mined_metal_this_tick, mined_carbon_this_tick, mined_hydrogen_this_tick, mined_silicon_this_tick } => {
                write!(f, "Resource miner subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, rate={rate}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}, mined_metal_this_tick={mined_metal_this_tick}, mined_carbon_this_tick={mined_carbon_this_tick}, mined_hydrogen_this_tick={mined_hydrogen_this_tick}, mined_silicon_this_tick={mined_silicon_this_tick}", controllable.name())
            }
            FlattiverseEventKind::ShieldSubsystem { controllable, slot, status, current, active, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Shield subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current={current}, active={active}, rate={rate}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}", controllable.name())
            }
            FlattiverseEventKind::DynamicShotFabricatorSubsystem { controllable, slot, status, active, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Dynamic shot fabricator subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, active={active}, rate={rate}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}", controllable.name())
            }
            FlattiverseEventKind::DynamicInterceptorFabricatorSubsystem { controllable, slot, status, active, rate, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick } => {
                write!(f, "Dynamic interceptor fabricator subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, active={active}, rate={rate}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}", controllable.name())
            }
            FlattiverseEventKind::DynamicShotLauncherSubsystem { controllable, slot, status, relative_movement, ticks, load, damage, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Dynamic shot launcher subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, relative_movement={relative_movement:?}, ticks={ticks:?}, load={load:?}, damage={damage:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::DynamicInterceptorLauncherSubsystem { controllable, slot, status, relative_movement, ticks, load, damage, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Dynamic interceptor launcher subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, relative_movement={relative_movement:?}, ticks={ticks:?}, load={load:?}, damage={damage:?}, consumed_energy_this_tick={consumed_energy_this_tick:?}, consumed_ions_this_tick={consumed_ions_this_tick:?}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick:?}", controllable.name())
            }
            FlattiverseEventKind::DynamicShotMagazineSubsystem { controllable, slot, status, current_shots } => {
                write!(f, "Dynamic shot magazine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current_shots={current_shots}", controllable.name())
            }
            FlattiverseEventKind::DynamicInterceptorMagazineSubsystem { controllable, slot, status, current_shots } => {
                write!(f, "Dynamic interceptor magazine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current_shots={current_shots}", controllable.name())
            }
            FlattiverseEventKind::ModernShipEngineSubsystem { controllable, slot, status, current_thrust, target_thrust, consumed_energy_this_tick, consumed_ions_this_tick, consumed_neutrinos_this_tick, } => {
                write!(f, "Dynamic interceptor magazine subsystem event: controllable={:?}, slot={slot:?}, status={status:?}, current_thrust={current_thrust}, target_thrust={target_thrust}, consumed_energy_this_tick={consumed_energy_this_tick}, consumed_ions_this_tick={consumed_ions_this_tick}, consumed_neutrinos_this_tick={consumed_neutrinos_this_tick}", controllable.name())
            }

            FlattiverseEventKind::PlayerScoreUpdated { player, before } => {
                write!(
                    f,
                    "Player score updated: {:?}, player_kills={}-{}, player_deaths={}-{}, friendly_kills={}-{}, friendly_deaths={}-{}, npc_kills={}-{}, npc_deaths={}-{}, neutral_deaths={}-{}, mission={}-{}.",
                    player.id(),
                    before.player_kills(), player.score().player_kills(),
                    before.player_deaths(), player.score().player_deaths(),
                    before.friendly_kills(), player.score().friendly_kills(),
                    before.friendly_deaths(), player.score().friendly_deaths(),
                    before.npc_kills(), player.score().npc_kills(),
                    before.npc_deaths(), player.score().npc_deaths(),
                    before.neutral_deaths(), player.score().neutral_deaths(),
                    before.mission(), player.score().mission(),
                )
            }

            FlattiverseEventKind::CompiledWithMessage { message, .. } => write!(f, "{message}"),
            FlattiverseEventKind::EnvironmentDamage { controllable, heat, heat_energy_cost, heat_energy_overflow, radiation, radiation_damage_before_armor, armor_blocked_damage, hull_damage } => {
                write!(f, "Environment damage event: controllable={:?}, heat={heat}, heat_energy_cost={heat_energy_cost}, heat_energy_overflow={heat_energy_overflow}, radiation={radiation}, radiation_damage_before_armor={radiation_damage_before_armor}, armor_blocked_damage={armor_blocked_damage}, hull_damage={hull_damage}", controllable.name())
            }
        }
    }
}

/// Connector-side classification of [`FlattiverseEvent`].
/// These values are meant for application-side dispatch and do not directly mirror wire-protocol
/// packet opcodes.
#[derive(Debug)]
pub enum FlattiverseEventKind {
    /// Raised when a player snapshot becomes known to the connector.
    PlayerJoined {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
    },
    /// Is raised when a player score has been updated.
    PlayerScoreUpdated {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The player score before the update.
        before: Score,
    },
    /// Raised when a player's connection has dropped but the player snapshot is still present for
    /// cleanup.
    PlayerDisconnected {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
    },
    /// Raised when a player snapshot is removed from the local galaxy mirror.
    PlayerParted {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
    },
    /// Raised when a player registers a new public controllable entry.
    ControllableInfoRegistered {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// Raised when a public controllable entry becomes alive in the world again.
    ControllableInfoContinued {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// Raised when a public controllable entry dies.
    ControllableInfoDestroyed {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        /// Reason the referenced controllable was destroyed.
        reason: PlayerUnitDestroyedReason,
    },
    /// Raised when one controllable runtime is destroyed by colliding with a neutral world unit.
    ControllableInfoDestroyedByNeutralCollision {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        /// Reason the referenced controllable was destroyed.
        reason: PlayerUnitDestroyedReason,
        /// Unit kind of the neutral collider.
        colliders_kind: UnitKind,
        /// Name of the neutral collider.
        colliders_name: String,
    },
    /// A PlayerUnit got destroyed by collision with a neutral unit.
    ControllableInfoDestroyedByPlayerUnit {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        /// Reason the referenced controllable was destroyed.
        reason: PlayerUnitDestroyedReason,
        /// Controllable entry of the destroyer.
        destroyed_unit: Arc<ControllableInfo>,
        /// Owner of the destroyer controllable.
        destroyer_player: Arc<Player>,
    },
    /// Is raised when a controllable-info score has been updated.
    ControllableInfoScoreUpdated {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        before: Score,
    },
    /// Raised when a public controllable entry is finally closed and removed.
    /// This is the final close, not the initial close request.
    ControllableInfoClosed {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The corresponding PlayerUnit the ControllableInfo informs about.
        controllable: Arc<ControllableInfo>,
    },
    /// Raised when a visible unit becomes newly known to the local visibility mirror.
    UnitAdded {
        /// Snapshot copy of the visible unit this event is about.
        unit: Arc<dyn Unit>,
    },
    /// Raised when the connector updates the snapshot of a currently visible unit.
    UnitUpdated {
        /// Snapshot copy of the visible unit this event is about.
        unit: Arc<dyn Unit>,
    },
    /// Raised when a previously known visible unit leaves the local visibility mirror.
    UnitRemoved {
        /// Snapshot copy of the visible unit this event is about.
        unit: Arc<dyn Unit>,
    },
    /// Raised when a previously known unit was changed by admin map editing.
    /// This event is a cache invalidation hint, not a full replacement unit snapshot.
    UnitAlteredByAdmin {
        /// The cluster id of the altered unit.
        cluster: ClusterId,
        /// The name of the altered unit.
        name: String,
    },
    /// Galaxy-wide system chat announcing that a flag has been scored.
    FlagScoredChat {
        /// Player who triggered the score.
        player: Arc<Player>,
        /// Controllable that triggered the score.
        controllable_info: Arc<ControllableInfo>,
        /// Team configured on the flag.
        flag_team: Arc<Team>,
        /// Name of the scored flag.
        flag_name: String,
    },
    /// Galaxy-wide system chat announcing that a domination point has scored.
    DominationPointScoredChat {
        /// Team that scored the domination point.
        team: Arc<Team>,
        /// Name of the domination point.
        domination_point_name: String,
    },
    /// Galaxy-wide system chat announcing that someone hit the own flag.
    OwnFlagHitChat {
        /// Player who triggered the own goal.
        player: Arc<Player>,
        /// Controllable that triggered the own goal.
        controllable_info: Arc<ControllableInfo>,
        /// Team configured on the flag.
        flag_team: Arc<Team>,
        /// Name of the flag.
        flag_name: String,
    },
    /// You received a galaxy chat message.
    GalaxyChat {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Galaxy>,
        /// The message of the chat.
        message: String,
    },
    /// You received a team chat message.
    TeamChat {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Player>,
        /// The message of the chat.
        message: String,
    },
    /// You received a private message of a team member.
    PlayerChat {
        /// Player snapshot this event refers to.
        player: Arc<Player>,
        /// The destination where this message was sent to.
        destination: Arc<Player>,
        /// The message of the chat.
        message: String,
    },
    /// Galaxy-wide system chat announcing that a ship hit the next mission target in sequence.
    MissionTargetHitChat {
        /// Player who hit the mission target.
        player: Arc<Player>,
        /// Controllable that hit the mission target.
        controllable_info: Arc<ControllableInfo>,
        /// Sequence number of the mission target that was hit.
        mission_target_sequence: u16,
    },
    /// Galaxy-wide system chat announcing that a flag became active again.
    FlagReactivatedChat {
        /// Team configured on the flag.
        flag_team: Arc<Team>,
        /// Name of the reactivated flag.
        flag_name: String,
    },
    /// Event emitted when a switch changes the state of one or more gates.
    GateSwitched {
        /// Cluster containing the switch and gates.
        cluster: Arc<Cluster>,
        /// Optional player who triggered the switch.
        invoker_player: Option<Arc<Player>>,
        /// Optional controllable that triggered the switch.
        invoker_controllable_info: Option<Arc<ControllableInfo>>,
        /// Name of the triggered switch.
        switch_name: String,
        /// Final states of the affected gates.
        gates: Vec<GateStateChange>,
    },
    /// Event emitted when a gate auto-restores to its configured default state.
    GateRestored {
        /// Cluster containing the gate.
        cluster: Arc<Cluster>,
        /// Name of the restored gate.
        gate_name: String,
        /// Final closed state after the restore.
        closed: bool,
    },
    /// Raised when the galaxy connection has terminated and no further protocol traffic will
    /// arrive.
    ConnectionTerminated {
        /// Optional close reason supplied by the local connector or the remote endpoint.
        message: Option<String>,
    },
    /// A tick happened.
    GalaxyTick {
        tick: u32,
    },

    /// Raised when the server initializes or updates the mirrored galaxy settings snapshot.
    GalaxySettingsUpdated {
        /// The updated [Galaxy].
        galaxy: Arc<Galaxy>,
        /// Previous settings snapshot.
        /// `None` when the connector receives the first settings snapshot after connect.
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

    /// Raised when the galaxy starts mirroring a newly configured tournament.
    TournamentCreated {
        /// Newly mirrored tournament snapshot.
        tournament: Arc<Tournament>,
    },
    /// Raised when the mirrored tournament snapshot changes.
    TournamentUpdated {
        /// Tournament snapshot before the update.
        old_tournament: Arc<Tournament>,
        /// Tournament snapshot after the update.
        new_tournament: Arc<Tournament>,
    },
    /// Raised when the currently mirrored tournament is removed from the galaxy.
    TournamentRemoved {
        /// Last tournament snapshot before removal.
        tournament: Arc<Tournament>,
    },
    /// Server-originated tournament system chat message.
    TournamentMessage {
        message: String,
    },
    /// Your controllable has collected a power-up.
    PowerUpCollected {
        /// The controllable that collected the power-up.
        controllable: Arc<Controllable>,
        /// The kind of the collected power-up.
        power_up_kind: UnitKind,
        /// The configured unit name of the collected power-up.
        power_up_name: String,
        /// The configured amount carried by the power-up.
        amount: f32,
        /// The amount that was actually applied to the controllable.
        applied_amount: f32,
    },

    // ------------------- ControllableSubsystemEvents -------------------
    /// Update of an armor subsystem on your own controllable.
    ArmorSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Flat damage reduction applied before the hull.
        reduction: f32,
        /// Direct damage blocked during the current tick.
        blocked_direct_damage_this_tick: f32,
        /// Radiation damage blocked during the current tick.
        blocked_radiation_damage_this_tick: f32,
    },
    /// Update of a battery subsystem on your own controllable.
    BatterySubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The current stored amount
        current: f32,
        /// The amount consumed during the current server tick.
        consumed_this_tick: f32,
    },
    /// Update of a cargo subsystem on your own controllable.
    CargoSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Metal currently stored in cargo.
        current_metal: f32,
        /// Carbon currently stored in cargo.
        current_carbon: f32,
        /// Hydrogen currently stored in cargo.
        current_hydrogen: f32,
        /// Silicon currently stored in cargo.
        current_silicon: f32,
        /// Nebula currently stored in cargo.
        current_nebula: f32,
        /// Average hue of the stored nebula.
        nebula_hue: f32,
    },
    /// Update of an energy-cell subsystem on your own controllable.
    EnergyCellSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The amount collected during the current server tick.
        collected_this_tick: f32,
    },
    /// Update of a scanner subsystem on your own controllable.
    DynamicScannerSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Whether the scanner was active during this server tick.
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
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
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
    /// Update of a hull subsystem on your own controllable
    HullSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The current hull integrity.
        current: f32,
    },
    /// Update of a nebula collector subsystem on your own controllable.
    NebulaCollectorSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Collector rate mirrored for the current server tick.
        rate: f32,
        /// Energy consumed by the collector during the current server tick.
        consumed_energy_this_tick: f32,
        /// Ions consumed by the collector during the current server tick.
        consumed_ions_this_tick: f32,
        /// Neutrinos consumed by the collector during the current server tick.
        consumed_neutrinos_this_tick: f32,
        /// Nebula amount collected during the current server tick.
        collected_this_tick: f32,
        /// Hue of the nebula material collected during the current server tick.
        collected_hue_this_tick: f32,
    },
    /// Update of a railgun subsystem on your own controllable.
    RailgunSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The direction processed in the current tick.
        direction: RailgunDirection,
        /// Energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// Ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// Neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Update of a repair subsystem on your own controllable.
    RepairSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Configured hull repair rate for the tick.
        rate: f32,
        /// Energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// Ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// Neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
        /// Hull repaired during the current tick.
        repaired_hull_this_tick: f32,
    },
    /// Update of a resource miner subsystem on your own controllable.
    ResourceMinerSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Configured mining rate for the tick.
        rate: f32,
        /// Energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// Ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// Neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
        /// Metal mined during the current tick.
        mined_metal_this_tick: f32,
        /// Carbon mined during the current tick.
        mined_carbon_this_tick: f32,
        /// Hydrogen mined during the current tick.
        mined_hydrogen_this_tick: f32,
        /// Silicon mined during the current tick.
        mined_silicon_this_tick: f32,
    },
    /// Update of a hull subsystem on your own controllable
    ShieldSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The current shield integrity.
        current: f32,
        /// Whether shield loading was active for the tick.
        active: bool,
        /// The configured shield load rate.
        rate: f32,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Update of a dynamic shot fabricator subsystem on your own controllable.
    DynamicShotFabricatorSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Whether the fabricator was active for the tick.
        active: bool,
        /// The configured fabrication rate.
        rate: f32,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Runtime update of a dynamic interceptor fabricator subsystem on your own controllable.
    DynamicInterceptorFabricatorSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// Whether the fabricator was active for the tick.
        active: bool,
        /// The configured fabrication rate.
        rate: f32,
        /// The energy consumed during the current server tick.
        consumed_energy_this_tick: f32,
        /// The ions consumed during the current server tick.
        consumed_ions_this_tick: f32,
        /// The neutrinos consumed during the current server tick.
        consumed_neutrinos_this_tick: f32,
    },
    /// Update of a shot launcher subsystem on your own controllable.
    DynamicShotLauncherSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
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
    /// Update of a dynamic interceptor launcher subsystem on your own controllable.
    DynamicInterceptorLauncherSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
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
    /// Update of a dynamic shot magazine subsystem on your own controllable.
    DynamicShotMagazineSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The currently stored shots.
        current_shots: f32,
    },
    /// Update of a dynamic interceptor magazine subsystem on your own controllable.
    DynamicInterceptorMagazineSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        /// The currently stored shots.
        current_shots: f32,
    },
    /// Update of a dynamic interceptor magazine subsystem on your own controllable.
    ModernShipEngineSubsystem {
        /// The controllable whose subsystem emitted this runtime event.
        controllable: Arc<Controllable>,
        /// The concrete subsystem slot on the controllable.
        slot: SubsystemSlot,
        /// Runtime status reported for the current server tick.
        /// This status is independent from configuration flags such as [`Controllable::active`] on
        /// specific subsystem types.
        status: SubsystemStatus,
        current_thrust: f32,
        target_thrust: f32,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
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
    /// Owner-only runtime update about passive heat and radiation in the current tick.
    EnvironmentDamage {
        /// The affected controllable.
        controllable: Arc<Controllable>,
        /// Aggregated incoming heat of the tick.
        heat: f32,
        /// Energy drained by heat in the tick.
        heat_energy_cost: f32,
        /// Heat that could not be paid and therefore overflowed into radiation.
        heat_energy_overflow: f32,
        /// Aggregated incoming radiation of the tick before heat overflow is added.
        radiation: f32,
        /// Radiation damage before armor reduction.
        radiation_damage_before_armor: f32,
        /// Radiation damage blocked by armor.
        armor_blocked_damage: f32,
        /// Hull damage caused by the passive environment in the tick.
        hull_damage: f32,
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
