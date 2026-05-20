use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{EventKind, EventManager, EventMode};
use mu_ui::{events_screen, EventScreen, EventScreenState, UiRoute, UiShellState};

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
const ENTRY_ACCENT: Color = Color::srgb(0.38, 0.66, 0.96);
const COUNTDOWN_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const REWARD_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const RESULT_ACCENT: Color = Color::srgb(0.76, 0.70, 0.96);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct EventsShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EventsShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: EventManager,
}

#[derive(Debug, Default, Resource)]
struct EventsShellState {
    root: Option<Entity>,
    key: Option<EventsShellKey>,
}

#[derive(Debug, Clone)]
struct EventsShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct EventsShellRoot;

impl Plugin for EventsShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventsShellState>()
            .add_systems(PostUpdate, sync_events_shell_system);
    }
}

fn sync_events_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    event_manager: Res<EventManager>,
    mut state: ResMut<EventsShellState>,
) {
    let current = events_shell_key(
        ui_shell.current(),
        session_state.phase(),
        event_manager.clone(),
    );

    let Some(key) = current else {
        clear_events_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_events_shell(&mut commands, &mut state);

    let Some(view) = events_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_events_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn events_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: EventManager,
) -> Option<EventsShellKey> {
    if events_shell_visible(route, phase) {
        Some(EventsShellKey {
            route,
            phase,
            manager,
        })
    } else {
        None
    }
}

fn events_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Events && phase != SessionPhase::Disconnected
}

fn events_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &EventManager,
) -> Option<EventsShellView> {
    if !events_shell_visible(route, phase) {
        return None;
    }

    let state = events_shell_state_for_phase_and_manager(phase, manager);
    let screen = events_screen(state);

    Some(EventsShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: events_shell_body(&screen, phase, manager),
        accent: accent_for_state(screen.state),
    })
}

fn events_shell_state_for_phase_and_manager(
    phase: SessionPhase,
    manager: &EventManager,
) -> EventScreenState {
    if phase == SessionPhase::ReadyForLogin {
        return EventScreenState::Error;
    }

    if manager.kind() == EventKind::None && !matches!(manager.mode(), EventMode::Idle) {
        return EventScreenState::Error;
    }

    match manager.mode() {
        EventMode::Idle | EventMode::Entry => EventScreenState::Entry,
        EventMode::Countdown => EventScreenState::Countdown,
        EventMode::Reward => EventScreenState::Reward,
        EventMode::Result => EventScreenState::Result,
        EventMode::Error => EventScreenState::Error,
    }
}

