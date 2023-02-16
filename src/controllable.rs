use crate::error::GameError;
use crate::network::connection_handle::ConnectionHandle;
use crate::network::query::{QueryCommand, QueryResponse};
use crate::team::TeamId;
use crate::units::player_unit::PlayerUnitSystems;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ControllableId(pub(crate) usize);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControllableState {
    /// The movement of your controllable.
    pub movement: Vector,
    /// The position of your controllable.
    pub position: Vector,
    /// The direction of your controllable.
    pub direction: f64,
    /// THe radius of your controllable.
    pub radius: f64,
    /// The gravity that your controllable is exercising on the other units.
    pub gravity: f64,
    /// The current energy output of your controllable.
    #[serde(rename = "energyOutput")]
    pub energy_output: f64,
    /// The rate at which your controllable is turning.
    #[serde(rename = "turnRate")]
    pub turn_rate: f64,
    #[serde(rename = "requestedScanDirection")]
    pub requested_scan_direction: f64,
    #[serde(rename = "requestedScanWidth")]
    pub requested_scan_width: f64,
    #[serde(rename = "requestedScanRange")]
    pub requested_scan_range: f64,
    #[serde(rename = "scanDirection")]
    pub scan_direction: f64,
    #[serde(rename = "scanWidth")]
    pub scan_width: f64,
    #[serde(rename = "scanRange")]
    pub scan_range: f64,
    #[serde(rename = "scanActivated")]
    pub scan_activated: bool,
    pub systems: PlayerUnitSystems,
}

pub struct Controllable {
    pub(crate) connection: Arc<ConnectionHandle>,
    /// The name of your controllable.
    pub name: String,
    /// The id of your controllable.
    pub id: ControllableId,
    /// If you have joined a team, the team of your controllable.
    pub team: Option<TeamId>,
    pub active: AtomicBool,
    pub state: Mutex<ControllableState>,
}

impl Controllable {
    pub async fn is_alive(&self) -> bool {
        self.state.lock().await.systems.hull.value > 0.0
    }

    pub fn blocking_is_alive(&self) -> bool {
        self.state.blocking_lock().systems.hull.value > 0.0
    }

    pub(crate) async fn die(&self) {
        self.active.store(false, Ordering::Relaxed);
        self.state.lock().await.systems.hull.value = 0.0;
    }

    pub(crate) async fn update_state(&self, state: ControllableState) {
        *self.state.lock().await = state;
    }

