use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_ui::{gate_screen, GateScreen, GateScreenState, UiRoute, UiShellState};

use crate::SessionPhase;

const SCREEN_PADDING: f32 = 28.0;
const CARD_MAX_WIDTH: f32 = 960.0;
const CARD_PADDING: f32 = 28.0;
const CARD_GAP: f32 = 18.0;
const CONTENT_GAP: f32 = 14.0;
const ACCENT_BAR_HEIGHT: f32 = 4.0;
const TITLE_FONT_SIZE: f32 = 42.0;
const STATUS_FONT_SIZE: f32 = 16.0;
const BODY_FONT_SIZE: f32 = 21.0;
const BACKDROP_COLOR: Color = Color::srgb(0.03, 0.04, 0.06);
const CARD_COLOR: Color = Color::srgb(0.08, 0.10, 0.14);
const PANEL_COLOR: Color = Color::srgb(0.11, 0.13, 0.18);
const TEXT_COLOR: Color = Color::srgb(0.95, 0.96, 0.98);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.72, 0.78, 0.84);
const GATE_ACCENT: Color = Color::srgb(0.45, 0.69, 0.96);
const MASTER_ACCENT: Color = Color::srgb(0.89, 0.73, 0.34);
const PRIVATE_ACCENT: Color = Color::srgb(0.53, 0.57, 0.65);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GateShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateShellKey {
    route: UiRoute,
    phase: SessionPhase,
}

#[derive(Debug, Default, Resource)]
struct GateShellState {
    root: Option<Entity>,
    key: Option<GateShellKey>,
}

#[derive(Debug, Clone)]
struct GateShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct GateShellRoot;

impl Plugin for GateShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GateShellState>()
            .add_systems(PostUpdate, sync_gate_shell_system);
    }
}

fn sync_gate_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    mut state: ResMut<GateShellState>,
) {
    let current = gate_shell_key(ui_shell.current(), session_state.phase());

    let Some(key) = current else {
        clear_gate_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_gate_shell(&mut commands, &mut state);

    let Some(view) = gate_shell_view(key.route, key.phase) else {
        return;
    };

    let root = spawn_gate_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn gate_shell_key(route: UiRoute, phase: SessionPhase) -> Option<GateShellKey> {
    if !gate_shell_visible(route, phase) {
        return None;
    }

    Some(GateShellKey { route, phase })
}

fn gate_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Gate && phase != SessionPhase::Disconnected
}

fn gate_shell_state_for_phase(phase: SessionPhase) -> GateScreenState {
    match phase {
        SessionPhase::LoggedIn => GateScreenState::GuildMaster,
        SessionPhase::ReadyForLogin => GateScreenState::GuestPublic,
        SessionPhase::Disconnected => GateScreenState::GuestPrivate,
    }
}

fn gate_shell_view(route: UiRoute, phase: SessionPhase) -> Option<GateShellView> {
    if !gate_shell_visible(route, phase) {
        return None;
    }

    let screen = gate_screen(gate_shell_state_for_phase(phase));

    Some(GateShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: gate_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn gate_shell_body(screen: &GateScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Gate shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Gate",
        &format!(
            "state={} | public={} | entrance_fee={} | view_entrance_fee={} | add_entrance_fee={} | max_entrance_fee={} | player_gold={:?} | enter_enabled={}",
            screen.state.as_str(),
            screen.public,
            screen.entrance_fee,
            screen.view_entrance_fee,
            screen.add_entrance_fee,
            screen.max_entrance_fee,
            screen.player_gold,
            screen.enter_enabled,
        ),
    );

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn accent_for_state(state: GateScreenState) -> Color {
    match state {
        GateScreenState::GuestPublic => GATE_ACCENT,
        GateScreenState::GuestPrivate => PRIVATE_ACCENT,
        GateScreenState::GuildMember => GATE_ACCENT,
        GateScreenState::GuildMaster => MASTER_ACCENT,
    }
}

fn status_line(route: UiRoute, phase: SessionPhase) -> String {
    format!(
        "route={} | group={} | session={}",
        route.slug(),
        route.group().as_str(),
        phase.as_str()
    )
}

fn spawn_gate_shell(commands: &mut Commands, view: &GateShellView) -> Entity {
    let root = commands
        .spawn((
            GateShellRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(SCREEN_PADDING)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BACKDROP_COLOR),
        ))
        .id();

    let card = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(CARD_MAX_WIDTH),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(CARD_PADDING)),
                row_gap: Val::Px(CARD_GAP),
                ..Default::default()
            },
            BackgroundColor(CARD_COLOR),
        ))
        .id();
    commands.entity(root).add_child(card);

    let accent = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(ACCENT_BAR_HEIGHT),
                ..Default::default()
            },
            BackgroundColor(view.accent),
        ))
        .id();
    commands.entity(card).add_child(accent);

    let title = spawn_text_block(commands, view.title, TITLE_FONT_SIZE, view.accent);
    commands.entity(card).add_child(title);

    let status = spawn_text_block(commands, &view.status, STATUS_FONT_SIZE, MUTED_TEXT_COLOR);
    commands.entity(card).add_child(status);

    let panel = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(CARD_PADDING * 0.75)),
                row_gap: Val::Px(CONTENT_GAP),
                ..Default::default()
            },
            BackgroundColor(PANEL_COLOR),
        ))
        .id();
    commands.entity(card).add_child(panel);

    let body = spawn_text_block(commands, &view.body, BODY_FONT_SIZE, TEXT_COLOR);
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

fn clear_gate_shell(commands: &mut Commands, state: &mut GateShellState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }

    state.key = None;
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

#[cfg(test)]
mod tests {
    use super::{gate_shell_view, GateShellPlugin, GateShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_ui::{UiRoute, UiShellState};

    fn gate_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&GateShellRoot>();
        query.iter(world).count()
    }

    fn gate_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<GateShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn gate_shell_view_renders_expected_bodies() {
        let logged_in_view = gate_shell_view(UiRoute::Gate, SessionPhase::LoggedIn).unwrap();

        assert_eq!(logged_in_view.title, "Gate");
        assert!(logged_in_view
            .status
            .contains("route=gate | group=gameplay"));
        assert!(logged_in_view.body.contains("Gate shell is active."));
        assert!(logged_in_view.body.contains("state=guild-master"));
        assert!(logged_in_view.body.contains("public=false"));
        assert!(logged_in_view.body.contains("entrance_fee=25000"));
        assert!(logged_in_view
            .body
            .contains("Configure gate access and fees."));
        assert!(logged_in_view.body.contains("TogglePublic"));
        assert!(logged_in_view.body.contains("Enter"));

        let guest_view = gate_shell_view(UiRoute::Gate, SessionPhase::ReadyForLogin).unwrap();

        assert!(guest_view.body.contains("state=guest-public"));
        assert!(guest_view
            .body
            .contains("Public gate access requires the entrance fee."));
        assert!(guest_view.body.contains("public=true"));
        assert!(guest_view.body.contains("player_gold=Some(12000)"));

        assert!(gate_shell_view(UiRoute::Gate, SessionPhase::Disconnected).is_none());
        assert!(gate_shell_view(UiRoute::World, SessionPhase::LoggedIn).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, GateShellPlugin));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::Gate);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(gate_shell_root_count(app.world_mut()), 1);
        let first_root = gate_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(gate_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<SessionState>()
            .sync_phase(SessionPhase::ReadyForLogin);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Gate);
        app.update();

        assert_eq!(gate_shell_root_count(app.world_mut()), 1);
        let second_root = gate_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(gate_shell_root_count(app.world_mut()), 0);
    }
}
