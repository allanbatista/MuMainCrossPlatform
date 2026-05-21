use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{PlayerShopManager, PlayerShopMode};
use mu_ui::{player_shop_screen, PlayerShopScreen, PlayerShopScreenState, UiRoute, UiShellState};

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
const EDITING_ACCENT: Color = Color::srgb(0.39, 0.72, 0.95);
const OPEN_ACCENT: Color = Color::srgb(0.48, 0.82, 0.58);
const PRICING_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PlayerShopShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerShopShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: PlayerShopManager,
}

#[derive(Debug, Default, Resource)]
struct PlayerShopShellState {
    root: Option<Entity>,
    key: Option<PlayerShopShellKey>,
}

#[derive(Debug, Clone)]
struct PlayerShopShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct PlayerShopShellRoot;

impl Plugin for PlayerShopShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShopShellState>()
            .add_systems(PostUpdate, sync_player_shop_shell_system);
    }
}

fn sync_player_shop_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    player_shop_manager: Res<PlayerShopManager>,
    mut state: ResMut<PlayerShopShellState>,
) {
    let current = player_shop_shell_key(
        ui_shell.current(),
        session_state.phase(),
        player_shop_manager.clone(),
    );

    let Some(key) = current else {
        clear_player_shop_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_player_shop_shell(&mut commands, &mut state);

    let Some(view) = player_shop_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_player_shop_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn player_shop_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: PlayerShopManager,
) -> Option<PlayerShopShellKey> {
    if !player_shop_shell_visible(route, phase) {
        return None;
    }

    Some(PlayerShopShellKey {
        route,
        phase,
        manager,
    })
}

fn player_shop_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::Marketplace && phase != SessionPhase::Disconnected
}

fn player_shop_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &PlayerShopManager,
) -> Option<PlayerShopShellView> {
    if !player_shop_shell_visible(route, phase) {
        return None;
    }

    let screen_state = player_shop_screen_state_for_mode(manager.mode());
    let screen = player_shop_screen(screen_state);

    Some(PlayerShopShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: player_shop_shell_body(&screen, phase, manager),
        accent: accent_for_state(screen.state),
    })
}

fn player_shop_screen_state_for_mode(mode: PlayerShopMode) -> PlayerShopScreenState {
    match mode {
        PlayerShopMode::Editing => PlayerShopScreenState::Editing,
        PlayerShopMode::Open => PlayerShopScreenState::Open,
        PlayerShopMode::Pricing => PlayerShopScreenState::Pricing,
        PlayerShopMode::Error => PlayerShopScreenState::Error,
    }
}

fn player_shop_shell_body(
    screen: &PlayerShopScreen,
    phase: SessionPhase,
    manager: &PlayerShopManager,
) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Player shop shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "Marketplace",
        &format!(
            "mode={} | screen_state={} | title={:?} | personal_shop_enabled={} | source_index={:?} | target_index={:?} | price={:?} | input_value_text_box_enabled={} | item_count={}",
            player_shop_mode_label(manager.mode()),
            screen.state.as_str(),
            manager.title(),
            manager.personal_shop_enabled(),
            manager.source_index(),
            manager.target_index(),
            manager.price(),
            manager.input_value_text_box_enabled(),
            manager.item_count(),
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

fn player_shop_mode_label(mode: PlayerShopMode) -> &'static str {
    match mode {
        PlayerShopMode::Editing => "editing",
        PlayerShopMode::Open => "open",
        PlayerShopMode::Pricing => "pricing",
        PlayerShopMode::Error => "error",
    }
}

