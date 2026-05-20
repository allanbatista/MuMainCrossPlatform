use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{GameShopManager, GameShopVersion};
use mu_ui::{game_shop_screen, GameShopScreen, GameShopScreenState, UiRoute, UiShellState};

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
const CATALOG_ACCENT: Color = Color::srgb(0.39, 0.72, 0.95);
const DETAILS_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);
const STORAGE_ACCENT: Color = Color::srgb(0.48, 0.82, 0.58);
const EMPTY_ACCENT: Color = Color::srgb(0.58, 0.62, 0.68);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct GameShopShellPlugin;

#[derive(Debug, Clone, PartialEq)]
struct GameShopShellKey {
    route: UiRoute,
    phase: SessionPhase,
    manager: GameShopManager,
}

#[derive(Debug, Default, Resource)]
struct GameShopShellState {
    root: Option<Entity>,
    key: Option<GameShopShellKey>,
}

#[derive(Debug, Clone)]
struct GameShopShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct GameShopShellRoot;

impl Plugin for GameShopShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameShopShellState>()
            .add_systems(PostUpdate, sync_game_shop_shell_system);
    }
}

fn sync_game_shop_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    game_shop_manager: Res<GameShopManager>,
    mut state: ResMut<GameShopShellState>,
) {
    let current = game_shop_shell_key(
        ui_shell.current(),
        session_state.phase(),
        game_shop_manager.clone(),
    );

    let Some(key) = current else {
        clear_game_shop_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_game_shop_shell(&mut commands, &mut state);

    let Some(view) = game_shop_shell_view(key.route, key.phase, &key.manager) else {
        return;
    };

    let root = spawn_game_shop_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn game_shop_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    manager: GameShopManager,
) -> Option<GameShopShellKey> {
    if !game_shop_shell_visible(route, phase) {
        return None;
    }

    Some(GameShopShellKey {
        route,
        phase,
        manager,
    })
}

fn game_shop_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    route == UiRoute::GameShop && phase != SessionPhase::Disconnected
}

fn game_shop_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    manager: &GameShopManager,
) -> Option<GameShopShellView> {
    if !game_shop_shell_visible(route, phase) {
        return None;
    }

    let screen = game_shop_screen(manager);

    Some(GameShopShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body: game_shop_shell_body(&screen, phase),
        accent: accent_for_state(screen.state),
    })
}

fn game_shop_shell_body(screen: &GameShopScreen, phase: SessionPhase) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "GameShop shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());
    push_paragraph(
        &mut body,
        "GameShop",
        &format!(
            "mode={} | screen_state={}",
            screen.state.as_str(),
            screen.state.as_str(),
        ),
    );
    push_paragraph(
        &mut body,
        "Execution",
        &format!(
            "shop_open={} | request_opening={} | script_version={} | current_script_version={} | banner_version={} | current_banner_version={}",
            screen.execution.shop_open,
            screen.execution.request_opening,
            format_game_shop_version(screen.execution.script_version),
            format_game_shop_version(screen.execution.current_script_version),
            format_game_shop_version(screen.execution.banner_version),
            format_game_shop_version(screen.execution.current_banner_version),
        ),
    );
    push_paragraph(
        &mut body,
        "Wallet",
        &format!(
            "total_cash={} | total_point={} | cash_credit_card={} | cash_prepaid={} | total_mileage={}",
            screen.wallet.total_cash,
            screen.wallet.total_point,
            screen.wallet.cash_credit_card,
            screen.wallet.cash_prepaid,
            screen.wallet.total_mileage,
        ),
    );

    if let Some(catalog) = screen.catalog.as_ref() {
        push_paragraph(
            &mut body,
            "Catalog",
            &format!(
                "zone_name={} | category_name={} | selected_page={} | total_pages={} | package_count={} | display_package_count={} | selected_package_name={:?} | selected_package_price={:?} | selected_package_price_unit={:?} | selected_package_quantity={:?}",
                catalog.zone_name,
                catalog.category_name,
                catalog.selected_page,
                catalog.total_pages,
                catalog.package_count,
                catalog.display_package_count,
                catalog.selected_package_name,
                catalog.selected_package_price,
                catalog.selected_package_price_unit,
                catalog.selected_package_quantity,
            ),
        );
    }

    if let Some(storage) = screen.storage.as_ref() {
        push_paragraph(
            &mut body,
            "Storage",
            &format!(
                "inventory_type={:?} | current_page={} | total_pages={} | current_item_count={} | total_item_count={}",
                storage.inventory_type,
                storage.current_page,
                storage.total_pages,
                storage.current_item_count,
                storage.total_item_count,
            ),
        );
    }

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    if let Some(detail) = screen.status_detail.as_deref() {
        push_paragraph(&mut body, "Detail", detail);
    }

    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn format_game_shop_version(version: GameShopVersion) -> String {
    format!(
        "zone={} | year={} | year_id={}",
        version.zone, version.year, version.year_id,
    )
}

