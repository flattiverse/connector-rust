use crate::utils::Atomic;

/// The live score state of a player or team inside one galaxy session.
#[derive(Debug, Clone, Default)]
pub struct Score {
    kills: Atomic<u16>,
    deaths: Atomic<u16>,
    mission: Atomic<u16>,
}

impl Score {
    pub(crate) fn update(&self, kills: u16, deaths: u16, mission: u16) {
        self.kills.store(kills);
        self.deaths.store(deaths);
        self.mission.store(mission);
    }

    /// Number of kills in the current galaxy runtime.
    #[inline]
    pub fn kills(&self) -> u16 {
        self.kills.load()
    }

    /// Number of deaths in the current galaxy runtime.
    #[inline]
    pub fn deaths(&self) -> u16 {
        self.deaths.load()
    }

    /// Number of mission points in the current galaxy runtime.
    #[inline]
    pub fn mission(&self) -> u16 {
        self.mission.load()
    }
}