fn accent_for_state(state: PlayerShopScreenState) -> Color {
    match state {
        PlayerShopScreenState::Editing => EDITING_ACCENT,
        PlayerShopScreenState::Open => OPEN_ACCENT,
        PlayerShopScreenState::Pricing => PRICING_ACCENT,
        PlayerShopScreenState::Error => ERROR_ACCENT,
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

fn spawn_player_shop_shell(commands: &mut Commands, view: &PlayerShopShellView) -> Entity {
    let root = commands
        .spawn((
            PlayerShopShellRoot,
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

fn clear_player_shop_shell(commands: &mut Commands, state: &mut PlayerShopShellState) {
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
    use super::{player_shop_shell_view, PlayerShopShellPlugin, PlayerShopShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{PlayerShopManager, PlayerShopPlugin};
    use mu_ui::{UiRoute, UiShellState};

    fn player_shop_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&PlayerShopShellRoot>();
        query.iter(world).count()
    }

    fn player_shop_shell_root_entity(
        world: &mut bevy::prelude::World,
    ) -> Option<bevy::prelude::Entity> {
        let mut query = world
            .query_filtered::<bevy::prelude::Entity, bevy::prelude::With<PlayerShopShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn player_shop_shell_view_renders_expected_bodies() {
        let manager = PlayerShopManager::new();
        let editing_view =
            player_shop_shell_view(UiRoute::Marketplace, SessionPhase::LoggedIn, &manager)
                .expect("player shop editing view missing");

        assert_eq!(editing_view.title, "Marketplace");
        assert!(editing_view
            .status
            .contains("route=marketplace | group=gameplay"));
        assert!(editing_view.body.contains("Player shop shell is active."));
        assert!(editing_view
            .body
            .contains("mode=editing | screen_state=editing"));
        assert!(editing_view.body.contains("title=\"\""));
        assert!(editing_view
            .body
            .contains("personal_shop_enabled=false | source_index=None"));
        assert!(editing_view.body.contains("Prepare the personal shop."));
        assert!(editing_view.body.contains("OpenShop"));
        assert_eq!(editing_view.accent, super::EDITING_ACCENT);

        let mut manager = PlayerShopManager::new();
        manager.set_title("Ares Market");
        manager.set_personal_shop_enabled(true);
        manager.set_item_selection(Some(4), Some(0));
        manager.set_price(250_000);
        manager.set_item_count(8);

        let pricing_view =
            player_shop_shell_view(UiRoute::Marketplace, SessionPhase::LoggedIn, &manager)
                .expect("player shop pricing view missing");

        assert!(pricing_view
            .body
            .contains("mode=pricing | screen_state=pricing"));
        assert!(pricing_view.body.contains("title=\"Ares Market\""));
        assert!(pricing_view.body.contains("source_index=Some(4)"));
        assert!(pricing_view.body.contains("target_index=Some(0)"));
        assert!(pricing_view.body.contains("price=Some(250000)"));
        assert!(pricing_view.body.contains("item_count=8"));
        assert!(pricing_view.body.contains("Set the item price."));
        assert_eq!(pricing_view.accent, super::PRICING_ACCENT);

        manager.mark_error();
        let error_view =
            player_shop_shell_view(UiRoute::Marketplace, SessionPhase::LoggedIn, &manager)
                .expect("player shop error view missing");

        assert!(error_view.body.contains("mode=error | screen_state=error"));
        assert!(error_view.body.contains("Player shop sync failed."));
        assert!(error_view.body.contains("Retry"));
        assert_eq!(error_view.accent, super::ERROR_ACCENT);

        assert!(
            player_shop_shell_view(UiRoute::Marketplace, SessionPhase::Disconnected, &manager)
                .is_none()
        );
        assert!(player_shop_shell_view(UiRoute::World, SessionPhase::LoggedIn, &manager).is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((
            mu_ui::UiShellPlugin,
            PlayerShopShellPlugin,
            PlayerShopPlugin,
        ));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::Marketplace);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(player_shop_shell_root_count(app.world_mut()), 1);
        let first_root = player_shop_shell_root_entity(app.world_mut()).unwrap();

        {
            let mut manager = app.world_mut().resource_mut::<PlayerShopManager>();
            manager.set_title("Ares Market");
            manager.set_personal_shop_enabled(true);
            manager.set_item_selection(Some(4), Some(0));
            manager.set_price(250_000);
            manager.set_item_count(8);
        }
        app.update();

        assert_eq!(player_shop_shell_root_count(app.world_mut()), 1);
        let second_root = player_shop_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(player_shop_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<SessionState>()
            .sync_phase(SessionPhase::LoggedIn);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Marketplace);
        app.world_mut().resource_mut::<PlayerShopManager>().reset();
        app.update();

        assert_eq!(player_shop_shell_root_count(app.world_mut()), 1);
        let third_root = player_shop_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(second_root, third_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(player_shop_shell_root_count(app.world_mut()), 0);
    }
}
