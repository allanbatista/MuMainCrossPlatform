pub const RENDER_DISTANCE_MULTIPLIER: f32 = 1.4;

pub const REFERENCE_ASPECT_RATIO: f32 = 4.0 / 3.0;

pub fn hfov_to_vfov(horizontal_fov_degrees: f32, _aspect_ratio: f32) -> f32 {
    let half_horizontal_radians = horizontal_fov_degrees.to_radians() * 0.5;
    let half_vertical_radians = (half_horizontal_radians.tan() / REFERENCE_ASPECT_RATIO).atan();
    half_vertical_radians.to_degrees() * 2.0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraConfig {
    pub h_fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect_ratio: f32,
    pub terrain_cull_range: f32,
    pub object_cull_range: f32,
    pub fog_start: f32,
    pub fog_end: f32,
}

impl CameraConfig {
    pub fn for_main_scene_default_camera() -> Self {
        let far_plane = 3000.0;

        Self {
            h_fov: 40.0,
            near_plane: 20.0,
            far_plane,
            aspect_ratio: REFERENCE_ASPECT_RATIO,
            terrain_cull_range: far_plane * RENDER_DISTANCE_MULTIPLIER,
            object_cull_range: far_plane,
            fog_start: far_plane,
            fog_end: far_plane * 1.25,
        }
    }

    pub fn for_main_scene_orbital_camera() -> Self {
        let far_plane = 3800.0;

        Self {
            h_fov: 40.0,
            near_plane: 20.0,
            far_plane,
            aspect_ratio: REFERENCE_ASPECT_RATIO,
            terrain_cull_range: far_plane * RENDER_DISTANCE_MULTIPLIER,
            object_cull_range: far_plane,
            fog_start: far_plane,
            fog_end: far_plane * 1.25,
        }
    }

    pub fn for_login_scene() -> Self {
        let far_plane = 20000.0;

        Self {
            h_fov: 90.0,
            near_plane: 10.0,
            far_plane,
            aspect_ratio: REFERENCE_ASPECT_RATIO,
            terrain_cull_range: far_plane,
            object_cull_range: far_plane,
            fog_start: far_plane,
            fog_end: far_plane * 1.25,
        }
    }

    pub fn for_character_scene() -> Self {
        let far_plane = 4100.0;

        Self {
            h_fov: 40.0,
            near_plane: 10.0,
            far_plane,
            aspect_ratio: REFERENCE_ASPECT_RATIO,
            terrain_cull_range: far_plane,
            object_cull_range: far_plane,
            fog_start: far_plane,
            fog_end: far_plane * 1.25,
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self::for_main_scene_default_camera()
    }
}

#[cfg(test)]
mod tests {
    use super::{hfov_to_vfov, CameraConfig, REFERENCE_ASPECT_RATIO, RENDER_DISTANCE_MULTIPLIER};

    const EPSILON: f32 = 0.01;

    #[test]
    fn hfov_conversion_matches_reference_aspect() {
        let vfov = hfov_to_vfov(40.0, 16.0 / 9.0);

        assert!((vfov - 30.53694).abs() < EPSILON, "vfov={vfov}");
    }

    #[test]
    fn main_scene_default_camera_matches_legacy_preset() {
        let config = CameraConfig::for_main_scene_default_camera();

        assert_eq!(config.h_fov, 40.0);
        assert_eq!(config.near_plane, 20.0);
        assert_eq!(config.far_plane, 3000.0);
        assert_eq!(config.aspect_ratio, REFERENCE_ASPECT_RATIO);
        assert_eq!(
            config.terrain_cull_range,
            3000.0 * RENDER_DISTANCE_MULTIPLIER
        );
        assert_eq!(config.object_cull_range, 3000.0);
        assert_eq!(config.fog_start, 3000.0);
        assert_eq!(config.fog_end, 3750.0);
    }

    #[test]
    fn orbital_camera_matches_legacy_visibility_values() {
        let config = CameraConfig::for_main_scene_orbital_camera();

        assert_eq!(config.far_plane, 3800.0);
        assert_eq!(config.terrain_cull_range, 5320.0);
        assert_eq!(config.object_cull_range, 3800.0);
    }

    #[test]
    fn login_and_character_presets_keep_scene_specific_far_planes() {
        let login = CameraConfig::for_login_scene();
        let character = CameraConfig::for_character_scene();

        assert_eq!(login.far_plane, 20000.0);
        assert_eq!(character.far_plane, 4100.0);
    }
}
