use crate::utils::Atomic;

/// The live score state of a player or team inside one galaxy session.
#[derive(Debug, Clone, Default)]
pub struct Score {
    player_kills: Atomic<u16>,
    player_deaths: Atomic<u16>,
    friendly_kills: Atomic<u16>,
    friendly_deaths: Atomic<u16>,
    npc_kills: Atomic<u16>,
    npc_deaths: Atomic<u16>,
    neutral_deaths: Atomic<u16>,
    mission: Atomic<u16>,
}

impl Score {
    pub(crate) fn update(
        &self,
        player_kills: u16,
        player_deaths: u16,
        friendly_kills: u16,
        friendly_deaths: u16,
        npc_kills: u16,
        npc_deaths: u16,
        neutral_deaths: u16,
        mission: u16,
    ) {
        self.player_kills.store(player_kills);
        self.player_deaths.store(player_deaths);
        self.friendly_kills.store(friendly_kills);
        self.friendly_deaths.store(friendly_deaths);
        self.npc_kills.store(npc_kills);
        self.npc_deaths.store(npc_deaths);
        self.neutral_deaths.store(neutral_deaths);
        self.mission.store(mission);
    }

    /// Number of kills of enemy players in the current galaxy runtime.
    #[inline]
    pub fn player_kills(&self) -> u16 {
        self.player_kills.load()
    }

    /// Number of deaths caused by enemy players in the current galaxy runtime.
    #[inline]
    pub fn player_deaths(&self) -> u16 {
        self.player_deaths.load()
    }

    /// Number of kills of teammates in the current galaxy runtime.
    #[inline]
    pub fn friendly_kills(&self) -> u16 {
        self.friendly_kills.load()
    }

    /// Number of deaths caused by the same team, including self-inflicted deaths, in the current
    /// galaxy runtime.
    #[inline]
    pub fn friendly_deaths(&self) -> u16 {
        self.friendly_deaths.load()
    }

    /// Number of kills of hostile NPCs in the current galaxy runtime.
    #[inline]
    pub fn npc_kills(&self) -> u16 {
        self.npc_kills.load()
    }

    /// Number of deaths caused by hostile NPCs in the current galaxy runtime.
    #[inline]
    pub fn npc_deaths(&self) -> u16 {
        self.npc_deaths.load()
    }

    /// Number of deaths caused by neutral units or the environment in the current galaxy runtime.
    #[inline]
    pub fn neutral_deaths(&self) -> u16 {
        self.neutral_deaths.load()
    }

    /// Number of mission points in the current galaxy runtime.
    #[inline]
    pub fn mission(&self) -> u16 {
        self.mission.load()
    }
}