    pub async fn r#continue(&self) -> Result<QueryResponse, GameError> {
        if self.is_alive().await {
            Err(GameError::ControllableMustBeDeadToContinue)
        } else {
            Ok({
                let result = self
                    .connection
                    .send_query(QueryCommand::ContinueControllable {
                        controllable: self.id,
                    })
                    .await?
                    .await?;
                self.active.store(true, Ordering::Relaxed);
                result
            })
        }
    }

    pub async fn kill(&self) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::KillControllable {
                    controllable: self.id,
                })
                .await?
                .await?)
        }
    }

    pub async fn set_nozzle(&self, value: f64) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let max_nozzle = self
                .state
                .lock()
                .await
                .systems
                .nozzle
                .specialization
                .max_value;

            if value.abs() > max_nozzle * 1.05 {
                Err(GameError::FloatingPointNumberOutOfRange)
            } else {
                Ok(self
                    .connection
                    .send_query(QueryCommand::SetControllableNozzle {
                        controllable: self.id,
                        nozzle: {
                            let max_value = max_nozzle;
                            value.clamp(-max_value, max_value)
                        },
                    })
                    .await?
                    .await?)
            }
        }
    }

    pub async fn set_thruster(&self, value: f64) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else if value < 0.0
            || value > {
                self.state
                    .lock()
                    .await
                    .systems
                    .thruster
                    .specialization
                    .max_value
            } * 1.05
        {
            Err(GameError::FloatingPointNumberOutOfRange)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableThruster {
                    controllable: self.id,
                    thrust: value,
                })
                .await?
                .await?)
        }
    }

    pub async fn set_scanner(
        &self,
        direction: f64,
        length: f64,
        width: f64,
        enabled: bool,
    ) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !direction.is_finite() || !length.is_finite() || !width.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let direction = (direction + 3600.0) % 360.0;
            let (max_length, max_angle) = {
                let lock = self.state.lock().await;
                (
                    lock.systems.scanner.specialization.max_range,
                    lock.systems.scanner.specialization.max_angle,
                )
            };

            if direction < 0.0
                || !(59.9..360.1).contains(&length)
                || length > max_length * 1.05
                || width < 19.9
                || width > max_angle * 1.05
            {
                return Err(GameError::FloatingPointNumberOutOfRange);
            }

            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableScanner {
                    controllable: self.id,
                    direction,
                    length: length.clamp(60.0, max_length),
                    width: width.clamp(20.0, max_angle),
                    enabled,
                })
                .await?
                .await?)
        }
    }

    /// Shoots a shot. It can only handle one shot per tick per ship and has a buffer of one
    /// additional shot. Units generally can shoot only one shot per tick, so specifying 3 shots in
    /// one tick will result in an error at the 3rd shot requested. The shot will be generated with
    /// the next tick. The server tries to anticipate, whether you are able to shoot. However, this
    /// may not be possible. So the call to this method ma be successful, but the shot may not be
    /// created, if you are out of energy or other required resources.
    /// Please observe events like [`crate::events::depleted_resource_event::DepletedResourceEvent`]
    /// to determine such situations.
    ///
    /// # The process
    ///
    /// The process is as described in the following steps.
    ///  * You call `.shoot()` with time `1`.
    ///  * The shot will be placed when the next tick is processed with time `1`.
    ///  * The next tick will change time to `0`.
    ///  * The next tick will delete the shot and create the explosion.
    ///  * In the next tick the explosion is removed and deals the damage.
    ///
    /// # Arguments
    ///
    ///  * `direction` - The direction in which you want to shoot. Calculated energy costs due to
    ///                  what the corresponding systems say are `true` for a exact forward shot.
    ///                  Shooting backwards the shot will cost **`7` times the energy**. Shooting
    ///                  `90` degrees sideways will cost **`4` times the energy** and so on. The
    ///                  length of this vector should be longer than `0.1`.
    ///  * `load` - The radius of the resulting explosion. The minimum value is `2.5`.
    ///  * `damage` - The damage dealt by the explosion. The minimum value is `0.001`.
    ///  * `time` - The amount of ticks the shot will live, before exploding.
    ///
    /// # Remarks
    ///
    /// Please query the status of your weapon systems for the corresponding maximums (`max_value`)
    /// and energy costs:
    ///   - [`Vector::length`] is `systems.weapon_launcher...specialization.max_value`
    ///   - `load` is `systems.weapon_payload_radius...specialization.max_value`
    ///   - `damage` is `systems.weapon_payload_damage...specialization.max_value`
    ///   - `time` is `systems.weapon_ammunition...specialization.max_value`
    pub async fn shoot(
        &self,
        direction: Vector,
        load: f64,
        damage: f64,
        time: u16,
    ) -> Result<QueryResponse, GameError> {
        if self.state.lock().await.systems.hull.value <= 0.0 {
            Err(GameError::ControllableMustBeAlive)
        } else if !load.is_finite() || !damage.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let state = self.state.lock().await;
            let s = &state.systems;

            if let (
                Some(weapon_ammunition),
                Some(_weapon_factory),
                Some(weapon_launcher),
                Some(weapon_payload_damage),
                Some(weapon_payload_radius),
            ) = (
                s.weapon_ammunition.as_ref(),
                s.weapon_factory.as_ref(),
                s.weapon_launcher.as_ref(),
                s.weapon_payload_damage.as_ref(),
                s.weapon_payload_radius.as_ref(),
            ) {
                let direction_length = direction.length();
                if direction_length < 0.075
                    || direction_length > weapon_launcher.specialization.max_value * 1.05
                    || load < 2.25
                    || load > weapon_payload_radius.specialization.max_value * 1.05
                    || damage < 0.00075
                    || damage > weapon_payload_damage.specialization.max_value
                    || time > weapon_ammunition.specialization.max_value.ceil() as u16
                {
                    Err(GameError::FloatingPointNumberInvalid)
                } else {
                    let load = load.clamp(2.5, weapon_launcher.specialization.max_value);
                    let damage =
                        damage.clamp(0.001, weapon_payload_damage.specialization.max_value);

                    Ok(self
                        .connection
                        .send_query(QueryCommand::ControllableShoot {
                            direction,
                            load,
                            damage,
                            time,
                        })
                        .await?
                        .await?)
                }
            } else {
                Err(GameError::MissingSystems)
            }
        }
    }

    /// Helper, executes the given [`FnOnce`] with a reference to the [`ControllableState`]
    pub async fn with_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Self, &ControllableState) -> T,
    {
        let lock = self.state.lock().await;
        f(self, &lock)
    }

    /// Helper, executes the given [`FnOnce`] with a reference to the [`ControllableState`]
    pub fn with_blocking_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Self, &ControllableState) -> T,
    {
        let lock = self.state.blocking_lock();
        f(self, &lock)
    }

    /// Helper, executes the given [`FnOnce`] with a reference to the [`ControllableState`].
    /// Awaits the returned [`Future`].
    pub async fn with_state_future<F, FF, T>(&self, f: F) -> T
    where
        FF: Future<Output = T>,
        F: FnOnce(&Self, &ControllableState) -> FF,
    {
        let lock = self.state.lock().await;
        f(self, &lock).await
    }
}
