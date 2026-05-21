use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection, Name,
    Node, Plugin, PositionType, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_audio::AudioRuntime;
use mu_render::SkillParticleQueue;
use mu_ui::{
    chat_screen, hotkeys_screen, hud_screen, minimap_screen, ChatMessageCount, ChatScreen,
    HotkeyBinding, HotkeysScreen, HudButton, HudGauge, HudScreen, MiniMapMarker, MiniMapScreen,
    UiRoute, UiShellState,
};

use crate::ClientRuntime;

const HUD_ROOT_LEFT: f32 = 24.0;
const HUD_ROOT_BOTTOM: f32 = 24.0;
const HUD_CARD_WIDTH: f32 = 640.0;
const HUD_CARD_PADDING: f32 = 18.0;
const HUD_CARD_GAP: f32 = 10.0;
const HUD_CONTENT_GAP: f32 = 8.0;
const HUD_ACCENT_BAR_HEIGHT: f32 = 4.0;
const HUD_TITLE_FONT_SIZE: f32 = 24.0;
const HUD_STATUS_FONT_SIZE: f32 = 14.0;
const HUD_BODY_FONT_SIZE: f32 = 16.0;
const HUD_CARD_COLOR: Color = Color::srgb(0.08, 0.11, 0.15);
const HUD_PANEL_COLOR: Color = Color::srgb(0.11, 0.14, 0.18);
const HUD_TEXT_COLOR: Color = Color::srgb(0.95, 0.97, 0.99);
const HUD_MUTED_TEXT_COLOR: Color = Color::srgb(0.71, 0.79, 0.86);
const HUD_ACCENT: Color = Color::srgb(0.25, 0.82, 0.86);
const CHAT_ACCENT: Color = Color::srgb(0.92, 0.61, 0.24);
const MINIMAP_ACCENT: Color = Color::srgb(0.42, 0.78, 0.42);
const HOTKEYS_ACCENT: Color = Color::srgb(0.86, 0.70, 0.26);
const SKILL_FEEDBACK_ACCENT: Color = Color::srgb(0.84, 0.42, 0.25);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct WorldHudPlugin;

#[derive(Debug, Default, Resource)]
struct WorldHudState {
    root: Option<Entity>,
    key: Option<WorldHudKey>,
}

#[derive(Debug, Clone)]
struct WorldHudSurfaceView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Debug, Clone, PartialEq)]
struct WorldHudKey {
    hud: HudScreen,
    chat: ChatScreen,
    minimap: MiniMapScreen,
    hotkeys: HotkeysScreen,
    skill_feedback: Option<SkillFeedbackKey>,
}

#[derive(Debug, Clone, PartialEq)]
struct SkillFeedbackKey {
    particle_count: usize,
    audio_events: usize,
    particle_snapshot: String,
    audio_diagnostics: String,
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
    skill_particles: Option<Res<SkillParticleQueue>>,
    audio_runtime: Option<Res<AudioRuntime>>,
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