fn accent_for_state(state: GameShopScreenState) -> Color {
    match state {
        GameShopScreenState::Empty => EMPTY_ACCENT,
        GameShopScreenState::Catalog => CATALOG_ACCENT,
        GameShopScreenState::Details => DETAILS_ACCENT,
        GameShopScreenState::Storage => STORAGE_ACCENT,
        GameShopScreenState::Error => ERROR_ACCENT,
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

fn spawn_game_shop_shell(commands: &mut Commands, view: &GameShopShellView) -> Entity {
    let root = commands
        .spawn((
            GameShopShellRoot,
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

fn clear_game_shop_shell(commands: &mut Commands, state: &mut GameShopShellState) {
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
    use super::{game_shop_shell_view, GameShopShellPlugin, GameShopShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{GameShopManager, GameShopPlugin, GameShopVersion};
    use mu_ui::{UiRoute, UiShellState};

    fn game_shop_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&GameShopShellRoot>();
        query.iter(world).count()
    }

    fn game_shop_shell_root_entity(
        world: &mut bevy::prelude::World,
    ) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<GameShopShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn game_shop_shell_view_renders_expected_bodies() {
        let empty_manager = GameShopManager::new();
        let empty_view =
            game_shop_shell_view(UiRoute::GameShop, SessionPhase::LoggedIn, &empty_manager)
                .expect("game shop empty view missing");

        assert_eq!(empty_view.title, "GameShop");
        assert!(empty_view
            .status
            .contains("route=game-shop | group=gameplay"));
        assert!(empty_view.body.contains("GameShop shell is active."));
        assert!(empty_view.body.contains("mode=empty | screen_state=empty"));
        assert!(empty_view.body.contains("No GameShop items are available."));
        assert_eq!(empty_view.accent, super::EMPTY_ACCENT);

        let mut catalog_manager = GameShopManager::new();
        catalog_manager.set_versions(
            GameShopVersion {
                zone: 1,
                year: 2024,
                year_id: 5,
            },
            GameShopVersion {
                zone: 1,
                year: 2024,
                year_id: 4,
            },
            GameShopVersion {
                zone: 2,
                year: 2024,
                year_id: 8,
            },
            GameShopVersion {
                zone: 2,
                year: 2024,
                year_id: 7,
            },
        );
        catalog_manager.set_wallet(1_200.0, 540.0, 150.0, 25.0, 90.0);
        catalog_manager.show_catalog("Lorencia", "Featured", 1, 4, 12, 8);

        let catalog_view =
            game_shop_shell_view(UiRoute::GameShop, SessionPhase::LoggedIn, &catalog_manager)
                .expect("game shop catalog view missing");

        assert!(catalog_view
            .body
            .contains("mode=catalog | screen_state=catalog"));
        assert!(catalog_view.body.contains("Execution"));
        assert!(catalog_view.body.contains("zone=1 | year=2024 | year_id=5"));
        assert!(catalog_view.body.contains(
            "zone_name=Lorencia | category_name=Featured | selected_page=1 | total_pages=4 | package_count=12 | display_package_count=8"
        ));
        assert_eq!(catalog_view.accent, super::CATALOG_ACCENT);

        catalog_manager.show_details("Wing Box", 1_500, "Cash", 1);
        let details_view =
            game_shop_shell_view(UiRoute::GameShop, SessionPhase::LoggedIn, &catalog_manager)
                .expect("game shop details view missing");

        assert!(details_view
            .body
            .contains("mode=details | screen_state=details"));
        assert!(details_view
            .body
            .contains("selected_package_name=Some(\"Wing Box\")"));
        assert_eq!(details_view.accent, super::DETAILS_ACCENT);

        catalog_manager.show_storage(Some(1), 2, 5, 4, 22);
        let storage_view =
            game_shop_shell_view(UiRoute::GameShop, SessionPhase::LoggedIn, &catalog_manager)
                .expect("game shop storage view missing");

        assert!(storage_view
            .body
            .contains("mode=storage | screen_state=storage"));
        assert!(storage_view.body.contains(
            "inventory_type=Some(1) | current_page=2 | total_pages=5 | current_item_count=4 | total_item_count=22"
        ));
        assert_eq!(storage_view.accent, super::STORAGE_ACCENT);

        let mut error_manager = GameShopManager::new();
        error_manager.show_error("GameShop catalog sync failed.");
        let error_view =
            game_shop_shell_view(UiRoute::GameShop, SessionPhase::LoggedIn, &error_manager)
                .expect("game shop error view missing");

        assert!(error_view.body.contains("mode=error | screen_state=error"));
        assert!(error_view.body.contains("GameShop catalog sync failed."));
        assert!(error_view.body.contains("Detail"));
        assert_eq!(error_view.accent, super::ERROR_ACCENT);

        assert!(game_shop_shell_view(
            UiRoute::GameShop,
            SessionPhase::Disconnected,
            &error_manager
        )
        .is_none());
        assert!(
            game_shop_shell_view(UiRoute::World, SessionPhase::LoggedIn, &error_manager).is_none()
        );
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, GameShopShellPlugin, GameShopPlugin));

        let mut ui_shell = mu_ui::UiShellState::default();
        ui_shell.set_route(UiRoute::GameShop);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(game_shop_shell_root_count(app.world_mut()), 1);
        let first_root = game_shop_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<GameShopManager>()
            .show_catalog("Lorencia", "Featured", 1, 4, 12, 8);
        app.update();

        assert_eq!(game_shop_shell_root_count(app.world_mut()), 1);
        let second_root = game_shop_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(game_shop_shell_root_count(app.world_mut()), 0);

        app.world_mut()
            .resource_mut::<SessionState>()
            .sync_phase(SessionPhase::LoggedIn);
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::GameShop);
        app.world_mut().resource_mut::<GameShopManager>().reset();
        app.update();

        assert_eq!(game_shop_shell_root_count(app.world_mut()), 1);
        let third_root = game_shop_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(second_root, third_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(game_shop_shell_root_count(app.world_mut()), 0);
    }
}
