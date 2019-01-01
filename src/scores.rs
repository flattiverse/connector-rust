
use std::fmt;
use std::sync::RwLock;

use crate::Error;
use crate::net::BinaryReader;


#[derive(Debug)]
pub struct ScoresMut {
    kill_enemy_ai_platform: u32,
    kill_enemy_ai_probe: u32,
    kill_enemy_ai_drone: u32,
    kill_enemy_ai_ship: u32,
    kill_enemy_ai_base: u32,
    kill_enemy_ai_creep: u32,

    kill_own_ai_platform: u32,
    kill_own_ai_probe: u32,
    kill_own_ai_drone: u32,
    kill_own_ai_ship: u32,
    kill_own_ai_base: u32,
    kill_own_ai_creep: u32,

    kill_enemy_player_platform: u32,
    kill_enemy_player_probe: u32,
    kill_enemy_player_drone: u32,
    kill_enemy_player_ship: u32,
    kill_enemy_player_base: u32,

    kill_own_player_platform: u32,
    kill_own_player_probe: u32,
    kill_own_player_drone: u32,
    kill_own_player_ship: u32,
    kill_own_player_base: u32,

    death_enemy_ai_platform: u32,
    death_enemy_ai_probe: u32,
    death_enemy_ai_drone: u32,
    death_enemy_ai_ship: u32,
    death_enemy_ai_base: u32,
    death_enemy_ai_creep: u32,

    death_own_ai_platform: u32,
    death_own_ai_probe: u32,
    death_own_ai_drone: u32,
    death_own_ai_ship: u32,
    death_own_ai_base: u32,
    death_own_ai_creep: u32,

    death_enemy_player_platform: u32,
    death_enemy_player_probe: u32,
    death_enemy_player_drone: u32,
    death_enemy_player_ship: u32,
    death_enemy_player_base: u32,

    death_own_player_platform: u32,
    death_own_player_probe: u32,
    death_own_player_drone: u32,
    death_own_player_ship: u32,
    death_own_player_base: u32,

    self_kills: u32,
    suicides: u32,
    death_neutral_units: u32,

    enemy_targets: u32,
    own_targets: u32,
    mission_targets: u32,

    pvp_score: f32,
}


#[derive(Debug)]
pub struct Scores {
    mutable: RwLock<ScoresMut>
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            mutable: RwLock::new(ScoresMut {
                kill_enemy_ai_platform: 0,
                kill_enemy_ai_probe: 0,
                kill_enemy_ai_drone: 0,
                kill_enemy_ai_ship: 0,
                kill_enemy_ai_base: 0,
                kill_enemy_ai_creep: 0,
                kill_own_ai_platform: 0,
                kill_own_ai_probe: 0,
                kill_own_ai_drone: 0,
                kill_own_ai_ship: 0,
                kill_own_ai_base: 0,
                kill_own_ai_creep: 0,
                kill_enemy_player_platform: 0,
                kill_enemy_player_probe: 0,
                kill_enemy_player_drone: 0,
                kill_enemy_player_ship: 0,
                kill_enemy_player_base: 0,
                kill_own_player_platform: 0,
                kill_own_player_probe: 0,
                kill_own_player_drone: 0,
                kill_own_player_ship: 0,
                kill_own_player_base: 0,
                death_enemy_ai_platform: 0,
                death_enemy_ai_probe: 0,
                death_enemy_ai_drone: 0,
                death_enemy_ai_ship: 0,
                death_enemy_ai_base: 0,
                death_enemy_ai_creep: 0,
                death_own_ai_platform: 0,
                death_own_ai_probe: 0,
                death_own_ai_drone: 0,
                death_own_ai_ship: 0,
                death_own_ai_base: 0,
                death_own_ai_creep: 0,
                death_enemy_player_platform: 0,
                death_enemy_player_probe: 0,
                death_enemy_player_drone: 0,
                death_enemy_player_ship: 0,
                death_enemy_player_base: 0,
                death_own_player_platform: 0,
                death_own_player_probe: 0,
                death_own_player_drone: 0,
                death_own_player_ship: 0,
                death_own_player_base: 0,
                self_kills: 0,
                suicides: 0,
                death_neutral_units: 0,
                enemy_targets: 0,
                own_targets: 0,
                mission_targets: 0,
                pvp_score: 0f32,
            })
        }
    }
}