    let key = WorldHudKey {
        hud: hud_screen(),
        chat: chat_screen(),
        minimap: minimap_screen(),
        hotkeys: hotkeys_screen(),
        skill_feedback: skill_feedback_key(skill_particles.as_deref(), audio_runtime.as_deref()),
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_world_hud(&mut commands, &mut state);

    let views = world_hud_views(&key);
    let root = spawn_world_hud(&mut commands, &views);
    state.root = Some(root);
    state.key = Some(key);
}

fn world_hud_views(key: &WorldHudKey) -> Vec<WorldHudSurfaceView> {
    let mut views = vec![
        hud_view(key.hud),
        chat_view(key.chat),
        minimap_view(key.minimap),
        hotkeys_view(key.hotkeys),
    ];

    if let Some(skill_feedback) = key.skill_feedback.as_ref() {
        views.push(skill_feedback_view(skill_feedback));
    }

    views
}

fn hud_view(screen: HudScreen) -> WorldHudSurfaceView {
    WorldHudSurfaceView {
        title: screen.title,
        status: format!(
            "route={} | group={} | layer_depth={} | key_event_order={}",
            screen.route.slug(),
            screen.layout.group.as_str(),
            screen.layer_depth,
            screen.key_event_order
        ),
        body: hud_body(&screen),
        accent: HUD_ACCENT,
    }
}

fn chat_view(screen: ChatScreen) -> WorldHudSurfaceView {
    WorldHudSurfaceView {
        title: screen.title,
        status: format!(
            "route={} | group={} | state={} | layer_depth={} | key_event_order={}",
            screen.route.slug(),
            screen.layout.group.as_str(),
            screen.state.as_str(),
            screen.layer_depth,
            screen.key_event_order
        ),
        body: chat_body(&screen),
        accent: CHAT_ACCENT,
    }
}

fn minimap_view(screen: MiniMapScreen) -> WorldHudSurfaceView {
    WorldHudSurfaceView {
        title: screen.title,
        status: format!(
            "route={} | group={} | loaded={} | layer_depth={}",
            screen.route.slug(),
            screen.layout.group.as_str(),
            screen.loaded,
            screen.layer_depth
        ),
        body: minimap_body(&screen),
        accent: MINIMAP_ACCENT,
    }
}

fn hotkeys_view(screen: HotkeysScreen) -> WorldHudSurfaceView {
    WorldHudSurfaceView {
        title: screen.title,
        status: format!(
            "route={} | group={} | state={} | layer_depth={}",
            screen.route.slug(),
            screen.layout.group.as_str(),
            screen.state.as_str(),
            screen.layer_depth
        ),
        body: hotkeys_body(&screen),
        accent: HOTKEYS_ACCENT,
    }
}

fn skill_feedback_view(key: &SkillFeedbackKey) -> WorldHudSurfaceView {
    WorldHudSurfaceView {
        title: "Skill Feedback",
        status: format!(
            "particle_count={} | audio_events={}",
            key.particle_count, key.audio_events
        ),
        body: skill_feedback_body(key),
        accent: SKILL_FEEDBACK_ACCENT,
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

fn chat_body(screen: &ChatScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "The chat log surface is active.");
    push_paragraph(&mut body, "Mode", screen.state.as_str());
    push_paragraph(
        &mut body,
        "Visibility",
        &format!(
            "show_chat_log={} | show_frame={} | current_message_type={}",
            screen.show_chat_log,
            screen.show_frame,
            screen.current_message_type.as_str(),
        ),
    );
    push_paragraph(
        &mut body,
        "Window",
        &format!(
            "showing_lines={} | render_end_line={} | back_alpha={} | pointed_message_index={:?}",
            screen.showing_lines,
            screen.render_end_line,
            screen.back_alpha,
            screen.pointed_message_index,
        ),
    );
    push_lines(
        &mut body,
        "Filters",
        screen.filters.iter().copied(),
        Some("none"),
    );
    push_lines(
        &mut body,
        "Message counts",
        screen.message_counts.iter().map(chat_message_count_label),
        None,
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

fn minimap_body(screen: &MiniMapScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "The minimap surface is active.");
    push_paragraph(&mut body, "Loaded", &screen.loaded.to_string());
    push_paragraph(
        &mut body,
        "Map",
        &format!(
            "{} | scale_index={} | hero_position=({}, {})",
            screen.map_name, screen.scale_index, screen.hero_position.0, screen.hero_position.1
        ),
    );
    push_lines(
        &mut body,
        "Map sizes",
        screen.map_sizes.iter().map(|size| size.to_string()),
        None,
    );
    push_lines(
        &mut body,
        "Markers",
        screen.markers.iter().map(minimap_marker_label),
        None,
    );
    push_paragraph(
        &mut body,
        "Exit button",
        &screen.exit_button_enabled.to_string(),
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

fn hotkeys_body(screen: &HotkeysScreen) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "The hotkey bar surface is active.");
    push_paragraph(&mut body, "Mode", screen.state.as_str());
    push_paragraph(&mut body, "Current skill", screen.current_skill);
    push_paragraph(
        &mut body,
        "Row page",
        &format!(
            "{} | prior_skill={} | pet_commands_visible={}",
            screen.skill_row_page,
            screen.prior_skill.unwrap_or("none"),
            screen.pet_commands_visible,
        ),
    );
    push_lines(
        &mut body,
        "Item hotkeys",
        screen.item_hotkeys.iter().map(hotkey_binding_label),
        None,
    );
    push_lines(
        &mut body,
        "Skill hotkeys",
        screen.skill_hotkeys.iter().map(hotkey_binding_label),
        None,
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

fn skill_feedback_body(key: &SkillFeedbackKey) -> String {
    let mut body = String::new();

    push_paragraph(
        &mut body,
        "Status",
        "The targeted skill feedback surface is active.",
    );
    push_paragraph(&mut body, "Particle queue", &key.particle_snapshot);
    push_paragraph(&mut body, "Audio diagnostics", &key.audio_diagnostics);

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

fn chat_message_count_label(count: &ChatMessageCount) -> String {
    format!("{} | {}", count.message_type.as_str(), count.count)
}

fn minimap_marker_label(marker: &MiniMapMarker) -> String {
    format!(
        "{} | {} | ({}, {}) | rot={}",
        marker.kind.as_str(),
        marker.label,
        marker.location.0,
        marker.location.1,
        marker.rotation,
    )
}

fn hotkey_binding_label(binding: &HotkeyBinding) -> String {
    let state = if binding.active { "active" } else { "inactive" };

    match binding.count {
        Some(count) => format!(
            "{} | {} x{} ({state})",
            binding.slot, binding.binding, count
        ),
        None => format!("{} | {} ({state})", binding.slot, binding.binding),
    }
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

fn spawn_world_hud(commands: &mut Commands, views: &[WorldHudSurfaceView]) -> Entity {
    let root = commands
        .spawn((
            WorldHudRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(HUD_ROOT_LEFT),
                bottom: Val::Px(HUD_ROOT_BOTTOM),
                width: Val::Px(HUD_CARD_WIDTH),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Start,
                row_gap: Val::Px(HUD_CARD_GAP),
                ..Default::default()
            },
            Name::new("world-hud-root"),
        ))
        .id();

    for view in views {
        let card = spawn_world_hud_surface(commands, view);
        commands.entity(root).add_child(card);
    }

    root
}

fn spawn_world_hud_surface(commands: &mut Commands, view: &WorldHudSurfaceView) -> Entity {
    let slug = hud_surface_slug(view.title);
    let card_name = format!("world-hud-{slug}-card");
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
            Name::new(card_name),
        ))
        .id();

    let accent_name = format!("world-hud-{slug}-accent");
    let accent = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(HUD_ACCENT_BAR_HEIGHT),
                ..Default::default()
            },
            BackgroundColor(view.accent),
            Name::new(accent_name),
        ))
        .id();
    commands.entity(card).add_child(accent);

    let title = spawn_text_block(commands, view.title, HUD_TITLE_FONT_SIZE, view.accent);
    commands.entity(card).add_child(title);

    let status = spawn_text_block(
        commands,
        &view.status,
        HUD_STATUS_FONT_SIZE,
        HUD_MUTED_TEXT_COLOR,
    );
    commands.entity(card).add_child(status);

    let panel_name = format!("world-hud-{slug}-panel");
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
            Name::new(panel_name),
        ))
        .id();
    commands.entity(card).add_child(panel);

    let body = spawn_text_block(commands, &view.body, HUD_BODY_FONT_SIZE, HUD_TEXT_COLOR);
    commands.entity(panel).add_child(body);

    card
}

