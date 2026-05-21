use bevy::prelude::{App, Plugin, Resource};
use mu_render::{hfov_to_vfov, CameraConfig};

const DEFAULT_DEFAULT_CAMERA_NEAR_PLANE: f32 = 500.0;
const DEFAULT_DEFAULT_CAMERA_FAR_PLANE: f32 = 3000.0;
const DEFAULT_DEFAULT_CAMERA_FOG_START_PCT: f32 = 1.00;
const DEFAULT_DEFAULT_CAMERA_FOG_END_PCT: f32 = 1.25;
const DEFAULT_ORBITAL_FAR_DIST: f32 = 5000.0;
const DEFAULT_ORBITAL_FAR_WIDTH: f32 = 8000.0;
const DEFAULT_ORBITAL_NEAR_WIDTH: f32 = 800.0;
const DEFAULT_ORBITAL_FOG_START_PCT: f32 = 1.00;
const DEFAULT_ORBITAL_FOG_END_PCT: f32 = 1.25;
const DEFAULT_CULL_RADIUS_ITEM: f32 = 100.0;
const DEFAULT_LOGIN_TERRAIN_DISTANCE: f32 = 3995.0;
const DEFAULT_LOGIN_OBJECT_DISTANCE: f32 = 5903.0;
const REFERENCE_WIDTH: u32 = 640;
const REFERENCE_HEIGHT: u32 = 480;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DevEditorWindowMode {
    #[default]
    Windowed,
    Fullscreen,
}

impl DevEditorWindowMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Windowed => "windowed",
            Self::Fullscreen => "fullscreen",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Windowed => "Windowed",
            Self::Fullscreen => "Fullscreen",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DevEditorDefaultCameraOverride {
    pub enabled: bool,
    pub near_plane: f32,
    pub far_plane: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub width_near_mul: f32,
    pub width_far_mul: f32,
    pub fog_override: bool,
    pub fog_on: bool,
    pub fog_start_pct: f32,
    pub fog_end_pct: f32,
}

impl Default for DevEditorDefaultCameraOverride {
    fn default() -> Self {
        Self {
            enabled: false,
            near_plane: DEFAULT_DEFAULT_CAMERA_NEAR_PLANE,
            far_plane: DEFAULT_DEFAULT_CAMERA_FAR_PLANE,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            width_near_mul: 1.0,
            width_far_mul: 1.0,
            fog_override: false,
            fog_on: true,
            fog_start_pct: DEFAULT_DEFAULT_CAMERA_FOG_START_PCT,
            fog_end_pct: DEFAULT_DEFAULT_CAMERA_FOG_END_PCT,
        }
    }
}

impl DevEditorDefaultCameraOverride {
    pub fn reset_to_main_scene_default_camera(&mut self) {
        let preset = CameraConfig::for_main_scene_default_camera();
        self.near_plane = preset.near_plane;
        self.far_plane = preset.far_plane;
        self.offset_x = 0.0;
        self.offset_y = 0.0;
        self.offset_z = 0.0;
        self.width_near_mul = 1.0;
        self.width_far_mul = 1.0;
        self.fog_start_pct = DEFAULT_DEFAULT_CAMERA_FOG_START_PCT;
        self.fog_end_pct = DEFAULT_DEFAULT_CAMERA_FOG_END_PCT;
    }

