use crate::galaxy_hierarchy::Galaxy;
use crate::network::PacketReader;
use crate::{GameError, ProgressState};
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct AccountId(pub(crate) i32);

#[derive(Debug, Clone)]
pub struct Account {
    galaxy: Weak<Galaxy>,
    id: AccountId,
    name: String,
    admin: bool,
    rank: i32,
    player_kills: i64,
    player_deaths: i64,
    has_avatar: bool,
    tournament_elo: Option<f32>,
}

impl Account {
    #[inline]
    pub(crate) fn try_read(
        galaxy: Weak<Galaxy>,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(Self {
            galaxy,
            id: AccountId(reader.read_int32()),
            name: reader.read_string(),
            admin: reader.read_byte() != 0x00,
            rank: reader.read_int32(),
            player_kills: reader.read_int64(),
            player_deaths: reader.read_int64(),
            has_avatar: reader.read_byte() != 0x00,
            tournament_elo: {
                if reader.read_byte() != 0x0 {
                    Some(reader.read_f32())
                } else {
                    None
                }
            },
        })
    }

    /// Stable account id from persistent galaxy storage.
    #[inline]
    pub fn id(&self) -> AccountId {
        self.id
    }

    /// Account name as currently known by the galaxy.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the account currently has admin permissions.
    #[inline]
    pub fn admin(&self) -> bool {
        self.admin
    }

    /// Global account rank mirrored from persistent storage.
    #[inline]
    pub fn rank(&self) -> i32 {
        self.rank
    }

    /// Lifetime player-kill statistic stored on the account.
    #[inline]
    pub fn player_kills(&self) -> i64 {
        self.player_kills
    }

    /// Lifetime player-death statistic stored on the account.
    #[inline]
    pub fn player_deaths(&self) -> i64 {
        self.player_deaths
    }

    /// Whether the account currently has a persisted avatar that can be downloaded.
    #[inline]
    pub fn has_avatar(&self) -> bool {
        self.has_avatar
    }

    /// Tournament Elo mirrored by the server, or `None` if no tournament rating is stored.
    #[inline]
    pub fn tournament_elo(&self) -> Option<f32> {
        self.tournament_elo
    }

    /// Downloads the small persisted avatar image of this account.
    pub async fn download_small_avatar(
        &self,
        progress_state: impl Into<Option<Arc<ProgressState>>>,
    ) -> Result<Vec<u8>, GameError> {
        self.galaxy
            .upgrade()
            .unwrap()
            .connection()
            .download_account_small_avatar(self.id, progress_state.into())
            .await
    }

    /// Downloads the large persisted avatar image of this account.
    pub async fn download_big_avatar(
        &self,
        progress_state: impl Into<Option<Arc<ProgressState>>>,
    ) -> Result<Vec<u8>, GameError> {
        self.galaxy
            .upgrade()
            .unwrap()
            .connection()
            .download_account_big_avatar(self.id, progress_state.into())
            .await
    }
}