impl Scores {
    pub fn update(&self, reader: &mut BinaryReader) -> Result<(), Error> {
        let mut mutable = self.mutable.write()?;
        mutable.kill_enemy_ai_platform = reader.read_u32()?;
        mutable.kill_enemy_ai_probe    = reader.read_u32()?;
        mutable.kill_enemy_ai_drone    = reader.read_u32()?;
        mutable.kill_enemy_ai_ship     = reader.read_u32()?;
        mutable.kill_enemy_ai_base     = reader.read_u32()?;
        mutable.kill_enemy_ai_creep    = reader.read_u32()?;

        mutable.kill_own_ai_platform   = reader.read_u32()?;
        mutable.kill_own_ai_probe      = reader.read_u32()?;
        mutable.kill_own_ai_drone      = reader.read_u32()?;
        mutable.kill_own_ai_ship       = reader.read_u32()?;
        mutable.kill_own_ai_base       = reader.read_u32()?;
        mutable.kill_own_ai_creep      = reader.read_u32()?;

        mutable.kill_enemy_player_platform = reader.read_u32()?;
        mutable.kill_enemy_player_probe    = reader.read_u32()?;
        mutable.kill_enemy_player_drone    = reader.read_u32()?;
        mutable.kill_enemy_player_ship     = reader.read_u32()?;
        mutable.kill_enemy_player_base     = reader.read_u32()?;

        mutable.kill_own_player_platform   = reader.read_u32()?;
        mutable.kill_own_player_probe      = reader.read_u32()?;
        mutable.kill_own_player_drone      = reader.read_u32()?;
        mutable.kill_own_player_ship       = reader.read_u32()?;
        mutable.kill_own_player_base       = reader.read_u32()?;

        mutable.death_enemy_ai_platform    = reader.read_u32()?;
        mutable.death_enemy_ai_probe       = reader.read_u32()?;
        mutable.death_enemy_ai_drone       = reader.read_u32()?;
        mutable.death_enemy_ai_ship        = reader.read_u32()?;
        mutable.death_enemy_ai_base        = reader.read_u32()?;
        mutable.death_enemy_ai_creep       = reader.read_u32()?;

        mutable.death_own_ai_platform      = reader.read_u32()?;
        mutable.death_own_ai_probe         = reader.read_u32()?;
        mutable.death_own_ai_drone         = reader.read_u32()?;
        mutable.death_own_ai_ship          = reader.read_u32()?;
        mutable.death_own_ai_base          = reader.read_u32()?;
        mutable.death_own_ai_creep         = reader.read_u32()?;

        mutable.death_enemy_player_platform= reader.read_u32()?;
        mutable.death_enemy_player_probe   = reader.read_u32()?;
        mutable.death_enemy_player_drone   = reader.read_u32()?;
        mutable.death_enemy_player_ship    = reader.read_u32()?;
        mutable.death_enemy_player_base    = reader.read_u32()?;

        mutable.death_own_player_platform  = reader.read_u32()?;
        mutable.death_own_player_probe     = reader.read_u32()?;
        mutable.death_own_player_drone     = reader.read_u32()?;
        mutable.death_own_player_ship      = reader.read_u32()?;
        mutable.death_own_player_base      = reader.read_u32()?;

        mutable.self_kills             = reader.read_u32()?;
        mutable.suicides               = reader.read_u32()?;
        mutable.death_neutral_units    = reader.read_u32()?;

        mutable.enemy_targets          = reader.read_u32()?;
        mutable.own_targets            = reader.read_u32()?;
        mutable.mission_targets        = reader.read_u32()?;

        mutable.pvp_score              = reader.read_single()?;

        Ok(())
    }

    pub fn kill_enemy_ai_platform(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_platform }
    pub fn kill_enemy_ai_probe(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_probe }
    pub fn kill_enemy_ai_drone(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_drone }
    pub fn kill_enemy_ai_ship(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_ship }
    pub fn kill_enemy_ai_base(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_base }
    pub fn kill_enemy_ai_creep(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_ai_creep }

