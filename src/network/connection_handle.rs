use crate::account::{Account, AccountId};
use crate::galaxy_hierarchy::{
    ClusterId, ControllableId, Crystal, CrystalGrade, EditableUnitSummary, Galaxy, PlayerId,
    Region, RegionTeam, Regions, ScannerSubsystemId, TeamId, TournamentConfiguration,
};
use crate::network::{
    ChunkedTransfer, InvalidArgumentKind, Packet, PacketReader, PacketWriter, Session,
    SessionHandler,
};
use crate::unit::UnitKind;
use crate::utils::{check_name_or_err, Readable};
use crate::{FlattiverseEvent, GameError, GameErrorKind, ProgressState, SubsystemSlot, Vector};
use async_channel::WeakSender;
use serde::Serialize;
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
    #[instrument(level = "debug", skip(self, message), fields(message = message.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn download_player_small_avatar(
        &self,
        player: PlayerId,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<u8>, GameError> {
        ChunkedTransfer::download_bytes(
            |offset, maximum_length| {
                self.send_command_with_payload(0xF1, move |writer| {
                    writer.write_byte(player.0);
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_length);
                })
            },
            progress_state,
            "small avatar".to_string(),
        )
        .await
    }

    /// Downloads the player's cached big avatar image bytes.
    #[inline]
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn download_player_big_avatar(
        &self,
        player: PlayerId,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<u8>, GameError> {
        ChunkedTransfer::download_bytes(
            |offset, maximum_length| {
                self.send_command_with_payload(0xF2, move |writer| {
                    writer.write_byte(player.0);
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_length);
                })
            },
            progress_state,
            "big avatar".to_string(),
        )
        .await
    }

    /// Downloads the small persisted avatar image of this account.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn download_account_small_avatar(
        &self,
        account: AccountId,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<u8>, GameError> {
        ChunkedTransfer::download_bytes(
            |offset, maximum_length| {
                self.send_command_with_payload(0xF3, move |writer| {
                    writer.write_int32(account.0);
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_length);
                })
            },
            progress_state,
            "small account avatar".to_string(),
        )
        .await
    }

    /// Downloads the large persisted avatar image of this account.
    #[inline]
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn download_account_big_avatar(
        &self,
        account: AccountId,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<u8>, GameError> {
        ChunkedTransfer::download_bytes(
            |offset, maximum_length| {
                self.send_command_with_payload(0xF4, move |writer| {
                    writer.write_int32(account.0);
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_length);
                })
            },
            progress_state,
            "big account avatar".to_string(),
        )
        .await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    #[inline]
    pub async fn chat_team(&self, team: TeamId, message: impl AsRef<str>) -> Result<(), GameError> {
        self.chat_team_split(team, message).await?.await
    }

    /// Sends a chat message to the connected [`crate::galaxy_hierarchy::Team`].
    #[instrument(level = "debug", skip(self, message), fields(message = message.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self, message), fields(message = message.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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

    /// Creates a classic style ship with up to three equipped crystals.
    #[inline]
    pub async fn create_classic_style_ship(
        &self,
        name: impl AsRef<str>,
        crystal_0_name: impl AsRef<str>,
        crystal_1_name: impl AsRef<str>,
        crystal_2_name: impl AsRef<str>,
    ) -> Result<ControllableId, GameError> {
        self.create_classic_style_ship_split(name, crystal_0_name, crystal_1_name, crystal_2_name)
            .await?
            .await
    }

    /// Creates a classic style ship with up to three equipped crystals.
    #[instrument(
        level = "debug",
        skip(
            self,
            name,
            crystal_0_name,
            crystal_1_name,
            crystal_2_name
        ),
        fields(
            name = name.as_ref(),
            crystal_0_name = crystal_0_name.as_ref(),
            crystal_1_name = crystal_1_name.as_ref(),
            crystal_2_name = crystal_2_name.as_ref()
        ),
        err(Display, level = "warn")
    )]
    pub async fn create_classic_style_ship_split(
        &self,
        name: impl AsRef<str>,
        crystal_0_name: impl AsRef<str>,
        crystal_1_name: impl AsRef<str>,
        crystal_2_name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<ControllableId, GameError>>, GameError> {
        check_name_or_err(name.as_ref())?;

        let session = self
            .send_command_with_payload(0x80, |writer| {
                writer.write_string_with_len_prefix(name.as_ref());
                writer.write_string_with_len_prefix(crystal_0_name.as_ref());
                writer.write_string_with_len_prefix(crystal_1_name.as_ref());
                writer.write_string_with_len_prefix(crystal_2_name.as_ref());
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                Ok(packet.read(|reader| ControllableId(reader.read_byte())))
            })
        })
    }

    /// Requests the current account-wide crystal snapshot.
    #[inline]
    pub async fn request_crystals(&self) -> Result<Vec<Crystal>, GameError> {
        self.request_crystals_split().await?.await
    }

    /// Requests the current account-wide crystal snapshot.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn request_crystals_split(
        &self,
    ) -> Result<impl Future<Output = Result<Vec<Crystal>, GameError>>, GameError> {
        let session = self.send_command_with_payload(0xA0, |_| {}).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                packet.read(|reader| Self::read_crystal_snapshot(reader))
            })
        })
    }

    /// Set the target scanner configuration on the server.
    #[inline]
    pub async fn static_scanner_subsystem_set(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        width: f32,
        length: f32,
        angle_offset: f32,
    ) -> Result<(), GameError> {
        self.static_scanner_subsystem_set_split(controllable, slot, width, length, angle_offset)
            .await?
            .await
    }

    /// Set the target scanner configuration on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_scanner_subsystem_set_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        width: f32,
        length: f32,
        angle_offset: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA2, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
                writer.write_f32(width);
                writer.write_f32(length);
                writer.write_f32(angle_offset);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the scanner on.
    #[inline]
    pub async fn static_scanner_subsystem_on(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<(), GameError> {
        self.static_scanner_subsystem_on_split(controllable, slot)
            .await?
            .await
    }

    /// Turns the scanner on.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_scanner_subsystem_on_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA3, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the scanner off.
    #[inline]
    pub async fn static_scanner_subsystem_off(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<(), GameError> {
        self.static_scanner_subsystem_off_split(controllable, slot)
            .await?
            .await
    }

    /// Turns the scanner off.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_scanner_subsystem_off_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA4, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Requests one shot for the next server tick.
    #[inline]
    pub async fn static_shot_launcher_subsystem_shoot(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        relative_speed: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        self.static_shot_launcher_subsystem_shoot_split(
            controllable,
            slot,
            relative_speed,
            ticks,
            load,
            damage,
        )
        .await?
        .await
    }

    /// Requests one shot for the next server tick.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_shot_launcher_subsystem_shoot_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        relative_speed: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA5, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
                writer.write_f32(relative_speed);
                writer.write_uint16(ticks);
                writer.write_f32(load);
                writer.write_f32(damage);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Sets the shot fabrication rate on the server.
    #[inline]
    pub async fn static_shot_fabricator_subsystem_set(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        rate: f32,
    ) -> Result<(), GameError> {
        self.static_shot_fabricator_subsystem_set_split(controllable, slot, rate)
            .await?
            .await
    }

    /// Sets the shot fabrication rate on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_shot_fabricator_subsystem_set_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA6, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
                writer.write_f32(rate);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the shot fabricator on.
    #[inline]
    pub async fn static_shot_fabricator_subsystem_on(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<(), GameError> {
        self.static_shot_fabricator_subsystem_on_split(controllable, slot)
            .await?
            .await
    }

    /// Turns the shot fabricator on.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_shot_fabricator_subsystem_on_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA7, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the shot fabricator off.
    #[inline]
    pub async fn static_shot_fabricator_subsystem_off(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<(), GameError> {
        self.static_shot_fabricator_subsystem_off_split(controllable, slot)
            .await?
            .await
    }

    /// Turns the shot fabricator off.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_shot_fabricator_subsystem_off_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA8, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Requests one shot for the next server tick.
    #[inline]
    pub async fn static_interceptor_launcher_subsystem_shoot(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        relative_speed: f32,
        angle_offset: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        self.static_interceptor_launcher_subsystem_shoot_split(
            controllable,
            slot,
            relative_speed,
            angle_offset,
            ticks,
            load,
            damage,
        )
        .await?
        .await
    }

    /// Requests one shot for the next server tick.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn static_interceptor_launcher_subsystem_shoot_split(
        &self,
        controllable: ControllableId,
        slot: SubsystemSlot,
        relative_speed: f32,
        angle_offset: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0xA9, |writer| {
                writer.write_byte(controllable.0);
                writer.write_byte(u8::from(slot));
                writer.write_f32(relative_speed);
                writer.write_f32(angle_offset);
                writer.write_uint16(ticks);
                writer.write_f32(load);
                writer.write_f32(damage);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Produces a crystal from nebula cargo.
    ///
    /// Returns `true` if a crystal was created; `false` if the nebula faded.
    #[inline]
    pub async fn produce_crystal(
        &self,
        controllable_id: ControllableId,
        name: impl AsRef<str>,
    ) -> Result<(bool, Vec<Crystal>), GameError> {
        self.produce_crystal_split(controllable_id, name)
            .await?
            .await
    }

    /// Produces a crystal from nebula cargo.
    ///
    /// Returns `true` if a crystal was created; `false` if the nebula faded.
    #[instrument(level = "debug", skip(self, name), fields(name = name.as_ref()), err(Display, level = "warn"))]
    pub async fn produce_crystal_split(
        &self,
        controllable_id: ControllableId,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<(bool, Vec<Crystal>), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x9D, |writer| {
                writer.write_byte(controllable_id.0);
                writer.write_string_with_len_prefix(name.as_ref());
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                packet.read(|reader| {
                    let produced = reader.read_byte() != 0x00;
                    let crystals = Self::read_crystal_snapshot(reader)?;
                    Ok((produced, crystals))
                })
            })
        })
    }

    /// Renames an account-wide crystal.
    #[inline]
    pub async fn rename_crystal(
        &self,
        old_name: impl AsRef<str>,
        new_name: impl AsRef<str>,
    ) -> Result<Vec<Crystal>, GameError> {
        self.rename_crystal_split(old_name, new_name).await?.await
    }

    /// Renames an account-wide crystal.
    #[instrument(
        level = "debug",
        skip(
            self,
            old_name,
            new_name
        ),
        fields(
            old_name = old_name.as_ref(),
            new_name = new_name.as_ref()
        ),
        err(Display, level = "warn")
    )]
    pub async fn rename_crystal_split(
        &self,
        old_name: impl AsRef<str>,
        new_name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<Vec<Crystal>, GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x9E, |writer| {
                writer.write_string_with_len_prefix(old_name.as_ref());
                writer.write_string_with_len_prefix(new_name.as_ref());
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                packet.read(|reader| Self::read_crystal_snapshot(reader))
            })
        })
    }

    /// Destroys an account-wide crystal.
    #[inline]
    pub async fn destroy_crystal(&self, name: impl AsRef<str>) -> Result<Vec<Crystal>, GameError> {
        self.destroy_crystal_split(name).await?.await
    }

    /// Destroys an account-wide crystal.
    #[instrument(level = "debug", skip(self, name), fields(name = name.as_ref()), err(Display, level = "warn"))]
    pub async fn destroy_crystal_split(
        &self,
        name: impl AsRef<str>,
    ) -> Result<impl Future<Output = Result<Vec<Crystal>, GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x9F, |writer| {
                writer.write_string_with_len_prefix(name.as_ref());
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check(response, |mut packet| {
                packet.read(|reader| Self::read_crystal_snapshot(reader))
            })
        })
    }

    fn read_crystal_snapshot(reader: &mut dyn PacketReader) -> Result<Vec<Crystal>, GameError> {
        let count = reader.read_byte();
        let mut crystals = Vec::with_capacity(count as usize);

        for _ in 0..count {
            crystals.push(Crystal {
                name: reader.read_string(),
                hue: reader.read_f32(),
                grade: CrystalGrade::read(reader),
                energy_battery_multiplier: reader.read_f32(),
                ions_battery_multiplier: reader.read_f32(),
                neutrinos_battery_multiplier: reader.read_f32(),
                hull_multiplier: reader.read_f32(),
                shield_multiplier: reader.read_f32(),
                armor_multiplier: reader.read_f32(),
                energy_cell_multiplier: reader.read_f32(),
                ions_cell_multiplier: reader.read_f32(),
                neutrinos_cell_multiplier: reader.read_f32(),
                shot_weapon_production_multiplier: reader.read_f32(),
                interceptor_weapon_production_multiplier: reader.read_f32(),
                crystal_cargo_limit_multiplier: reader.read_f32(),
                locked: reader.read_byte() != 0x00,
            });
        }

        Ok(crystals)
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
    #[instrument(level = "debug", skip(self, xml), fields(xml = xml.as_ref()), err(Display, level = "warn"))]
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
                writer.write_string_with_len_prefix(xml);
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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

    #[instrument(
        level = "debug",
        skip(self, progress_state),
        err(Display, level = "warn")
    )]
    pub async fn query_cluster_editable_units(
        &self,
        cluster: ClusterId,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<EditableUnitSummary>, GameError> {
        ChunkedTransfer::download_items(
            |offset, maximum_count| {
                self.send_command_with_payload(0x27, move |writer| {
                    writer.write_byte(cluster.0);
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_count);
                })
            },
            |reader| {
                Ok(EditableUnitSummary {
                    kind: UnitKind::read(reader),
                    name: reader.read_string(),
                })
            },
            progress_state,
            ChunkedTransfer::EDITABLE_UNIT_CHUNK_MAXIMUM_COUNT,
            "editable unit query result",
        )
        .await
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
    #[instrument(level = "debug", skip(self, xml), fields(xml = xml.as_ref()), err(Display, level = "warn"))]
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
                writer.write_string_with_len_prefix(xml);
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
    #[instrument(level = "debug", skip(self, name), fields(name = name.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self, name), fields(name = name.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self, xml), fields(xml = xml.as_ref()), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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

    /// Sets the repair rate on the server.
    #[inline]
    pub async fn repair_subsystem_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.repair_subsystem_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the repair rate on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn repair_subsystem_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x93);
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

    /// Sets the mining rate on the server.
    #[inline]
    pub async fn resource_miner_subsystem_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.resource_miner_subsystem_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the mining rate on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn resource_miner_subsystem_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x94);
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

    /// Requests a worm-hole jump on the server.
    #[inline]
    pub async fn jump_drive_subsystem_jump(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.jump_drive_subsystem_jump_split(controllable)
            .await?
            .await
    }

    /// Requests a worm-hole jump on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn jump_drive_subsystem_jump_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let mut packet = Packet::default();
        packet.header_mut().set_command(0x95);
        packet.write(|writer| {
            writer.write_byte(controllable.0);
        });

        let session = self.send_packet_on_new_session(packet).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Requests one interceptor for the next server tick.
    #[inline]
    pub async fn dynamic_shot_interceptor_subsystem_shoot(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        self.dynamic_shot_interceptor_subsystem_shoot_split(
            controllable,
            relative_movement,
            ticks,
            load,
            damage,
        )
        .await?
        .await
    }

    /// Requests one interceptor for the next server tick.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn dynamic_shot_interceptor_subsystem_shoot_split(
        &self,
        controllable: ControllableId,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x96, |writer| {
                writer.write_byte(controllable.0);
                relative_movement.write(writer);
                writer.write_uint16(ticks);
                writer.write_f32(load);
                writer.write_f32(damage);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Sets the interceptor fabrication rate on the server.
    #[inline]
    pub async fn dynamic_interceptor_fabricator_subsystem_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.dynamic_interceptor_fabricator_subsystem_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the interceptor fabrication rate on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn dynamic_interceptor_fabricator_subsystem_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x97, |writer| {
                writer.write_byte(controllable.0);
                writer.write_f32(rate);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the interceptor fabricator on.
    #[inline]
    pub async fn dynamic_interceptor_fabricator_subsystem_on(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dynamic_interceptor_fabricator_subsystem_on_split(controllable)
            .await?
            .await
    }

    /// Turns the interceptor fabricator on.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn dynamic_interceptor_fabricator_subsystem_on_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x98, |writer| {
                writer.write_byte(controllable.0);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Turns the interceptor fabricator off.
    #[inline]
    pub async fn dynamic_interceptor_fabricator_subsystem_off(
        &self,
        controllable: ControllableId,
    ) -> Result<(), GameError> {
        self.dynamic_interceptor_fabricator_subsystem_off_split(controllable)
            .await?
            .await
    }

    /// Turns the interceptor fabricator off.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn dynamic_interceptor_fabricator_subsystem_off_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x99, |writer| {
                writer.write_byte(controllable.0);
            })
            .await?;

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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
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

    /// Fires the railgun forward.
    #[inline]
    pub async fn fire_railgun_front(&self, controllable: ControllableId) -> Result<(), GameError> {
        self.fire_railgun_front_split(controllable).await?.await
    }

    /// Fires the railgun forward.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn fire_railgun_front_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.fire_railgun_split(controllable, 0x9A).await
    }

    /// Fires the railgun backward.
    #[inline]
    pub async fn fire_railgun_back(&self, controllable: ControllableId) -> Result<(), GameError> {
        self.fire_railgun_back_split(controllable).await?.await
    }

    /// Fires the railgun backward.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn fire_railgun_back_split(
        &self,
        controllable: ControllableId,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.fire_railgun_split(controllable, 0x9B).await
    }

    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    async fn fire_railgun_split(
        &self,
        controllable: ControllableId,
        command: u8,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(command, |writer| {
                writer.write_byte(controllable.0);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Sets the target nebula-collection rate on the server.
    #[inline]
    pub async fn nebula_collector_set(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<(), GameError> {
        self.nebula_collector_set_split(controllable, rate)
            .await?
            .await
    }

    /// Sets the target nebula-collection rate on the server.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn nebula_collector_set_split(
        &self,
        controllable: ControllableId,
        rate: f32,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self
            .send_command_with_payload(0x9C, |writer| {
                writer.write_byte(controllable.0);
                writer.write_f32(rate);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Configures a tournament from a typed connector-side description.
    #[inline]
    pub async fn galaxy_configure_tournament(
        &self,
        configuration: &TournamentConfiguration,
    ) -> Result<(), GameError> {
        self.galaxy_configure_tournament_split(configuration)
            .await?
            .await
    }

    /// Configures a tournament from a typed connector-side description.
    #[instrument(level = "debug", skip(self), err(Display, level = "warn"))]
    pub async fn galaxy_configure_tournament_split(
        &self,
        configuration: &TournamentConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Tournament {
            mode: u8,
            duration_ticks: u32,
            #[serde(flatten)]
            teams: Vec<Team>,
            #[serde(flatten)]
            accounts: Vec<Account>,
            #[serde(flatten)]
            match_history: Vec<Match>,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Team {
            id: u8,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Account {
            id: i32,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Match {
            winner_team_id: u8,
        }

        let tournament = Tournament {
            mode: u8::from(configuration.mode()),
            duration_ticks: configuration.duration_ticks(),
            teams: configuration
                .teams()
                .iter()
                .map(|t| Team {
                    id: t.team().id().0,
                })
                .collect(),
            accounts: configuration
                .teams()
                .iter()
                .flat_map(|t| t.account_ids().iter().map(|a| Account { id: a.0 }))
                .collect(),
            match_history: configuration
                .winning_team_ids()
                .iter()
                .map(|t| Match {
                    winner_team_id: t.0,
                })
                .collect(),
        };

        let xml = serde_xml_rs::to_string(&tournament).unwrap();
        let session = self
            .send_command_with_payload(0x60, |writer| {
                writer.write_string_with_len_prefix(&xml);
            })
            .await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Advances the configured tournament from preparation into the commencing stage.
    #[inline]
    pub async fn galaxy_commence_tournament(&self) -> Result<(), GameError> {
        self.galaxy_commence_tournament_split().await?.await
    }

    /// Advances the configured tournament from preparation into the commencing stage.
    #[inline]
    pub async fn galaxy_commence_tournament_split(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self.send_command_with_payload(0x61, |_| {}).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Starts a previously commenced tournament so that it enters the running stage.
    #[inline]
    pub async fn galaxy_start_tournament(&self) -> Result<(), GameError> {
        self.galaxy_start_tournament_split().await?.await
    }

    /// Starts a previously commenced tournament so that it enters the running stage.
    #[inline]
    pub async fn galaxy_start_tournament_split(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self.send_command_with_payload(0x62, |_| {}).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Removes the currently configured tournament from the galaxy.
    #[inline]
    pub async fn galaxy_cancel_tournament(&self) -> Result<(), GameError> {
        self.galaxy_cancel_tournament_split().await?.await
    }

    /// Removes the currently configured tournament from the galaxy.
    #[inline]
    pub async fn galaxy_cancel_tournament_split(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        let session = self.send_command_with_payload(0x63, |_| {}).await?;

        Ok(async move {
            let response = session.response().await?;
            GameError::check_ok(response)
        })
    }

    /// Queries the account list that the server exposes for tournament tooling.
    #[instrument(
        level = "debug",
        skip(self, galaxy, progress_state),
        err(Display, level = "warn")
    )]
    pub async fn galaxy_query_accounts(
        &self,
        galaxy: &Arc<Galaxy>,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<Arc<Account>>, GameError> {
        ChunkedTransfer::download_items(
            |offset, maximum_length| {
                self.send_command_with_payload(0x64, move |writer| {
                    writer.write_int32(offset);
                    writer.write_uint16(maximum_length);
                })
            },
            |reader| Ok(Arc::new(Account::try_read(Arc::downgrade(galaxy), reader)?)),
            progress_state,
            ChunkedTransfer::ACCOUNT_CHUNK_MAXIMUM_COUNT,
            "account query result".to_string(),
        )
        .await
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
    pub(crate) async fn send_command_with_payload(
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
    pub(crate) async fn send_packet_on_new_session(
        &self,
        mut packet: Packet,
    ) -> Result<Session, GameError> {
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