fn events_shell_body(screen: &EventScreen, phase: SessionPhase, manager: &EventManager) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Events shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Event",
        &format!(
            "mode={} | screen_state={} | kind={} | event_name={:?} | event_level={:?} | ticket_slot={:?}",
            event_mode_label(manager.mode()),
            screen.state.as_str(),
            event_kind_label(manager.kind()),
            event_name(manager),
            manager.event_level(),
            manager.ticket_slot(),
        ),
    );
    push_paragraph(
        &mut body,
        "Progress",
        &format!(
            "countdown_seconds={:?} | reward_text={:?} | result_text={:?}",
            manager.countdown_seconds(),
            manager.reward_text(),
            manager.result_text(),
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

fn event_name(manager: &EventManager) -> Option<&str> {
    let name = manager.event_name();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn event_mode_label(mode: EventMode) -> &'static str {
    match mode {
        EventMode::Idle => "idle",
        EventMode::Entry => "entry",
        EventMode::Countdown => "countdown",
        EventMode::Reward => "reward",
        EventMode::Result => "result",
        EventMode::Error => "error",
    }
}

fn event_kind_label(kind: EventKind) -> &'static str {
    match kind {
        EventKind::None => "none",
        EventKind::BloodCastle => "blood-castle",
        EventKind::ChaosCastle => "chaos-castle",
        EventKind::Doppelganger => "doppelganger",
        EventKind::CursedTemple => "cursed-temple",
        EventKind::DevilSquare => "devil-square",
        EventKind::LuckyCoin => "lucky-coin",
        EventKind::EmpireGuardian => "empire-guardian",
        EventKind::IllusionTemple => "illusion-temple",
        EventKind::Kanturu => "kanturu",
        EventKind::CryWolf => "crywolf",
        EventKind::BattleSoccer => "battle-soccer",
    }
}

fn accent_for_state(state: EventScreenState) -> Color {
    match state {
        EventScreenState::Entry => ENTRY_ACCENT,
        EventScreenState::Countdown => COUNTDOWN_ACCENT,
        EventScreenState::Reward => REWARD_ACCENT,
        EventScreenState::Result => RESULT_ACCENT,
        EventScreenState::Error => ERROR_ACCENT,
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

fn spawn_events_shell(commands: &mut Commands, view: &EventsShellView) -> Entity {
    let root = commands
        .spawn((
            EventsShellRoot,
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
        ))
        .id()
}

fn clear_events_shell(commands: &mut Commands, state: &mut EventsShellState) {
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
}

fn push_lines<I, T>(output: &mut String, heading: &str, values: I, empty_label: Option<&str>)
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut values = values.into_iter().map(Into::into).peekable();
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(heading);
    output.push('\n');

    if values.peek().is_none() {
        output.push_str(empty_label.unwrap_or("<none>"));
        return;
    }

    for value in values {
        output.push_str("- ");
        output.push_str(&value);
        output.push('\n');
    }
    output.pop();
}

#[cfg(test)]
mod tests {
    use super::{
        events_shell_state_for_phase_and_manager, events_shell_view, EventsShellPlugin,
        EventsShellRoot,
    };
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{EventKind, EventManager, EventPlugin};
    use mu_ui::{EventScreenState, UiRoute, UiShellState};

    fn events_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&EventsShellRoot>();
        query.iter(world).count()
    }

    fn events_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<EventsShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn events_shell_visibility_follows_route_and_disconnect() {
        assert!(super::events_shell_visible(
            UiRoute::Events,
            SessionPhase::LoggedIn
        ));
        assert!(!super::events_shell_visible(
            UiRoute::Events,
            SessionPhase::Disconnected
        ));
        assert!(!super::events_shell_visible(
            UiRoute::Friend,
            SessionPhase::LoggedIn
        ));
    }

    #[test]
    fn events_shell_state_follows_manager_state() {
        let mut manager = EventManager::new();
        manager.set_event(EventKind::DevilSquare, "Devil Square");
        manager.set_entry(Some(4), Some(1));

        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            EventScreenState::Entry
        );

        let view = events_shell_view(UiRoute::Events, SessionPhase::LoggedIn, &manager)
            .expect("events shell view");
        assert!(view.body.contains("mode=entry | screen_state=entry"));
        assert!(view.body.contains("kind=devil-square"));
        assert!(view.body.contains("event_name=Some(\"Devil Square\")"));
        assert!(view.body.contains("event_level=Some(4)"));
        assert!(view.body.contains("ticket_slot=Some(1)"));

        manager.start_countdown(45);
        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            EventScreenState::Countdown
        );
        let view = events_shell_view(UiRoute::Events, SessionPhase::LoggedIn, &manager)
            .expect("events shell view");
        assert!(view
            .body
            .contains("mode=countdown | screen_state=countdown"));
        assert!(view.body.contains("countdown_seconds=Some(45)"));

        manager.set_reward("Lucky Coin Bundle");
        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            EventScreenState::Reward
        );
        let view = events_shell_view(UiRoute::Events, SessionPhase::LoggedIn, &manager)
            .expect("events shell view");
        assert!(view.body.contains("mode=reward | screen_state=reward"));
        assert!(view
            .body
            .contains("reward_text=Some(\"Lucky Coin Bundle\")"));

        manager.set_result("Second place secured.");
        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            EventScreenState::Result
        );
        let view = events_shell_view(UiRoute::Events, SessionPhase::LoggedIn, &manager)
            .expect("events shell view");
        assert!(view.body.contains("mode=result | screen_state=result"));
        assert!(view
            .body
            .contains("result_text=Some(\"Second place secured.\")"));

        manager.mark_error();
        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::LoggedIn, &manager),
            EventScreenState::Error
        );
        let view = events_shell_view(UiRoute::Events, SessionPhase::LoggedIn, &manager)
            .expect("events shell view");
        assert!(view.body.contains("mode=error | screen_state=error"));
    }

    #[test]
    fn events_shell_view_returns_error_for_login_edge() {
        let manager = EventManager::new();
        let view = events_shell_view(UiRoute::Events, SessionPhase::ReadyForLogin, &manager)
            .expect("events shell view");
        assert_eq!(
            events_shell_state_for_phase_and_manager(SessionPhase::ReadyForLogin, &manager),
            EventScreenState::Error
        );
        assert!(view.body.contains("state=error"));
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, EventsShellPlugin, EventPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Events);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        {
            let mut manager = app.world_mut().resource_mut::<EventManager>();
            manager.set_event(EventKind::ChaosCastle, "Chaos Castle");
            manager.set_entry(Some(7), Some(3));
        }

        app.update();

        assert_eq!(events_shell_root_count(app.world_mut()), 1);
        let first_root = events_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<EventManager>()
            .start_countdown(45);
        app.update();

        assert_eq!(events_shell_root_count(app.world_mut()), 1);
        let second_root = events_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(events_shell_root_count(app.world_mut()), 0);
    }
}