    pub fn kill_own_ai_platform(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_platform }
    pub fn kill_own_ai_probe(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_probe }
    pub fn kill_own_ai_drone(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_drone }
    pub fn kill_own_ai_ship(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_ship }
    pub fn kill_own_ai_base(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_base }
    pub fn kill_own_ai_creep(&self) -> u32 { self.mutable.read().unwrap().kill_own_ai_creep }

    pub fn kill_enemy_player_platform(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_player_platform }
    pub fn kill_enemy_player_probe(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_player_probe }
    pub fn kill_enemy_player_drone(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_player_drone }
    pub fn kill_enemy_player_ship(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_player_ship }
    pub fn kill_enemy_player_base(&self) -> u32 { self.mutable.read().unwrap().kill_enemy_player_base }

    pub fn kill_own_player_platform(&self) -> u32 { self.mutable.read().unwrap().kill_own_player_platform }
    pub fn kill_own_player_probe(&self) -> u32 { self.mutable.read().unwrap().kill_own_player_probe }
    pub fn kill_own_player_drone(&self) -> u32 { self.mutable.read().unwrap().kill_own_player_drone }
    pub fn kill_own_player_ship(&self) -> u32 { self.mutable.read().unwrap().kill_own_player_ship }
    pub fn kill_own_player_base(&self) -> u32 { self.mutable.read().unwrap().kill_own_player_base }

    pub fn death_enemy_ai_platform(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_platform }
    pub fn death_enemy_ai_probe(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_probe }
    pub fn death_enemy_ai_drone(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_drone }
    pub fn death_enemy_ai_ship(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_ship }
    pub fn death_enemy_ai_base(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_base }
    pub fn death_enemy_ai_creep(&self) -> u32 { self.mutable.read().unwrap().death_enemy_ai_creep }

    pub fn death_own_ai_platform(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_platform }
    pub fn death_own_ai_probe(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_probe }
    pub fn death_own_ai_drone(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_drone }
    pub fn death_own_ai_ship(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_ship }
    pub fn death_own_ai_base(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_base }
    pub fn death_own_ai_creep(&self) -> u32 { self.mutable.read().unwrap().death_own_ai_creep }

    pub fn death_enemy_player_platform(&self) -> u32 { self.mutable.read().unwrap().death_enemy_player_platform }
    pub fn death_enemy_player_probe(&self) -> u32 { self.mutable.read().unwrap().death_enemy_player_probe }
    pub fn death_enemy_player_drone(&self) -> u32 { self.mutable.read().unwrap().death_enemy_player_drone }
    pub fn death_enemy_player_ship(&self) -> u32 { self.mutable.read().unwrap().death_enemy_player_ship }
    pub fn death_enemy_player_base(&self) -> u32 { self.mutable.read().unwrap().death_enemy_player_base }

    pub fn death_own_player_platform(&self) -> u32 { self.mutable.read().unwrap().death_own_player_platform }
    pub fn death_own_player_probe(&self) -> u32 { self.mutable.read().unwrap().death_own_player_probe }
    pub fn death_own_player_drone(&self) -> u32 { self.mutable.read().unwrap().death_own_player_drone }
    pub fn death_own_player_ship(&self) -> u32 { self.mutable.read().unwrap().death_own_player_ship }
    pub fn death_own_player_base(&self) -> u32 { self.mutable.read().unwrap().death_own_player_base }

    pub fn self_kills(&self) -> u32 { self.mutable.read().unwrap().self_kills }
    pub fn suicides(&self) -> u32 { self.mutable.read().unwrap().suicides }
    pub fn death_neutral_units(&self) -> u32 { self.mutable.read().unwrap().death_neutral_units }

    pub fn enemy_targets(&self) -> u32 { self.mutable.read().unwrap().enemy_targets }
    pub fn own_targets(&self) -> u32 { self.mutable.read().unwrap().own_targets }
    pub fn mission_targets(&self) -> u32 { self.mutable.read().unwrap().mission_targets }

    pub fn pvp_score(&self) -> f32 { self.mutable.read().unwrap().pvp_score }
}

impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mutable.read().unwrap().pvp_score)
    }
}