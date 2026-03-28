use crate::galaxy_hierarchy::{
    ClusterId, ControllableId, PlayerId, Region, RegionTeam, Regions, ScannerSubsystemId, TeamId,
};
use crate::network::{InvalidArgumentKind, Packet, PacketWriter, Session, SessionHandler};
use crate::utils::check_name_or_err_32;
use crate::{FlattiverseEvent, GameError, GameErrorKind, Vector};
use async_channel::WeakSender;
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
    pub(crate) event_sender: WeakSender<FlattiverseEvent>,
}

impl Debug for ConnectionHandle {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionHandle").finish_non_exhaustive()
    }
}

impl ConnectionHandle {
    pub(crate) fn new(
        sender: Sender<SenderData>,
        event_sender: WeakSender<FlattiverseEvent>,
    ) -> Self {
        Self {
            sender,
            sessions: Arc::default(),
            event_sender,
        }
    }

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
            GameError::check_ok(response)
        })
    }

    /// Downloads the player's cached small avatar image bytes.
    #[inline]
    pub async fn download_player_small_avatar(
        &self,
        player: PlayerId,
    ) -> Result<Vec<u8>, GameError> {
        self.download_player_small_avatar_split(player).await?.await
    }

    /// Downloads the player's cached small avatar image bytes.
    pub async fn download_player_small_avatar_split(
        &self,
        player: PlayerId,
    ) -> Result<impl Future<Output = Result<Vec<u8>, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xF1);
        packet.write(|writer| writer.write_byte(player.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut response| {
                Ok(response.read(|reader| reader.read_remaining_as_bytes()))
            })
        })
    }

    /// Downloads the player's cached big avatar image bytes.
    #[inline]
    pub async fn download_player_big_avatar(&self, player: PlayerId) -> Result<Vec<u8>, GameError> {
        self.download_player_big_avatar_split(player).await?.await
    }

    /// Downloads the player's cached big avatar image bytes.
    pub async fn download_player_big_avatar_split(
        &self,
        player: PlayerId,
    ) -> Result<impl Future<Output = Result<Vec<u8>, GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0xF2);
        packet.write(|writer| writer.write_byte(player.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut response| {
                Ok(response.read(|reader| reader.read_remaining_as_bytes()))
            })
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
            GameError::check_ok(response)
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
            GameError::check_ok(response)
        })
    }

    /// Call this to request closing a [`crate::galaxy_hierarchy::Controllable`]. The server may
    /// keep it alive for a grace period before it is finally removed.
    #[inline]
    pub async fn request_controllable_close(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.request_controllable_close_split(controllable)
            .await?
            .await
    }

    /// Call this to request closing a [`crate::galaxy_hierarchy::Controllable`]. The server may
    /// keep it alive for a grace period before it is finally removed.
    pub async fn request_controllable_close_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x8F);
        packet.write(|writer| writer.write_byte(controllable.0));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
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
            GameError::check_ok(response)
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
            GameError::check_ok(response)
        })
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
                GameError::check_ok(response)
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
            GameError::check_ok(response)
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
                GameError::check_ok(response)
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
                GameError::check_ok(response)
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

    /// Configures galaxy metadata, teams and clusters from an XML document.
    /// Missing attributes keep old values for the referenced element.
    /// Team/Cluster elements define the final set: missing ids are removed.
    /// Unknown attributes and unknown child nodes are rejected by the server.
    ///
    /// ```xml
    /// <Galaxy Name="New Name">
    ///   <Team Id="0" />
    ///   <Team Id="1" Name="Green" ColorR="64" ColorG="255" ColorB="64" />
    ///   <Cluster Id="0" Name="Playground" Start="true" Respawn="false" />
    /// </Galaxy>
    /// ```
    ///
    /// Team id 12 (Spectators) must not be included.
    /// Team names must be unique.
    /// Removing a team fails if any remaining cluster still has regions referencing that team.
    /// Galaxy/Team/Cluster names must be non-empty and at most 32 characters.
    /// Description must be at most 4096 characters.
    /// At least one cluster must end up with `Start="true"`.
    #[inline]
    pub async fn configure_galaxy(&self, xml: impl AsRef<str>) -> Result<(), GameError> {
        self.configure_galaxy_split(xml).await?.await
    }

    /// Configures galaxy metadata, teams and clusters from an XML document.
    /// Missing attributes keep old values for the referenced element.
    /// Team/Cluster elements define the final set: missing ids are removed.
    /// Unknown attributes and unknown child nodes are rejected by the server.
    ///
    /// ```xml
    /// <Galaxy Name="New Name">
    ///   <Team Id="0" />
    ///   <Team Id="1" Name="Green" ColorR="64" ColorG="255" ColorB="64" />
    ///   <Cluster Id="0" Name="Playground" Start="true" Respawn="false" />
    /// </Galaxy>
    /// ```
    ///
    /// Team id 12 (Spectators) must not be included.
    /// Team names must be unique.
    /// Removing a team fails if any remaining cluster still has regions referencing that team.
    /// Galaxy/Team/Cluster names must be non-empty and at most 32 characters.
    /// Description must be at most 4096 characters.
    /// At least one cluster must end up with `Start="true"`.
    pub async fn configure_galaxy_split(
        &self,
        xml: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x04);
        packet.write(|writer| writer.write_string_with_len_prefix(xml.as_ref()));

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Sets the target movement impulse on the server.
    /// Values just above the maximum are clipped to the maximum before tey are sent.
    #[inline]
    pub async fn classic_ship_engine_subsystem_set(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<(), GameError> {
        self.classic_ship_engine_subsystem_set_split(controllable, movement)
            .await?
            .await
    }

    /// Sets the target movement impulse on the server.
    /// Values just above the maximum are clipped to the maximum before tey are sent.
    pub async fn classic_ship_engine_subsystem_set_split(
        &self,
        controllable: ControllableId,
        movement: Vector,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x87);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
            movement.write(writer);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Requests one shot for the next server tick.
    #[inline]
    pub async fn dynamic_shot_launcher_subsystem_shoot(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        self.dynamic_shot_launcher_subsystem_shoot_split(
            controllable,
            relative_movement,
            ticks,
            load,
            damage,
        )
        .await?
        .await
    }

    /// Requests one shot for the next server tick.
    pub async fn dynamic_shot_launcher_subsystem_shoot_split(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
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
            GameError::check_ok(response)
        })
    }

    /// Set the target scanner configuration on the server.
    #[inline]
    pub async fn dynamic_scanner_subsystem_set(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
        width: f32,
        length: f32,
        angle: f32,
    ) -> Result<(), GameError> {
        self.dynamic_scanner_subsystem_set_split(controllable, scanner, width, length, angle)
            .await?
            .await
    }

    /// Set the target scanner configuration on the server.
    pub async fn dynamic_scanner_subsystem_set_split(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
        width: f32,
        length: f32,
        angle: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.send_command_with_payload(0x89, |writer| {
            writer.write_byte(controllable.0);
            writer.write_byte(scanner.0);
            writer.write_f32(width);
            writer.write_f32(length);
            writer.write_f32(angle);
        })
        .await
        .map(GameError::check_response_ok)
    }

    /// Turns the scanner on.
    #[inline]
    pub async fn dynamic_scanner_subsystem_on(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
    ) -> Result<(), GameError> {
        self.dynamic_scanner_subsystem_on_split(controllable, scanner)
            .await?
            .await
    }

    /// Turns the scanner on.
    pub async fn dynamic_scanner_subsystem_on_split(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.send_command_with_payload(0x8A, |writer| {
            writer.write_byte(controllable.0);
            writer.write_byte(scanner.0);
        })
        .await
        .map(GameError::check_response_ok)
    }

    /// Turns the scanner off.
    #[inline]
    pub async fn dynamic_scanner_subsystem_off(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
    ) -> Result<(), GameError> {
        self.dynamic_scanner_subsystem_off_split(controllable, scanner)
            .await?
            .await
    }

    /// Turns the scanner off.
    pub async fn dynamic_scanner_subsystem_off_split(
        &self,
        controllable: ControllableId,
        scanner: ScannerSubsystemId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.send_command_with_payload(0x8B, |writer| {
            writer.write_byte(controllable.0);
            writer.write_byte(scanner.0);
        })
        .await
        .map(GameError::check_response_ok)
    }

    /// Sets the shield load rate on the server.
    #[inline]
    pub async fn shield_subsystem_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.shield_subsystem_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the shield load rate on the server.
    pub async fn shield_subsystem_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x90);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
            writer.write_f32(rate);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns shield loading on.
    #[inline]
    pub async fn shield_subsystem_on(&self, controllable: ControllableId) -> Result<(), GameError> {
        self.shield_subsystem_on_split(controllable).await?.await
    }

    /// Turns shield loading on.
    pub async fn shield_subsystem_on_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x91);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns shield loading off.
    #[inline]
    pub async fn shield_subsystem_off(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.shield_subsystem_off_split(controllable).await?.await
    }

    /// Turns shield loading off.
    pub async fn shield_subsystem_off_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x92);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Sets the shot fabrication rate on the server.
    #[inline]
    pub async fn dynamic_shot_fabricator_subsystem_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.dynamic_shot_fabricator_subsystem_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the shot fabrication rate on the server.
    pub async fn dynamic_shot_fabricator_subsystem_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x8C);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
            writer.write_f32(rate);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the shot fabricator on.
    #[inline]
    pub async fn dynamic_shot_fabricator_subsystem_on(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dynamic_shot_fabricator_subsystem_on_split(controllable)
            .await?
            .await
    }

    /// Turns the shot fabricator on.
    pub async fn dynamic_shot_fabricator_subsystem_on_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x8D, |writer| {
                writer.write_byte(controllable.0);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the shot fabricator off.
    #[inline]
    pub async fn dynamic_shot_fabricator_subsystem_off(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dynamic_shot_fabricator_subsystem_off_split(controllable)
            .await?
            .await
    }

    /// Turns the shot fabricator off.
    pub async fn dynamic_shot_fabricator_subsystem_off_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x8E, |writer| {
                writer.write_byte(controllable.0);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
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
    async fn send_command_with_payload(
        &self,
        command: u8,
        f: impl FnOnce(&mut dyn PacketWriter),
    ) -> Result<Session, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(command);
        packet.write(f);

        self.send_packet_on_new_session(packet).await
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
