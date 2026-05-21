use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_editor_admin::{DevEditorState, EditorConsoleState, EditorCoreAction, EditorCoreState};

const ADMIN_CORE_TITLE: &str = "Admin Core";

pub use mu_editor_admin::EditorCoreState as AdminCoreScreenState;

#[derive(Debug, Clone, PartialEq)]
pub struct AdminCoreScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: AdminCoreScreenState,
    pub title: &'static str,
    pub console: EditorConsoleState,
    pub dev_editor: DevEditorState,
    pub actions: &'static [EditorCoreAction],
}

pub fn admin_core_screen(
    state: &EditorCoreState,
    console: &EditorConsoleState,
    dev_editor: &DevEditorState,
) -> AdminCoreScreen {
    AdminCoreScreen {
        route: UiRoute::AdminCore,
        layout: UiShellLayout::for_route(UiRoute::AdminCore),
        widgets: UiShellWidgetSet::for_route(UiRoute::AdminCore),
        state: state.clone(),
        title: ADMIN_CORE_TITLE,
        console: console.clone(),
        dev_editor: dev_editor.clone(),
        actions: state.toolbar_actions(),
    }
}

impl AdminCoreScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|title={}|shell={}|console={}|dev_editor={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.title,
            self.state.snapshot(),
            self.console.snapshot(),
            self.dev_editor.snapshot(),
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::admin_core_screen;
    use mu_editor_admin::{
        DevEditorState, EditorConsoleState, EditorCoreAction, EditorCoreState, EditorShellState,
    };

    #[test]
    fn admin_core_closed_snapshot_includes_shell_console_and_dev_editor() {
        let core = EditorCoreState::default();
        let console = EditorConsoleState::default();
        let dev_editor = DevEditorState::default();
        let screen = admin_core_screen(&core, &console, &dev_editor);

        assert_eq!(screen.route, crate::UiRoute::AdminCore);
        assert_eq!(screen.state.shell_state, EditorShellState::Closed);
        assert_eq!(screen.actions, &[EditorCoreAction::OpenEditor]);
        assert!(screen.console.editor_entries.is_empty());
        assert_eq!(screen.dev_editor.cull_radius_item, 100.0);
        assert_eq!(
            screen.snapshot(),
            "route=admin-core|group=admin|title=Admin Core|shell=state=closed|editor_enabled=false|hovering_ui=false|previous_frame_hovering_ui=false|show_item_editor=false|show_skill_editor=false|show_dev_editor=false|show_console=true|language_locale=en|language_name=English|available_locales=[\"en\"]|actions=[OpenEditor]|console=editor_entries=[]|game_entries=[]|log_file_path=None|dev_editor=default_override=enabled=false|near_plane=500.0|far_plane=3000.0|offset=(0.0, 0.0, 0.0)|width_mul=(1.0, 1.0)|fog_override=false|fog_on=true|fog_range_pct=(1.00, 1.25)|orbital_override=enabled=false|far_dist=5000.0|far_width=8000.0|near_dist=0.0|near_width=800.0|fog_on=true|fog_range_pct=(1.00, 1.25)|render_toggles=terrain=true|static_objects=true|effects=true|dropped_items=true|item_labels=true|equipped_items=true|weather_effects=true|ui=true|hero=true|npcs=true|monsters=true|shaders=true|skill_effects=true|debug_flags=character_pick_boxes=false|item_pick_boxes=false|item_cull_sphere=false|tile_grid=false|cull_radius_item=100.0|login_distances=(3995.0, 5903.0)|graphics=current_resolution=0x0|opengl_viewport=0x0|screen_rate=0.00x0.00|window_mode=windowed|client=0x0|calculated_scale=0.00x0.00|mismatch=false|actions=[OpenEditor]|layout=UiShellLayout { group: Admin, outer_margin: 24.0, panel_gap: 18.0, sidebar_width: 420.0, content_max_width: 1280.0, footer_height: 32.0 }|widgets=[Header, Body, Sidebar, Footer]"
        );
    }

    #[test]
    fn admin_core_open_snapshot_tracks_shell_actions() {
        let mut core = EditorCoreState::default();
        core.open();
        core.toggle_console();
        core.toggle_dev_editor();
        core.set_language_options("pt", "Portuguese", ["en", "pt"]);

        let mut console = EditorConsoleState::default();
        console.log_editor("editor ready");
        let mut dev_editor = DevEditorState::default();
        dev_editor.reset_default_camera_override();

        let screen = admin_core_screen(&core, &console, &dev_editor);

        assert_eq!(
            screen.actions,
            &[
                EditorCoreAction::ToggleItemEditor,
                EditorCoreAction::ToggleSkillEditor,
                EditorCoreAction::ToggleDevEditor,
                EditorCoreAction::ToggleConsole,
                EditorCoreAction::CloseEditor,
            ]
        );
        assert!(screen.state.is_enabled());
        assert!(!screen.state.is_showing_console());
        assert!(screen.state.is_showing_dev_editor());
        assert!(screen
            .console
            .editor_entries
            .contains(&"editor ready".to_string()));
        assert_eq!(screen.dev_editor.default_override.near_plane, 20.0);
        assert!(screen
            .snapshot()
            .contains("route=admin-core|group=admin|title=Admin Core"));
    }
}
