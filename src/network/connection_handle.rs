use crate::galaxy_hierarchy::{
    ClusterId, ControllableId, PlayerId, Region, RegionTeam, Regions, TeamId,
};
use crate::network::{InvalidArgumentKind, Packet, Session, SessionHandler};
use crate::utils::check_name_or_err_32;
use crate::{GameError, GameErrorKind, Vector};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::error::RecvError;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) sender: Sender<SenderData>,
    pub(crate) sessions: Arc<SessionHandler>,
}

impl Debug for ConnectionHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionHandle").finish_non_exhaustive()
    }
}

impl From<Sender<SenderData>> for ConnectionHandle {
    fn from(sender: Sender<SenderData>) -> Self {
        Self {
            sender,
            sessions: Arc::new(SessionHandler::default()),
        }
    }
}

impl ConnectionHandle {
    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Player`].
    #[inline]
    pub async fn chat_player(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.chat_player_split(player, message).await?.await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Player`].
    pub async fn chat_player_split(
        &self,
        player: PlayerId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC6);
        packet.write(|writer| {
            writer.write_byte(player.0);
            writer.write_string_with_len_prefix(message.as_ref())
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    #[inline]
    pub async fn chat_team(&self, team: TeamId, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_team_split(team, message).await?.await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    pub async fn chat_team_split(
        &self,
        team: TeamId,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC5);
        packet.write(|writer| {
            writer.write_byte(team.0);
            writer.write_string_with_len_prefix(message.as_ref())
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Sends a chat message to all players in the connected [`crate::galaxy_hierarchy::Galaxy`].
    #[inline]
    pub async fn chat_galaxy(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_galaxy_split(message).await?.await
    }

    /// Sends a chat message with to all players in the connected [`crate::galaxy_hierarchy::Galaxy`].
    pub async fn chat_galaxy_split(
        &self,
        message: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xC4);
        packet.write(|writer| writer.write_string_with_len_prefix(message.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to close a [`crate::galaxy_hierarchy::Controllable`].
    #[inline]
    pub async fn dispose_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dispose_controllable_split(controllable).await?.await
    }

    /// Call this to close a [`crate::galaxy_hierarchy::Controllable`].
    pub async fn dispose_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x8F);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to continue the game with the unit after you are dead or when you hve created the
    /// unit.
    #[inline]
    pub async fn continue_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.continue_controllable_split(controllable).await?.await
    }

    /// Call this to continue the game with the unit after you are dead or when you hve created the
    /// unit.
    pub async fn continue_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x84);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to suicide (=self destroy).
    #[inline]
    pub async fn suicide_controllable(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.suicide_controllable_split(controllable).await?.await
    }

    /// Call this to suicide (=self destroy).
    pub async fn suicide_controllable_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x85);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Call this to move your ship. This vector will be the impulse your ship gets every tick until
    /// you specify a new vector. Length of 0 will turn off your engines.
    #[inline]
    pub async fn classic_controllable_move(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<(), GameError> {
        self.classic_controllable_move_split(controllable, movement)
            .await?
            .await
    }

    /// Call this to move your ship. This vector will be the impulse your ship gets every tick until
    /// you specify a new vector. Length of 0 will turn off your engines.
    pub async fn classic_controllable_move_split(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if movement.is_damaged() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::ConstrainedInfinity,
                parameter: "movement".to_string(),
            }
            .into())
        } else if movement.length() > 0.101f32 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "movement".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x87);
            packet.write(|writer| {
                writer.write_byte(controllable.0);
                movement.write(writer);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Shoots a shot into the specified direction and with the specified parameters. Please note
    /// that you can only shoot one shot per tick.
    ///
    /// * `relative_movement` - The direction in which the shot will fly (value range `[0.1f; 3f]`).
    /// * `ticks` - The ticks how long the shot will fly (value range `[3; 140]`).
    /// * `load` - The explosion size when the ticks reach 0 (value range `[3; 25]`).
    /// * `damage` - The damage the shot should inflict (value range `[0.1f; 3f]`).
    #[inline]
    pub async fn classic_controllable_shoot(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        self.classic_controllable_shoot_split(controllable, relative_movement, ticks, load, damage)
            .await?
            .await
    }

    /// Shoots a shot into the specified direction and with the specified parameters. Please note
    /// that you can only shoot one shot per tick.
    ///
    /// * `relative_movement` - The direction in which the shot will fly (value range `[0.1f; 3f]`).
    /// * `ticks` - The ticks how long the shot will fly (value range `[3; 140]`).
    /// * `load` - The explosion size when the ticks reach 0 (value range `[3; 25]`).
    /// * `damage` - The damage the shot should inflict (value range `[0.1f; 3f]`).
    pub async fn classic_controllable_shoot_split(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if relative_movement.is_damaged() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::ConstrainedInfinity,
                parameter: "relativeMovement".to_string(),
            }
            .into())
        } else if relative_movement.length() > 3.001 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "relativeMovement".to_string(),
            }
            .into())
        } else if relative_movement.length() < 0.099 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooSmall,
                parameter: "relativeMovement".to_string(),
            }
            .into())
        } else if ticks < 3 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooSmall,
                parameter: "ticks".to_string(),
            }
            .into())
        } else if ticks > 140 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "ticks".to_string(),
            }
            .into())
        } else if load < 3.0 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooSmall,
                parameter: "load".to_string(),
            }
            .into())
        } else if load > 25.0 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "load".to_string(),
            }
            .into())
        } else if damage < 0.099 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooSmall,
                parameter: "damage".to_string(),
            }
            .into())
        } else if damage > 3.001 {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::TooLarge,
                parameter: "damage".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x88);
            packet.write(|writer| {
                writer.write_byte(controllable.0);
                relative_movement.write(writer);
                writer.write_uint16(ticks);
                writer.write_f32(load);
                writer.write_f32(damage);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Create a classic style ship.
    #[inline]
    pub async fn create_classic_style_ship(
        &self,
        name: impl AsRef<str>,
    ) -> Result<ControllableId, GameError> {
        self.create_classic_style_ship_split(name).await?.await
    }

    /// Create a classic style ship.
    pub async fn create_classic_style_ship_split(
        &self,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<ControllableId, GameError>>, GameError> {
        check_name_or_err_32(name.as_ref())?;

        let mut packet = Packet::default();
        packet.header_mut().set_command(0x80);
        packet.write(|writer| writer.write_string_with_len_prefix(name.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                Ok(packet.read(|reader| ControllableId(reader.read_byte())))
            })
        })
    }

    /// Creates or updates a region within the cluster:
    ///
    /// ```xml
    /// <Region Id="66" Name="Spawn A" Left="-150" Top="-300" Right="150" Bottom="0">
    ///   <Team Id="0" />
    /// </Region>
    /// ```
    #[inline]
    pub async fn set_cluster_region(
        &self,
        cluster: ClusterId,
        xml: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.set_cluster_region_split(cluster, xml).await?.await
    }

    /// Creates or updates a region within the cluster:
    ///
    /// ```xml
    /// <Region Id="66" Name="Spawn A" Left="-150" Top="-300" Right="150" Bottom="0">
    ///   <Team Id="0" />
    /// </Region>
    /// ```
    pub async fn set_cluster_region_split(
        &self,
        cluster: ClusterId,
        xml: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let xml = xml.as_ref();
        if xml.is_empty() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::AmbiguousXmlData,
                parameter: "xml".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x24);
            packet.write(|writer| {
                writer.write_byte(cluster.0);
                writer.write_string_with_len_prefix(&xml);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Removes a region by id from the cluster.
    #[inline]
    pub async fn remove_cluster_region(
        &self,
        cluster: ClusterId,
        region: u8,
    ) -> Result<(), GameError> {
        self.remove_cluster_region_split(cluster, region)
            .await?
            .await
    }

    /// Removes a region by id from the cluster.
    pub async fn remove_cluster_region_split(
        &self,
        cluster: ClusterId,
        region: u8,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x25);
        packet.write(|writer| {
            writer.write_byte(cluster.0);
            writer.write_byte(region);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |_| Ok(()))
        })
    }

    /// Queries all regions of the cluster as XML.
    #[inline]
    pub async fn query_cluster_regions(&self, cluster: ClusterId) -> Result<String, GameError> {
        self.query_cluster_regions_split(cluster).await?.await
    }

    /// Queries all regions of the cluster as XML.
    pub async fn query_cluster_regions_split(
        &self,
        cluster: ClusterId,
    ) -> Result<impl Future<Output = Result<String, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x26);
        packet.write(|writer| writer.write_byte(cluster.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut response| {
                response.read(|reader| {
                    let region_count = reader.read_uint16();
                    let mut regions = Regions(Vec::with_capacity(region_count as usize));

                    for _ in 0..region_count {
                        regions.0.push(Region {
                            id: reader.read_byte(),
                            name: reader.opt_read_string(),
                            left: reader.read_f32(),
                            top: reader.read_f32(),
                            right: reader.read_f32(),
                            bottom: reader.read_f32(),
                            teams: {
                                let start_location_teams = reader.read_uint32();
                                let mut teams =
                                    Vec::with_capacity(start_location_teams.count_ones() as usize);

                                for team_id in 0..32u8 {
                                    let team_mask = 1u32 << team_id;
                                    if (start_location_teams & team_mask) != 0 && team_id != 12 {
                                        teams.push(RegionTeam { id: team_id })
                                    }
                                }

                                teams
                            },
                        })
                    }

                    Ok(serde_xml_rs::to_string(&regions).unwrap())
                })
            })
        })
    }

    /// Creates or updates a single editable map unit in the cluster.
    /// Root node must be the unit type itself, for example `<Sun />`.
    /// For `<Buoy />` an optional message attribute is supported (max 384 characters).
    /// For `<MissionTarget />` the team is required and child nodes `<Vector X="..." Y="..." />`
    /// are supported.
    #[inline]
    pub async fn set_cluster_unit(
        &self,
        cluster: ClusterId,
        xml: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.set_cluster_unit_split(cluster, xml).await?.await
    }

    /// Creates or updates a single editable map unit in the cluster.
    /// Root node must be the unit type itself, for example `<Sun />`.
    /// For `<Buoy />` an optional message attribute is supported (max 384 characters).
    /// For `<MissionTarget />` the team is required and child nodes `<Vector X="..." Y="..." />`
    /// are supported.
    #[inline]
    pub async fn set_cluster_unit_split(
        &self,
        cluster: ClusterId,
        xml: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let xml = xml.as_ref();
        if xml.is_empty() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::AmbiguousXmlData,
                parameter: "xml".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x28);
            packet.write(|writer| {
                writer.write_byte(cluster.0);
                writer.write_string_with_len_prefix(&xml);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Removes a single editable map unit by name.
    #[inline]
    pub async fn remove_cluster_unit(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
    ) -> Result<(), GameError> {
        self.remove_cluster_unit_split(cluster, name).await?.await
    }

    /// Removes a single editable map unit by name.
    pub async fn remove_cluster_unit_split(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let name = name.as_ref();
        if name.is_empty() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::AmbiguousXmlData,
                parameter: "xml".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x29);
            packet.write(|writer| {
                writer.write_byte(cluster.0);
                writer.write_string_with_len_prefix(name);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |_| Ok(()))
            })
        }
    }

    /// Queries the XML of one specific editable map unit by name.
    #[inline]
    pub async fn query_cluster_unit_xml(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
    ) -> Result<String, GameError> {
        self.query_cluster_unit_xml_split(cluster, name)
            .await?
            .await
    }

    /// Queries the XML of one specific editable map unit by name.
    pub async fn query_cluster_unit_xml_split(
        &self,
        cluster: ClusterId,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<String, GameError>>, GameError> {
        let name = name.as_ref();
        if name.is_empty() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::AmbiguousXmlData,
                parameter: "name".to_string(),
            }
            .into())
        } else {
            let mut packet = Packet::default();
            packet.header_mut().set_command(0x2A);
            packet.write(|writer| {
                writer.write_byte(cluster.0);
                writer.write_string_with_len_prefix(name);
            });

            let session = self.send_packet_on_new_session(packet).await?;

            Ok(async move {
                let response = session.response().await?;
                GameError::check(response, |mut response| {
                    Ok(response.read(|reader| reader.read_string()))
                })
            })
        }
    }

    #[inline]
    pub(crate) fn respond_to_ping(&self, challenge: u16) -> Result<(), GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x00);
        packet.write(|writer| writer.write_uint16(challenge));
        self.send_packet_directly(packet)
    }

    #[inline]
    fn send_packet_directly(&self, packet: Packet) -> Result<(), GameError> {
        self.sender
            .try_send(SenderData::Packet(packet))
            .map_err(|_| {
                GameErrorKind::ConnectionTerminated {
                    reason: Some(Arc::from("Packet-Sender gone")),
                }
                .into()
            })
    }

    #[inline]
    async fn send_packet_on_new_session(&self, mut packet: Packet) -> Result<Session, GameError> {
        let session = self
            .sessions
            .get()
            .ok_or(GameErrorKind::SessionsExhausted)?;

        packet.header_mut().set_session(session.id().0);

        self.sender.send(SenderData::Packet(packet)).await?;

        Ok(session)
    }
}

pub enum SenderData {
    #[cfg(not(feature = "wasm"))]
    Raw(tokio_tungstenite::tungstenite::Message),
    Packet(Packet),
    Close,
}

impl From<RecvError> for GameError {
    #[inline]
    fn from(e: RecvError) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated {
            reason: Some(Arc::from("Receiver gone")),
        }
        .into()
    }
}

impl<T> From<SendError<T>> for GameError {
    #[inline]
    fn from(e: SendError<T>) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated {
            reason: Some(Arc::from("Sender gone")),
        }
        .into()
    }
}

impl From<async_channel::RecvError> for GameError {
    #[inline]
    fn from(e: async_channel::RecvError) -> Self {
        debug!("Connection Terminated: {e:?}");
        GameErrorKind::ConnectionTerminated {
            reason: Some(Arc::from("Receiver gone")),
        }
        .into()
    }
}
