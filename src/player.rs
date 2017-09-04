
use std::sync::Weak;
use std::sync::RwLock;

use Scores;
use Version;
use Connector;
use PlatformKind;
use PerformanceMark;
use UniversalHolder;
use dotnet::TimeSpan;

pub struct Player {
    name:        String,
    platform:    PlatformKind,
    version:     Version,
    performance: PerformanceMark,

    id:     u16,
    rank:   u32,
    level:   u8,
    elo:    i32,

    game_scores:         Scores,
    player_scores:       Scores,
    clan:                Option<String>,
    average_commit_time: TimeSpan,
    last_commit_time:    TimeSpan,
    ping:                TimeSpan,

    connector:      Weak<Connector>,
    universe_group: Weak<RwLock<UniverseGroup>>,
    team:           Weak<RwLock<Team>>,

    active: bool,
    online: bool,

    controllables: RwLock<UniversalHolder<ControllableInfo>>
}

impl Player {

    pub fn controllable_info(&self, index: u8) -> Option<Arc<RwLock<ControllableInfo>>> {

    }

    pub fn universe_group(&self) -> &Weak<RwLock<UniverseGroup>> {
        &self.universe_group
    }
}