fn hud_surface_slug(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}

fn skill_feedback_key(
    skill_particles: Option<&SkillParticleQueue>,
    audio_runtime: Option<&AudioRuntime>,
) -> Option<SkillFeedbackKey> {
    let particle_count = skill_particles.map_or(0, SkillParticleQueue::len);
    let audio_events = audio_runtime.map_or(0, AudioRuntime::pending_audio_events);

    if particle_count == 0 && audio_events == 0 {
        return None;
    }

    Some(SkillFeedbackKey {
        particle_count,
        audio_events,
        particle_snapshot: skill_particles
            .map(SkillParticleQueue::snapshot)
            .unwrap_or_else(|| "Particle queue unavailable.".to_string()),
        audio_diagnostics: audio_runtime
            .map(AudioRuntime::diagnostics)
            .map(|diagnostics| diagnostics.to_string())
            .unwrap_or_else(|| "Audio runtime unavailable.".to_string()),
    })
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

    state.key = None;
}

#[cfg(test)]
mod tests {
    use super::{
        chat_body, chat_message_count_label, hotkey_binding_label, hotkeys_body, hud_body,
        hud_button_label, hud_gauge_label, minimap_body, minimap_marker_label, WorldHudPlugin,
        WorldHudRoot, WorldHudState,
    };
    use crate::ClientRuntime;
    use bevy::prelude::App;
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
    use mu_ui::{
        chat_screen, hotkeys_screen, hud_screen, minimap_screen, ChatMessageCount, ChatMessageType,
        HotkeyBinding, HudButton, HudButtonKind, HudGauge, HudGaugeKind, MiniMapMarker,
        MiniMapMarkerKind, UiRoute, UiShellState,
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
        assert!(state.key.is_none());
    }

