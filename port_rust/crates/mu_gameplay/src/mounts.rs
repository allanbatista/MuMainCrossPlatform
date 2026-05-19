use bevy::prelude::{App, Plugin, Resource};

const GROUND_MOUNT_CAMERA_OFFSET: f32 = 30.0;
const MOUNT_TRANSITION_MS: f32 = 500.0;
const MOUNT_LERP_TAU: f32 = MOUNT_TRANSITION_MS / 3000.0;
const MOUNT_LERP_SNAP_THRESHOLD: f32 = 0.5;
const REFERENCE_FPS: f32 = 25.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MountKind {
    #[default]
    None,
    Uniria,
    Dinorant,
    DarkHorse,
    Fenrir,
}

impl MountKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Uniria => "uniria",
            Self::Dinorant => "dinorant",
            Self::DarkHorse => "dark-horse",
            Self::Fenrir => "fenrir",
        }
    }

    pub fn target_camera_offset(self) -> f32 {
        match self {
            Self::None | Self::Dinorant => 0.0,
            Self::Uniria | Self::DarkHorse | Self::Fenrir => GROUND_MOUNT_CAMERA_OFFSET,
        }
    }

    pub fn is_ground_mount(self) -> bool {
        matches!(self, Self::Uniria | Self::DarkHorse | Self::Fenrir)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MountManager {
    mount: MountKind,
    safe_zone: bool,
    world_active: bool,
    current_camera_offset: f32,
}

impl Default for MountManager {
    fn default() -> Self {
        Self {
            mount: MountKind::None,
            safe_zone: false,
            world_active: true,
            current_camera_offset: 0.0,
        }
    }
}

impl Resource for MountManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct MountPlugin;

impl Plugin for MountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MountManager>();
    }
}

impl MountManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mount(&self) -> MountKind {
        self.mount
    }

    pub fn set_mount(&mut self, mount: MountKind) {
        self.mount = mount;
    }

    pub fn safe_zone(&self) -> bool {
        self.safe_zone
    }

    pub fn set_safe_zone(&mut self, safe_zone: bool) {
        self.safe_zone = safe_zone;
    }

    pub fn world_active(&self) -> bool {
        self.world_active
    }

    pub fn set_world_active(&mut self, world_active: bool) {
        self.world_active = world_active;
    }

    pub fn current_camera_offset(&self) -> f32 {
        self.current_camera_offset
    }

    pub fn target_camera_offset(&self) -> f32 {
        mount_camera_offset(self.mount, self.safe_zone, self.world_active)
    }

    pub fn sync_camera_offset(&mut self) {
        self.current_camera_offset = self.target_camera_offset();
    }

    pub fn advance_camera_offset(&mut self, fps_animation_factor: f32) {
        let target = self.target_camera_offset();
        let delta = target - self.current_camera_offset;

        if delta.abs() < MOUNT_LERP_SNAP_THRESHOLD {
            self.current_camera_offset = target;
            return;
        }

        let dt = fps_animation_factor / REFERENCE_FPS;
        let blend = 1.0 - (-dt / MOUNT_LERP_TAU).exp();
        self.current_camera_offset += delta * blend;
    }

    pub fn snapshot(&self) -> String {
        format!(
            "mount={}|safe_zone={}|world_active={}|current_offset={:.2}|target_offset={:.2}",
            self.mount.as_str(),
            self.safe_zone,
            self.world_active,
            self.current_camera_offset,
            self.target_camera_offset(),
        )
    }
}

pub fn mount_camera_offset(mount: MountKind, safe_zone: bool, world_active: bool) -> f32 {
    if safe_zone || !world_active {
        return 0.0;
    }

    mount.target_camera_offset()
}

#[cfg(test)]
mod tests {
    use super::{mount_camera_offset, MountKind, MountManager, MountPlugin};
    use bevy::prelude::App;

    #[test]
    fn mount_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(MountPlugin);

        let manager = app.world().resource::<MountManager>();
        assert_eq!(manager.mount(), MountKind::None);
        assert!(manager.world_active());
    }

    #[test]
    fn mount_camera_offsets_match_legacy_rules() {
        assert_eq!(mount_camera_offset(MountKind::None, false, true), 0.0);
        assert_eq!(mount_camera_offset(MountKind::Dinorant, false, true), 0.0);
        assert_eq!(mount_camera_offset(MountKind::Uniria, false, true), 30.0);
        assert_eq!(mount_camera_offset(MountKind::DarkHorse, false, true), 30.0);
        assert_eq!(mount_camera_offset(MountKind::Fenrir, false, true), 30.0);
        assert_eq!(mount_camera_offset(MountKind::Fenrir, true, true), 0.0);
        assert_eq!(mount_camera_offset(MountKind::Fenrir, false, false), 0.0);
    }

    #[test]
    fn mount_offset_smoothly_approaches_the_target() {
        let mut manager = MountManager::new();
        manager.set_mount(MountKind::Fenrir);
        manager.advance_camera_offset(1.0);

        assert!(manager.current_camera_offset() > 0.0);
        assert!(manager.current_camera_offset() < 30.0);
    }

    #[test]
    fn mount_snapshot_reflects_the_current_camera_state() {
        let mut manager = MountManager::new();
        manager.set_mount(MountKind::DarkHorse);
        manager.sync_camera_offset();

        assert_eq!(
            manager.snapshot(),
            "mount=dark-horse|safe_zone=false|world_active=true|current_offset=30.00|target_offset=30.00"
        );
    }
}
