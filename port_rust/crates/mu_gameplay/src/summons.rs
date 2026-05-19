use bevy::prelude::{App, Plugin, Resource};

use crate::mounts::MountKind;

const SUMMON_LEVEL_TIER_ONE: u8 = 7;
const SUMMON_LEVEL_TIER_TWO: u8 = 11;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SummonKind {
    #[default]
    Explosion,
    Requiem,
    Pollution,
}

impl SummonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Explosion => "explosion",
            Self::Requiem => "requiem",
            Self::Pollution => "pollution",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSummonPose {
    #[default]
    Standard,
    Uniria,
    Dinorant,
    Fenrir,
}

impl PlayerSummonPose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Uniria => "uniria",
            Self::Dinorant => "dinorant",
            Self::Fenrir => "fenrir",
        }
    }

    pub fn from_mount(mount: MountKind) -> Self {
        match mount {
            MountKind::Uniria => Self::Uniria,
            MountKind::Dinorant => Self::Dinorant,
            MountKind::Fenrir => Self::Fenrir,
            MountKind::None | MountKind::DarkHorse => Self::Standard,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SummonCastState {
    pub kind: SummonKind,
    pub weapon_level_tier: u8,
    pub source_x: f32,
    pub source_y: f32,
    pub target_x: f32,
    pub target_y: f32,
}

impl SummonCastState {
    pub fn summary(&self) -> String {
        format!(
            "kind={}|tier={}|source=({:.1},{:.1})|target=({:.1},{:.1})",
            self.kind.as_str(),
            self.weapon_level_tier,
            self.source_x,
            self.source_y,
            self.target_x,
            self.target_y,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SummonManager {
    player_pose: PlayerSummonPose,
    last_cast: Option<SummonCastState>,
}

impl Default for SummonManager {
    fn default() -> Self {
        Self {
            player_pose: PlayerSummonPose::Standard,
            last_cast: None,
        }
    }
}

impl Resource for SummonManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct SummonPlugin;

impl Plugin for SummonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SummonManager>();
    }
}

impl SummonManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn player_pose(&self) -> PlayerSummonPose {
        self.player_pose
    }

    pub fn last_cast(&self) -> Option<&SummonCastState> {
        self.last_cast.as_ref()
    }

    pub fn set_player_pose(&mut self, mount: MountKind) {
        self.player_pose = PlayerSummonPose::from_mount(mount);
    }

    pub fn clear_last_cast(&mut self) {
        self.last_cast = None;
    }

    pub fn cast(
        &mut self,
        kind: SummonKind,
        weapon_level: u8,
        source_x: f32,
        source_y: f32,
        target_x: f32,
        target_y: f32,
    ) -> SummonCastState {
        let cast = SummonCastState {
            kind,
            weapon_level_tier: summon_weapon_level_tier(weapon_level),
            source_x,
            source_y,
            target_x,
            target_y,
        };
        self.last_cast = Some(cast.clone());
        cast
    }

    pub fn snapshot(&self) -> String {
        let last_cast = self
            .last_cast
            .as_ref()
            .map_or_else(|| "none".to_string(), SummonCastState::summary);

        format!("pose={}|last_cast={}", self.player_pose.as_str(), last_cast,)
    }
}

pub fn summon_weapon_level_tier(weapon_level: u8) -> u8 {
    if weapon_level >= SUMMON_LEVEL_TIER_TWO {
        2
    } else if weapon_level >= SUMMON_LEVEL_TIER_ONE {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        summon_weapon_level_tier, PlayerSummonPose, SummonKind, SummonManager, SummonPlugin,
    };
    use crate::mounts::MountKind;
    use bevy::prelude::App;

    #[test]
    fn summon_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(SummonPlugin);

        let manager = app.world().resource::<SummonManager>();
        assert_eq!(manager.player_pose(), PlayerSummonPose::Standard);
    }

    #[test]
    fn summon_pose_follows_the_mount_kind() {
        let mut manager = SummonManager::new();
        manager.set_player_pose(MountKind::Uniria);
        assert_eq!(manager.player_pose(), PlayerSummonPose::Uniria);

        manager.set_player_pose(MountKind::Dinorant);
        assert_eq!(manager.player_pose(), PlayerSummonPose::Dinorant);

        manager.set_player_pose(MountKind::DarkHorse);
        assert_eq!(manager.player_pose(), PlayerSummonPose::Standard);
    }

    #[test]
    fn summon_weapon_tier_matches_the_legacy_thresholds() {
        assert_eq!(summon_weapon_level_tier(0), 0);
        assert_eq!(summon_weapon_level_tier(6), 0);
        assert_eq!(summon_weapon_level_tier(7), 1);
        assert_eq!(summon_weapon_level_tier(10), 1);
        assert_eq!(summon_weapon_level_tier(11), 2);
        assert_eq!(summon_weapon_level_tier(15), 2);
    }

    #[test]
    fn summon_cast_snapshot_tracks_the_last_effect() {
        let mut manager = SummonManager::new();
        let cast = manager.cast(SummonKind::Pollution, 12, 100.0, 200.0, 300.0, 400.0);

        assert_eq!(cast.weapon_level_tier, 2);
        assert_eq!(
            manager.snapshot(),
            "pose=standard|last_cast=kind=pollution|tier=2|source=(100.0,200.0)|target=(300.0,400.0)"
        );
    }
}
