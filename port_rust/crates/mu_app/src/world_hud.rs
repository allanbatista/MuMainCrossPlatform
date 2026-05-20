use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection, Name,
    Node, Plugin, PositionType, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{hud_screen, HudButton, HudGauge, HudScreen, UiRoute, UiShellState};

use crate::ClientRuntime;

const HUD_ROOT_LEFT: f32 = 24.0;
const HUD_ROOT_BOTTOM: f32 = 24.0;
const HUD_CARD_WIDTH: f32 = 520.0;
const HUD_CARD_PADDING: f32 = 20.0;
const HUD_CARD_GAP: f32 = 12.0;
const HUD_CONTENT_GAP: f32 = 10.0;
const HUD_ACCENT_BAR_HEIGHT: f32 = 4.0;
const HUD_TITLE_FONT_SIZE: f32 = 28.0;
const HUD_STATUS_FONT_SIZE: f32 = 16.0;
const HUD_BODY_FONT_SIZE: f32 = 18.0;
const HUD_CARD_COLOR: Color = Color::srgb(0.08, 0.11, 0.15);
const HUD_PANEL_COLOR: Color = Color::srgb(0.11, 0.14, 0.18);
const HUD_TEXT_COLOR: Color = Color::srgb(0.95, 0.97, 0.99);
const HUD_MUTED_TEXT_COLOR: Color = Color::srgb(0.71, 0.79, 0.86);
const HUD_ACCENT: Color = Color::srgb(0.25, 0.82, 0.86);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct WorldHudPlugin;

#[derive(Debug, Default, Resource)]
struct WorldHudState {
    root: Option<Entity>,
    screen: Option<HudScreen>,
}

#[derive(Debug, Clone)]
struct WorldHudView {
    title: &'static str,
    status: String,
    body: String,
}

#[derive(Component)]
struct WorldHudRoot;

impl Plugin for WorldHudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldHudState>()
            .add_systems(PostUpdate, sync_world_hud_system);
    }
}

fn sync_world_hud_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    client_runtime: Res<ClientRuntime>,
    mut state: ResMut<WorldHudState>,
) {
    if ui_shell.current() != UiRoute::World
        || !client_runtime.world_ready()
        || !client_runtime.terrain_ready()
        || !client_runtime.render_entities_ready()
    {
        clear_world_hud(&mut commands, &mut state);
        return;
    }

    let screen = hud_screen();
    if state.screen == Some(screen) && state.root.is_some() {
        return;
    }

    clear_world_hud(&mut commands, &mut state);

    let view = hud_view(screen);
    let root = spawn_world_hud(&mut commands, &view);
    state.root = Some(root);
    state.screen = Some(screen);
}

fn hud_view(screen: HudScreen) -> WorldHudView {
    WorldHudView {
        title: screen.title,
        status: format!(
            "route={} | group={} | layer_depth={} | key_event_order={}",
            screen.route.slug(),
            screen.layout.group.as_str(),
            screen.layer_depth,
            screen.key_event_order
        ),
        body: hud_body(&screen),
    }
}

fn hud_body(screen: &HudScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "The world HUD main frame is active.");
    push_paragraph(&mut body, "Master Level", &screen.master_level.to_string());
    push_paragraph(
        &mut body,
        "Experience Effect",
        &screen.exp_effect_active.to_string(),
    );
    push_paragraph(
        &mut body,
        "Layout",
        &format!(
            "outer_margin={} | panel_gap={} | sidebar_width={} | content_max_width={} | footer_height={}",
            screen.layout.outer_margin,
            screen.layout.panel_gap,
            screen.layout.sidebar_width,
            screen.layout.content_max_width,
            screen.layout.footer_height,
        ),
    );
    push_lines(
        &mut body,
        "Gauges",
        screen.gauges.iter().map(hud_gauge_label),
        None,
    );
    push_lines(
        &mut body,
        "Buttons",
        screen.buttons.iter().map(hud_button_label),
        None,
    );
    push_lines(
        &mut body,
        "Widgets",
        screen
            .widgets
            .widgets()
            .iter()
            .map(|widget| format!("{widget:?}")),
        None,
    );

    body
}

fn hud_gauge_label(gauge: &HudGauge) -> String {
    format!(
        "{} | {} / {}",
        gauge.kind.as_str(),
        gauge.current,
        gauge.maximum
    )
}

fn hud_button_label(button: &HudButton) -> String {
    let state = if button.pressed { "pressed" } else { "idle" };
    format!("{} ({state})", button.kind.as_str())
}

fn push_paragraph(output: &mut String, heading: &str, text: &str) {
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');
    output.push_str(text);
    output.push('\n');
}

fn push_lines<I, S>(output: &mut String, heading: &str, items: I, empty_message: Option<&str>)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');

    let mut wrote_any = false;
    for item in items {
        wrote_any = true;
        output.push_str("- ");
        output.push_str(item.as_ref());
        output.push('\n');
    }

    if !wrote_any {
        if let Some(empty_message) = empty_message {
            output.push_str("- ");
            output.push_str(empty_message);
            output.push('\n');
        }
    }
}