    #[test]
    fn world_hud_spawns_visible_overlay_when_world_route_is_ready() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let state = app.world().resource::<WorldHudState>();
        assert!(state.root.is_some());
        assert!(state.key.is_some());

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
        assert!(state.key.is_none());
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
        assert!(state.key.is_none());
    }

    #[test]
    fn world_hud_body_lists_the_world_surface_stack_state() {
        let hud = hud_body(&hud_screen());
        let chat = chat_body(&chat_screen());
        let minimap = minimap_body(&minimap_screen());
        let hotkeys = hotkeys_body(&hotkeys_screen());

        assert!(hud.contains("The world HUD main frame is active."));
        assert!(hud.contains("life"));
        assert!(hud.contains("inventory"));
        assert!(hud.contains("Widgets"));
        assert!(chat.contains("The chat log surface is active."));
        assert!(chat.contains("Message counts"));
        assert!(minimap.contains("The minimap surface is active."));
        assert!(minimap.contains("Potion Merchant"));
        assert!(hotkeys.contains("The hotkey bar surface is active."));
        assert!(hotkeys.contains("Q | Greater Healing Potion"));
    }

    #[test]
    fn world_hud_includes_skill_feedback_card_when_queue_is_present() {
        let key = super::WorldHudKey {
            hud: hud_screen(),
            chat: chat_screen(),
            minimap: minimap_screen(),
            hotkeys: hotkeys_screen(),
            skill_feedback: Some(super::SkillFeedbackKey {
                particle_count: 1,
                audio_events: 1,
                particle_snapshot:
                    "state=ready|count=1|events=[skill=6|cue=teleport-burst|target=77]".to_string(),
                audio_diagnostics: "state=ready|queued_audio_events=1".to_string(),
            }),
        };

        let views = super::world_hud_views(&key);

        assert_eq!(views.len(), 5);
        let skill_feedback = views.last().expect("skill feedback card missing");
        assert_eq!(skill_feedback.title, "Skill Feedback");
        assert!(skill_feedback.status.contains("particle_count=1"));
        assert!(skill_feedback.body.contains("teleport-burst"));
        assert!(skill_feedback.body.contains("queued_audio_events=1"));
    }

    #[test]
    fn format_helpers_keep_labels_readable() {
        let gauge = HudGauge::new(HudGaugeKind::Experience, 2481200, 3120000);
        let button = HudButton::new(HudButtonKind::Inventory, true);
        let count = ChatMessageCount::new(ChatMessageType::Party, 0);
        let marker = MiniMapMarker::new(MiniMapMarkerKind::Portal, "Lorencia Gate", (180, 96), 45);
        let hotkey = HotkeyBinding::new("Q", "Greater Healing Potion", Some(12), true);

        assert_eq!(hud_gauge_label(&gauge), "experience | 2481200 / 3120000");
        assert_eq!(hud_button_label(&button), "inventory (pressed)");
        assert_eq!(chat_message_count_label(&count), "party | 0");
        assert_eq!(
            minimap_marker_label(&marker),
            "portal | Lorencia Gate | (180, 96) | rot=45"
        );
        assert_eq!(
            hotkey_binding_label(&hotkey),
            "Q | Greater Healing Potion x12 (active)"
        );
    }
}
