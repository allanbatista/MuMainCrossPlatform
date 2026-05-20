use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{TradeManager, TradeMode};
use mu_ui::{trade_screen, TradeScreen, TradeScreenState, UiRoute, UiShellState};

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
const REQUESTED_ACCENT: Color = Color::srgb(0.34, 0.69, 0.84);
const ACTIVE_ACCENT: Color = Color::srgb(0.42, 0.82, 0.58);
const CONFIRMING_ACCENT: Color = Color::srgb(0.89, 0.73, 0.34);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct TradeShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TradeShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: TradeManager,
}

#[derive(Debug, Default, Resource)]
struct TradeShellState {
    root: Option<Entity>,
    key: Option<TradeShellKey>,
}

#[derive(Debug, Clone)]
struct TradeShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct TradeShellRoot;

impl Plugin for TradeShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TradeShellState>()
            .add_systems(PostUpdate, sync_trade_shell_system);
    }
}

fn sync_trade_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    trade_manager: Res<TradeManager>,
    mut state: ResMut<TradeShellState>,
) {
    let current = trade_shell_key(
        ui_shell.current(),
        session_state.phase(),
        trade_manager.clone(),
    );

    let Some(key) = current else {
        clear_trade_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_trade_shell(&mut commands, &mut state);

    let Some(view) = trade_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_trade_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn trade_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: TradeManager,
) -> Option<TradeShellKey> {
    if !trade_shell_visible(route, phase) {
        return None;
    }

    Some(TradeShellKey {
        route,
        phase,
        manager,
    })
}

fn trade_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Trade && phase != SessionPhase::Disconnected
}

fn trade_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &TradeManager,
) -> Option<TradeShellView> {
    if !trade_shell_visible(route, phase) {
        return None;
    }

    let screen_state = trade_shell_state_for_mode(manager.mode());
    let screen = trade_screen(screen_state);

    Some(TradeShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: trade_shell_body(&screen, phase, manager.mode()),
        accent: accent_for_state(screen.state),
    })
}

fn trade_shell_state_for_mode(mode: TradeMode) -> TradeScreenState {
    match mode {
        TradeMode::Idle | TradeMode::Requested => TradeScreenState::Requested,
        TradeMode::Active => TradeScreenState::Active,
        TradeMode::Confirming => TradeScreenState::Confirming,
        TradeMode::Error => TradeScreenState::Error,
    }
}

fn trade_shell_body(screen: &TradeScreen, phase: SessionPhase, mode: TradeMode) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Trade shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Trade",
        &format!(
            "mode={} | screen_state={}",
            trade_mode_label(mode),
            screen.state.as_str(),
        ),
    );
    push_paragraph(
        &mut body,
        "Partner",
        &format!(
            "partner_id={:?} | partner_level={:?} | partner_guild_type={:?}",
            screen.partner_id, screen.partner_level, screen.partner_guild_type,
        ),
    );
    push_paragraph(
        &mut body,
        "Gold",
        &format!(
            "partner_trade_gold={:?} | my_trade_gold={:?} | partner_confirmed={} | my_confirmed={} | trade_alert={} | my_trade_wait={:?}",
            screen.partner_trade_gold,
            screen.my_trade_gold,
            screen.partner_confirmed,
            screen.my_confirmed,
            screen.trade_alert,
            screen.my_trade_wait,
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

fn trade_mode_label(mode: TradeMode) -> &'static str {
    match mode {
        TradeMode::Idle => "idle",
        TradeMode::Requested => "requested",
        TradeMode::Active => "active",
        TradeMode::Confirming => "confirming",
        TradeMode::Error => "error",
    }
}

fn accent_for_state(state: TradeScreenState) -> Color {
    match state {
        TradeScreenState::Requested => REQUESTED_ACCENT,
        TradeScreenState::Active => ACTIVE_ACCENT,
        TradeScreenState::Confirming => CONFIRMING_ACCENT,
        TradeScreenState::Error => ERROR_ACCENT,
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

fn spawn_trade_shell(commands: &mut Commands, view: &TradeShellView) -> Entity {
    let root = commands
        .spawn((
            TradeShellRoot,
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

fn clear_trade_shell(commands: &mut Commands, state: &mut TradeShellState) {
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
    use super::{trade_shell_view, TradeShellPlugin, TradeShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{TradeManager, TradePlugin};
    use mu_ui::{UiRoute, UiShellState};

    fn trade_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&TradeShellRoot>();
        query.iter(world).count()
    }

    fn trade_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<TradeShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn trade_shell_view_renders_expected_bodies() {
        let manager = TradeManager::new();
        let requested_view = trade_shell_view(UiRoute::Trade, SessionPhase::LoggedIn, &manager)
            .expect("trade view missing");

        assert_eq!(requested_view.title, "Trade");
        assert!(requested_view
            .status
            .contains("route=trade | group=gameplay"));
        assert!(requested_view.body.contains("Trade shell is active."));
        assert!(requested_view
            .body
            .contains("mode=idle | screen_state=requested"));
        assert!(requested_view.body.contains("Trade request received."));
        assert!(requested_view.body.contains("Accept"));
        assert!(requested_view.body.contains("Decline"));

        let mut manager = TradeManager::new();
        manager.request_trade("VeryLongPartnerName", 320, 4);
        manager.accept_trade();
        manager.set_my_trade_gold(150_000);
        let active_view = trade_shell_view(UiRoute::Trade, SessionPhase::LoggedIn, &manager)
            .expect("trade active view missing");

        assert!(active_view
            .body
            .contains("mode=active | screen_state=active"));
        assert!(active_view.body.contains("Exchange items and zen."));
        assert!(active_view.body.contains("MoveItem"));
        assert!(active_view.body.contains("Confirm"));

        manager.toggle_my_confirmed();
        manager.set_partner_confirmed(true);
        let confirming_view = trade_shell_view(UiRoute::Trade, SessionPhase::LoggedIn, &manager)
            .expect("trade confirming view missing");

        assert!(confirming_view
            .body
            .contains("mode=confirming | screen_state=confirming"));
        assert!(confirming_view
            .body
            .contains("Confirm the trade before closing."));

        manager.mark_error();
        let error_view = trade_shell_view(UiRoute::Trade, SessionPhase::LoggedIn, &manager)
            .expect("trade error view missing");

        assert!(error_view.body.contains("mode=error | screen_state=error"));
        assert!(error_view.body.contains("Trade sync failed."));

        assert!(trade_shell_view(UiRoute::Trade, SessionPhase::Disconnected, &manager).is_none());
        assert!(trade_shell_view(UiRoute::World, SessionPhase::LoggedIn, &manager).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, TradeShellPlugin, TradePlugin));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::Trade);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(trade_shell_root_count(app.world_mut()), 1);
        let first_root = trade_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<TradeManager>()
            .request_trade("Blade", 320, 4);
        app.update();

        assert_eq!(trade_shell_root_count(app.world_mut()), 1);
        let second_root = trade_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(trade_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<SessionState>()
            .sync_phase(SessionPhase::LoggedIn);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Trade);
        app.world_mut().resource_mut::<TradeManager>().reset();
        app.update();

        assert_eq!(trade_shell_root_count(app.world_mut()), 1);
        let third_root = trade_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(second_root, third_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(trade_shell_root_count(app.world_mut()), 0);
    }
}