fn spawn_world_hud(commands: &mut Commands, view: &WorldHudView) -> Entity {
    let root = commands
        .spawn((
            WorldHudRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_ROOT_LEFT),
                bottom: Val::Px(HUD_ROOT_BOTTOM),
                width: Val::Px(HUD_CARD_WIDTH),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..Default::default()
            },
            Name::new("world-hud-root"),
        ))
        .id();

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(HUD_CARD_PADDING)),
                row_gap: Val::Px(HUD_CARD_GAP),
                ..Default::default()
            },
            BackgroundColor(HUD_CARD_COLOR),
            Name::new("world-hud-card"),
        ))
        .id();
    commands.entity(root).add_child(card);

    let accent = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(HUD_ACCENT_BAR_HEIGHT),
                ..Default::default()
            },
            BackgroundColor(HUD_ACCENT),
            Name::new("world-hud-accent"),
        ))
        .id();
    commands.entity(card).add_child(accent);

    let title = spawn_text_block(commands, view.title, HUD_TITLE_FONT_SIZE, HUD_ACCENT);
    commands.entity(card).add_child(title);

    let status = spawn_text_block(
        commands,
        &view.status,
        HUD_STATUS_FONT_SIZE,
        HUD_MUTED_TEXT_COLOR,
    );
    commands.entity(card).add_child(status);

    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(HUD_CARD_PADDING * 0.75)),
                row_gap: Val::Px(HUD_CONTENT_GAP),
                ..Default::default()
            },
            BackgroundColor(HUD_PANEL_COLOR),
            Name::new("world-hud-panel"),
        ))
        .id();
    commands.entity(card).add_child(panel);

    let body = spawn_text_block(commands, &view.body, HUD_BODY_FONT_SIZE, HUD_TEXT_COLOR);
    commands.entity(panel).add_child(body);

    root
}

fn spawn_text_block(
    commands: &mut Commands,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextFont {
                font_size,
                ..Default::default()
            },
            TextColor(color),
            Node {
                width: Val::Percent(100.0),
                ..Default::default()
            },
        ))
        .id()
}

fn clear_world_hud(commands: &mut Commands, state: &mut WorldHudState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }

    state.screen = None;
}

#[cfg(test)]
mod tests {
    use super::{
        hud_body, hud_button_label, hud_gauge_label, WorldHudPlugin, WorldHudRoot, WorldHudState,
    };
    use crate::ClientRuntime;
    use bevy::prelude::App;
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
    use mu_ui::{
        hud_screen, HudButton, HudButtonKind, HudGauge, HudGaugeKind, UiRoute, UiShellState,
    };

    fn repo_assets_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn spawn_ready_app() -> App {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, WorldHudPlugin));
        app.insert_resource(ClientRuntime::new());
        app
    }

    fn load_world(app: &mut App) {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        app.world_mut()
            .resource_mut::<ClientRuntime>()
            .load_world_bundle(bundle);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();
    }

    #[test]
    fn world_hud_plugin_registers_state() {
        let mut app = App::new();
        app.add_plugins(WorldHudPlugin);

        let state = app.world().resource::<WorldHudState>();
        assert!(state.root.is_none());
        assert!(state.screen.is_none());
    }

    #[test]
    fn world_hud_spawns_visible_overlay_when_world_route_is_ready() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let state = app.world().resource::<WorldHudState>();
        assert!(state.root.is_some());
        assert!(state.screen.is_some());

        let root = state.root.expect("world HUD root missing");
        assert!(app.world().entity(root).contains::<WorldHudRoot>());
    }

    #[test]
    fn world_hud_clears_when_route_leaves_world() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        {
            let mut ui_shell = app.world_mut().resource_mut::<UiShellState>();
            ui_shell.set_route(UiRoute::Login);
        }

        app.update();

        let state = app.world().resource::<WorldHudState>();
        assert!(state.root.is_none());
        assert!(state.screen.is_none());
    }

    #[test]
    fn world_hud_waits_for_world_projection() {
        let mut app = spawn_ready_app();
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        let state = app.world().resource::<WorldHudState>();
        assert!(state.root.is_none());
        assert!(state.screen.is_none());
    }

    #[test]
    fn hud_body_lists_the_legacy_main_frame_state() {
        let screen = hud_screen();
        let body = hud_body(&screen);

        assert!(body.contains("The world HUD main frame is active."));
        assert!(body.contains("life"));
        assert!(body.contains("inventory"));
        assert!(body.contains("Widgets"));
        assert!(body.contains("Overlay"));
    }

    #[test]
    fn format_helpers_keep_labels_readable() {
        let gauge = HudGauge::new(HudGaugeKind::Experience, 2481200, 3120000);
        let button = HudButton::new(HudButtonKind::Inventory, true);

        assert_eq!(hud_gauge_label(&gauge), "experience | 2481200 / 3120000");
        assert_eq!(hud_button_label(&button), "inventory (pressed)");
    }
}