    pub fn snapshot(&self) -> String {
        format!(
            "enabled={}|near_plane={:?}|far_plane={:?}|offset={:?}|width_mul={:?}|fog_override={}|fog_on={}|fog_range_pct=({:.2}, {:.2})",
            self.enabled,
            self.near_plane,
            self.far_plane,
            (self.offset_x, self.offset_y, self.offset_z),
            (self.width_near_mul, self.width_far_mul),
            self.fog_override,
            self.fog_on,
            self.fog_start_pct,
            self.fog_end_pct,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DevEditorOrbitalCameraOverride {
    pub enabled: bool,
    pub far_dist: f32,
    pub far_width: f32,
    pub near_dist: f32,
    pub near_width: f32,
    pub fog_on: bool,
    pub fog_start_pct: f32,
    pub fog_end_pct: f32,
}

impl Default for DevEditorOrbitalCameraOverride {
    fn default() -> Self {
        Self {
            enabled: false,
            far_dist: DEFAULT_ORBITAL_FAR_DIST,
            far_width: DEFAULT_ORBITAL_FAR_WIDTH,
            near_dist: 0.0,
            near_width: DEFAULT_ORBITAL_NEAR_WIDTH,
            fog_on: true,
            fog_start_pct: DEFAULT_ORBITAL_FOG_START_PCT,
            fog_end_pct: DEFAULT_ORBITAL_FOG_END_PCT,
        }
    }
}

impl DevEditorOrbitalCameraOverride {
    pub fn reset_to_natural_pyramid(&mut self, camera_config: &CameraConfig, aspect_ratio: f32) {
        let aspect_ratio = if aspect_ratio > 0.0 {
            aspect_ratio
        } else {
            CameraConfig::for_main_scene_default_camera().aspect_ratio
        };
        let v_fov = hfov_to_vfov(camera_config.h_fov, aspect_ratio);
        let tan_half = (v_fov.to_radians() * 0.5).tan();

        self.far_dist = camera_config.terrain_cull_range;
        self.far_width = 2.0 * tan_half * camera_config.terrain_cull_range * aspect_ratio;
        self.near_dist = 0.0;
        self.near_width = DEFAULT_ORBITAL_NEAR_WIDTH;
        self.fog_start_pct = DEFAULT_ORBITAL_FOG_START_PCT;
        self.fog_end_pct = DEFAULT_ORBITAL_FOG_END_PCT;
    }

    pub fn snapshot(&self) -> String {
        format!(
            "enabled={}|far_dist={:?}|far_width={:?}|near_dist={:?}|near_width={:?}|fog_on={}|fog_range_pct=({:.2}, {:.2})",
            self.enabled,
            self.far_dist,
            self.far_width,
            self.near_dist,
            self.near_width,
            self.fog_on,
            self.fog_start_pct,
            self.fog_end_pct,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DevEditorRenderToggles {
    pub render_terrain: bool,
    pub render_static_objects: bool,
    pub render_effects: bool,
    pub render_dropped_items: bool,
    pub render_item_labels: bool,
    pub render_equipped_items: bool,
    pub render_weather_effects: bool,
    pub render_ui: bool,
    pub render_hero: bool,
    pub render_npcs: bool,
    pub render_monsters: bool,
    pub render_shaders: bool,
    pub render_skill_effects: bool,
}

impl Default for DevEditorRenderToggles {
    fn default() -> Self {
        Self {
            render_terrain: true,
            render_static_objects: true,
            render_effects: true,
            render_dropped_items: true,
            render_item_labels: true,
            render_equipped_items: true,
            render_weather_effects: true,
            render_ui: true,
            render_hero: true,
            render_npcs: true,
            render_monsters: true,
            render_shaders: true,
            render_skill_effects: true,
        }
    }
}

impl DevEditorRenderToggles {
    pub fn enable_supported_render_toggles(&mut self) {
        self.render_terrain = true;
        self.render_static_objects = true;
        self.render_effects = true;
        self.render_dropped_items = true;
        self.render_item_labels = true;
        self.render_weather_effects = true;
    }

    pub fn disable_supported_render_toggles(&mut self) {
        self.render_terrain = false;
        self.render_static_objects = false;
        self.render_effects = false;
        self.render_dropped_items = false;
        self.render_item_labels = false;
        self.render_weather_effects = false;
    }

    pub fn snapshot(&self) -> String {
        format!(
            "terrain={}|static_objects={}|effects={}|dropped_items={}|item_labels={}|equipped_items={}|weather_effects={}|ui={}|hero={}|npcs={}|monsters={}|shaders={}|skill_effects={}",
            self.render_terrain,
            self.render_static_objects,
            self.render_effects,
            self.render_dropped_items,
            self.render_item_labels,
            self.render_equipped_items,
            self.render_weather_effects,
            self.render_ui,
            self.render_hero,
            self.render_npcs,
            self.render_monsters,
            self.render_shaders,
            self.render_skill_effects,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DevEditorDebugFlags {
    pub show_character_pick_boxes: bool,
    pub show_item_pick_boxes: bool,
    pub show_item_cull_sphere: bool,
    pub show_tile_grid: bool,
}

impl DevEditorDebugFlags {
    pub fn snapshot(&self) -> String {
        format!(
            "character_pick_boxes={}|item_pick_boxes={}|item_cull_sphere={}|tile_grid={}",
            self.show_character_pick_boxes,
            self.show_item_pick_boxes,
            self.show_item_cull_sphere,
            self.show_tile_grid,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DevEditorGraphicsState {
    pub current_resolution_width: u32,
    pub current_resolution_height: u32,
    pub opengl_viewport_width: u32,
    pub opengl_viewport_height: u32,
    pub screen_rate_x: f32,
    pub screen_rate_y: f32,
    pub window_mode: DevEditorWindowMode,
    pub actual_window_client_width: u32,
    pub actual_window_client_height: u32,
    pub calculated_scale_x: f32,
    pub calculated_scale_y: f32,
    pub window_size_mismatch: bool,
}

impl Default for DevEditorGraphicsState {
    fn default() -> Self {
        Self {
            current_resolution_width: 0,
            current_resolution_height: 0,
            opengl_viewport_width: 0,
            opengl_viewport_height: 0,
            screen_rate_x: 0.0,
            screen_rate_y: 0.0,
            window_mode: DevEditorWindowMode::Windowed,
            actual_window_client_width: 0,
            actual_window_client_height: 0,
            calculated_scale_x: 0.0,
            calculated_scale_y: 0.0,
            window_size_mismatch: false,
        }
    }
}

impl DevEditorGraphicsState {
    pub fn copy_debug_info(&self) -> String {
        let aspect_ratio = if self.current_resolution_height > 0 {
            self.current_resolution_width as f32 / self.current_resolution_height as f32
        } else {
            0.0
        };
        let expected_ui_scale_x = if REFERENCE_WIDTH > 0 {
            self.current_resolution_width as f32 / REFERENCE_WIDTH as f32
        } else {
            0.0
        };
        let expected_ui_scale_y = if REFERENCE_HEIGHT > 0 {
            self.current_resolution_height as f32 / REFERENCE_HEIGHT as f32
        } else {
            0.0
        };

        format!(
            "=== Graphics Debug Info ===\nCurrent Resolution: {} x {}\nOpenGL Viewport: {} x {}\nScreen Rate: {:.2} x {:.2}\nWindow Mode: {}\nActual Window Client: {} x {}\nCalculated Scale from Client: {:.2} x {:.2}\nMismatch: {}\nUI Reference System: {} x {}\nExpected UI Scale: WindowWidth/{} = {:.2}, WindowHeight/{} = {:.2}\nAspect Ratio: {:.3}\n",
            self.current_resolution_width,
            self.current_resolution_height,
            self.opengl_viewport_width,
            self.opengl_viewport_height,
            self.screen_rate_x,
            self.screen_rate_y,
            self.window_mode.label(),
            self.actual_window_client_width,
            self.actual_window_client_height,
            self.calculated_scale_x,
            self.calculated_scale_y,
            if self.window_size_mismatch { "YES" } else { "NO" },
            REFERENCE_WIDTH,
            REFERENCE_HEIGHT,
            REFERENCE_WIDTH,
            expected_ui_scale_x,
            REFERENCE_HEIGHT,
            expected_ui_scale_y,
            aspect_ratio,
        )
    }

    pub fn snapshot(&self) -> String {
        format!(
            "current_resolution={}x{}|opengl_viewport={}x{}|screen_rate={:.2}x{:.2}|window_mode={}|client={}x{}|calculated_scale={:.2}x{:.2}|mismatch={}",
            self.current_resolution_width,
            self.current_resolution_height,
            self.opengl_viewport_width,
            self.opengl_viewport_height,
            self.screen_rate_x,
            self.screen_rate_y,
            self.window_mode.as_str(),
            self.actual_window_client_width,
            self.actual_window_client_height,
            self.calculated_scale_x,
            self.calculated_scale_y,
            self.window_size_mismatch,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DevEditorState {
    pub default_override: DevEditorDefaultCameraOverride,
    pub orbital_override: DevEditorOrbitalCameraOverride,
    pub render_toggles: DevEditorRenderToggles,
    pub debug_flags: DevEditorDebugFlags,
    pub cull_radius_item: f32,
    pub login_terrain_distance: f32,
    pub login_object_distance: f32,
    pub graphics: DevEditorGraphicsState,
}

impl Default for DevEditorState {
    fn default() -> Self {
        Self {
            default_override: DevEditorDefaultCameraOverride::default(),
            orbital_override: DevEditorOrbitalCameraOverride::default(),
            render_toggles: DevEditorRenderToggles::default(),
            debug_flags: DevEditorDebugFlags::default(),
            cull_radius_item: DEFAULT_CULL_RADIUS_ITEM,
            login_terrain_distance: DEFAULT_LOGIN_TERRAIN_DISTANCE,
            login_object_distance: DEFAULT_LOGIN_OBJECT_DISTANCE,
            graphics: DevEditorGraphicsState::default(),
        }
    }
}

impl Resource for DevEditorState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct DevEditorPlugin;

impl Plugin for DevEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DevEditorState>();
    }
}

impl DevEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_default_camera_override(&mut self) {
        self.default_override.reset_to_main_scene_default_camera();
    }

    pub fn reset_orbital_camera_override_for_aspect_ratio(&mut self, aspect_ratio: f32) {
        let camera_config = CameraConfig::for_main_scene_orbital_camera();
        self.orbital_override
            .reset_to_natural_pyramid(&camera_config, aspect_ratio);
    }

    pub fn reset_login_scene_distances(&mut self) {
        self.login_terrain_distance = DEFAULT_LOGIN_TERRAIN_DISTANCE;
        self.login_object_distance = DEFAULT_LOGIN_OBJECT_DISTANCE;
    }

    pub fn enable_supported_render_toggles(&mut self) {
        self.render_toggles.enable_supported_render_toggles();
    }

    pub fn disable_supported_render_toggles(&mut self) {
        self.render_toggles.disable_supported_render_toggles();
    }

    pub fn snapshot(&self) -> String {
        format!(
            "default_override={}|orbital_override={}|render_toggles={}|debug_flags={}|cull_radius_item={:?}|login_distances=({:?}, {:?})|graphics={}",
            self.default_override.snapshot(),
            self.orbital_override.snapshot(),
            self.render_toggles.snapshot(),
            self.debug_flags.snapshot(),
            self.cull_radius_item,
            self.login_terrain_distance,
            self.login_object_distance,
            self.graphics.snapshot(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DevEditorDebugFlags, DevEditorDefaultCameraOverride, DevEditorGraphicsState,
        DevEditorOrbitalCameraOverride, DevEditorPlugin, DevEditorRenderToggles, DevEditorState,
        DevEditorWindowMode,
    };
    use bevy::prelude::App;
    use mu_render::{hfov_to_vfov, CameraConfig};

    #[test]
    fn dev_editor_state_matches_legacy_defaults() {
        let state = DevEditorState::default();

        assert_eq!(
            state.default_override,
            DevEditorDefaultCameraOverride::default()
        );
        assert_eq!(
            state.orbital_override,
            DevEditorOrbitalCameraOverride::default()
        );
        assert_eq!(state.render_toggles, DevEditorRenderToggles::default());
        assert_eq!(state.debug_flags, DevEditorDebugFlags::default());
        assert_eq!(state.cull_radius_item, 100.0);
        assert_eq!(state.login_terrain_distance, 3995.0);
        assert_eq!(state.login_object_distance, 5903.0);
        assert_eq!(state.graphics.window_mode, DevEditorWindowMode::Windowed);
        assert_eq!(
            state.default_override.snapshot(),
            "enabled=false|near_plane=500.0|far_plane=3000.0|offset=(0.0, 0.0, 0.0)|width_mul=(1.0, 1.0)|fog_override=false|fog_on=true|fog_range_pct=(1.00, 1.25)"
        );
    }

    #[test]
    fn reset_helpers_restore_factory_values() {
        let mut state = DevEditorState::default();
        state.default_override.enabled = true;
        state.default_override.near_plane = 1.0;
        state.default_override.far_plane = 2.0;
        state.default_override.offset_x = 3.0;
        state.default_override.offset_y = 4.0;
        state.default_override.offset_z = 5.0;
        state.default_override.width_near_mul = 6.0;
        state.default_override.width_far_mul = 7.0;
        state.default_override.fog_override = true;
        state.default_override.fog_on = false;
        state.default_override.fog_start_pct = 0.5;
        state.default_override.fog_end_pct = 0.6;

        state.orbital_override.enabled = true;
        state.orbital_override.far_dist = 1.0;
        state.orbital_override.far_width = 2.0;
        state.orbital_override.near_dist = 3.0;
        state.orbital_override.near_width = 4.0;
        state.orbital_override.fog_on = false;
        state.orbital_override.fog_start_pct = 0.5;
        state.orbital_override.fog_end_pct = 0.6;

        state.render_toggles.disable_supported_render_toggles();
        state.debug_flags = DevEditorDebugFlags {
            show_character_pick_boxes: true,
            show_item_pick_boxes: true,
            show_item_cull_sphere: true,
            show_tile_grid: true,
        };
        state.cull_radius_item = 42.0;
        state.login_terrain_distance = 1.0;
        state.login_object_distance = 2.0;

        state.reset_default_camera_override();
        state.reset_orbital_camera_override_for_aspect_ratio(4.0 / 3.0);
        state.reset_login_scene_distances();

        let orbital_preset = CameraConfig::for_main_scene_orbital_camera();
        let expected_far_width = {
            let v_fov = hfov_to_vfov(orbital_preset.h_fov, 4.0 / 3.0);
            let tan_half = (v_fov.to_radians() * 0.5).tan();
            2.0 * tan_half * orbital_preset.terrain_cull_range * (4.0 / 3.0)
        };

        assert_eq!(state.default_override.near_plane, 20.0);
        assert_eq!(state.default_override.far_plane, 3000.0);
        assert_eq!(state.default_override.offset_x, 0.0);
        assert_eq!(state.default_override.offset_y, 0.0);
        assert_eq!(state.default_override.offset_z, 0.0);
        assert_eq!(state.default_override.width_near_mul, 1.0);
        assert_eq!(state.default_override.width_far_mul, 1.0);
        assert!(state.default_override.fog_override);
        assert!(!state.default_override.fog_on);
        assert_eq!(state.default_override.fog_start_pct, 1.0);
        assert_eq!(state.default_override.fog_end_pct, 1.25);

        assert_eq!(
            state.orbital_override.far_dist,
            orbital_preset.terrain_cull_range
        );
        assert!((state.orbital_override.far_width - expected_far_width).abs() < 0.01);
        assert_eq!(state.orbital_override.near_dist, 0.0);
        assert_eq!(state.orbital_override.near_width, 800.0);
        assert!(!state.orbital_override.fog_on);
        assert_eq!(state.orbital_override.fog_start_pct, 1.0);
        assert_eq!(state.orbital_override.fog_end_pct, 1.25);

        assert!(state.render_toggles.render_hero);
        assert!(state.render_toggles.render_npcs);
        assert!(state.render_toggles.render_monsters);
        assert!(state.render_toggles.render_shaders);
        assert!(state.render_toggles.render_skill_effects);
        assert!(state.debug_flags.show_character_pick_boxes);
        assert!(state.debug_flags.show_item_pick_boxes);
        assert!(state.debug_flags.show_item_cull_sphere);
        assert!(state.debug_flags.show_tile_grid);
        assert_eq!(state.cull_radius_item, 42.0);
        assert_eq!(state.login_terrain_distance, 3995.0);
        assert_eq!(state.login_object_distance, 5903.0);
    }

    #[test]
    fn graphics_debug_info_is_stable() {
        let graphics = DevEditorGraphicsState {
            current_resolution_width: 1280,
            current_resolution_height: 720,
            opengl_viewport_width: 1280,
            opengl_viewport_height: 720,
            screen_rate_x: 2.0,
            screen_rate_y: 1.5,
            window_mode: DevEditorWindowMode::Fullscreen,
            actual_window_client_width: 1260,
            actual_window_client_height: 700,
            calculated_scale_x: 1.97,
            calculated_scale_y: 1.46,
            window_size_mismatch: true,
        };

        assert_eq!(
            graphics.copy_debug_info(),
            "=== Graphics Debug Info ===\nCurrent Resolution: 1280 x 720\nOpenGL Viewport: 1280 x 720\nScreen Rate: 2.00 x 1.50\nWindow Mode: Fullscreen\nActual Window Client: 1260 x 700\nCalculated Scale from Client: 1.97 x 1.46\nMismatch: YES\nUI Reference System: 640 x 480\nExpected UI Scale: WindowWidth/640 = 2.00, WindowHeight/480 = 1.50\nAspect Ratio: 1.778\n"
        );
        assert_eq!(
            graphics.snapshot(),
            "current_resolution=1280x720|opengl_viewport=1280x720|screen_rate=2.00x1.50|window_mode=fullscreen|client=1260x700|calculated_scale=1.97x1.46|mismatch=true"
        );
    }

    #[test]
    fn plugin_registers_the_resource() {
        let mut app = App::new();
        app.add_plugins(DevEditorPlugin);

        let state = app.world().resource::<DevEditorState>();

        assert_eq!(state.cull_radius_item, 100.0);
        assert_eq!(state.graphics.window_mode, DevEditorWindowMode::Windowed);
    }
}
