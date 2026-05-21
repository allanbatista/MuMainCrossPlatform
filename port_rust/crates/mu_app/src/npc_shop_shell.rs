use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{NpcDialogueState, NpcManager, NpcShopMode, NpcShopState};
use mu_ui::{
    npc_screen, shop_screen, NpcScreen, NpcScreenState, ShopScreen, ShopScreenState, UiRoute,
    UiShellState,
};

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
const NPC_ACCENT: Color = Color::srgb(0.35, 0.78, 0.52);
const SHOP_ACCENT: Color = Color::srgb(0.90, 0.66, 0.26);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct NpcShopShellPlugin;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NpcShopShellKey {
    route: UiRoute,
    phase: SessionPhase,
    dialogue: NpcDialogueState,
    shop: NpcShopState,
}

#[derive(Debug, Default, Resource)]
struct NpcShopShellState {
    root: Option<Entity>,
    key: Option<NpcShopShellKey>,
}

#[derive(Debug, Clone)]
struct NpcShopShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct NpcShopShellRoot;

impl Plugin for NpcShopShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NpcShopShellState>()
            .add_systems(PostUpdate, sync_npc_shop_shell_system);
    }
}

fn sync_npc_shop_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    npc_manager: Res<NpcManager>,
    mut state: ResMut<NpcShopShellState>,
) {
    let current = npc_shop_shell_key(
        ui_shell.current(),
        session_state.phase(),
        npc_manager.dialogue().clone(),
        npc_manager.shop().clone(),
    );

    let Some(key) = current else {
        clear_npc_shop_shell(&mut commands, &mut state);
        return;
    };

    if state.key.as_ref() == Some(&key) && state.root.is_some() {
        return;
    }

    clear_npc_shop_shell(&mut commands, &mut state);

    let Some(view) = npc_shop_shell_view(key.route, key.phase, &key.dialogue, &key.shop) else {
        return;
    };

    let root = spawn_npc_shop_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn npc_shop_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    dialogue: NpcDialogueState,
    shop: NpcShopState,
) -> Option<NpcShopShellKey> {
    if npc_shop_shell_visible(route, phase) {
        Some(NpcShopShellKey {
            route,
            phase,
            dialogue,
            shop,
        })
    } else {
        None
    }
}

fn npc_shop_shell_visible(route: UiRoute, phase: SessionPhase) -> bool {
    matches!(route, UiRoute::Npc | UiRoute::Shop) && phase != SessionPhase::Disconnected
}

fn npc_shop_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    dialogue: &NpcDialogueState,
    shop: &NpcShopState,
) -> Option<NpcShopShellView> {
    if !npc_shop_shell_visible(route, phase) {
        return None;
    }

    match route {
        UiRoute::Npc => {
            let screen = npc_screen(npc_screen_state(dialogue));
            Some(NpcShopShellView {
                title: screen.title,
                status: status_line(screen.route, phase),
                body: npc_shell_body(&screen, phase, dialogue),
                accent: NPC_ACCENT,
            })
        }
        UiRoute::Shop => {
            let screen = shop_screen(shop_screen_state(shop));
            Some(NpcShopShellView {
                title: screen.title,
                status: status_line(screen.route, phase),
                body: shop_shell_body(&screen, phase, shop),
                accent: SHOP_ACCENT,
            })
        }
        _ => None,
    }
}

fn npc_screen_state(dialogue: &NpcDialogueState) -> NpcScreenState {
    if dialogue.quest_list_mode {
        NpcScreenState::QuestList
    } else if dialogue.selected_answer.is_some() && !dialogue.can_click {
        NpcScreenState::Confirming
    } else {
        NpcScreenState::Dialogue
    }
}

fn shop_screen_state(shop: &NpcShopState) -> ShopScreenState {
    if shop.selling_item {
        ShopScreenState::Selling
    } else if shop.repair_shop && matches!(shop.mode, NpcShopMode::Repair) {
        ShopScreenState::Repair
    } else {
        ShopScreenState::BuySell
    }
}

fn npc_shell_body(screen: &NpcScreen, phase: SessionPhase, dialogue: &NpcDialogueState) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "NPC dialogue shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_paragraph(
        &mut body,
        "Dialogue",
        &format!(
            "npc_key={} | npc_name={} | dialogue_index={} | quest_list_mode={} | can_click={}",
            dialogue.npc_key,
            dialogue.npc_name,
            dialogue.dialogue_index,
            dialogue.quest_list_mode,
            dialogue.can_click,
        ),
    );
    push_paragraph(
        &mut body,
        "Paging",
        &format!(
            "page_index={} | max_page={} | selected_answer={:?} | contribute_point={}",
            dialogue.page_index,
            dialogue.max_page,
            dialogue.selected_answer,
            dialogue.contribute_point,
        ),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn shop_shell_body(screen: &ShopScreen, phase: SessionPhase, shop: &NpcShopState) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "NPC shop shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_paragraph(
        &mut body,
        "Shop",
        &format!(
            "npc_key={} | npc_name={} | tax_rate={} | repair_shop={} | mode={}",
            shop.npc_key,
            shop.npc_name,
            shop.tax_rate,
            shop.repair_shop,
            shop.mode.as_str(),
        ),
    );
    push_paragraph(
        &mut body,
        "Selling",
        &format!(
            "selling_item={} | standby_item_key={:?}",
            shop.selling_item, shop.standby_item_key,
        ),
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn status_line(route: UiRoute, phase: SessionPhase) -> String {
    format!(
        "route={} | group={} | session={}",
        route.slug(),
        route.group().as_str(),
        phase.as_str()
    )
}

fn spawn_npc_shop_shell(commands: &mut Commands, view: &NpcShopShellView) -> Entity {
    let root = commands
        .spawn((
            NpcShopShellRoot,
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

fn clear_npc_shop_shell(commands: &mut Commands, state: &mut NpcShopShellState) {
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
    use super::{npc_shop_shell_view, NpcShopShellPlugin, NpcShopShellRoot};
    use crate::{SessionPhase, SessionState};
    use bevy::prelude::App;
    use mu_gameplay::{NpcManager, NpcPlugin, NpcShopMode};
    use mu_ui::{UiRoute, UiShellState};

    fn npc_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&NpcShopShellRoot>();
        query.iter(world).count()
    }

    fn npc_shell_root_entity(world: &mut bevy::prelude::World) -> Option<bevy::prelude::Entity> {
        let mut query =
            world.query_filtered::<bevy::prelude::Entity, bevy::prelude::With<NpcShopShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn npc_and_shop_shell_views_render_expected_bodies() {
        let mut manager = NpcManager::new();
        manager.open_dialogue(236, "Marlon", 18);
        manager.set_dialogue_page(1, 3);
        manager.set_quest_list_mode(true);
        manager.set_selected_answer(Some(2));
        manager.set_contribute_point(1200);
        manager.set_dialogue_clickable(false);

        let npc_view = npc_shop_shell_view(
            UiRoute::Npc,
            SessionPhase::LoggedIn,
            manager.dialogue(),
            manager.shop(),
        )
        .unwrap();

        assert_eq!(npc_view.title, "NPC");
        assert!(npc_view.status.contains("route=npc | group=gameplay"));
        assert!(npc_view.body.contains("NPC dialogue shell is active."));
        assert!(npc_view
            .body
            .contains("npc_key=236 | npc_name=Marlon | dialogue_index=18"));
        assert!(npc_view.body.contains("quest_list_mode=true"));
        assert!(npc_view.body.contains("Actions"));

        manager.open_shop(42, "Potion Merchant", 5, true);
        manager.set_shop_mode(NpcShopMode::Repair);
        manager.set_shop_standby_item_key(Some(0x0102_0304));

        let shop_view = npc_shop_shell_view(
            UiRoute::Shop,
            SessionPhase::LoggedIn,
            manager.dialogue(),
            manager.shop(),
        )
        .unwrap();

        assert_eq!(shop_view.title, "Shop");
        assert!(shop_view.status.contains("route=shop | group=gameplay"));
        assert!(shop_view.body.contains("NPC shop shell is active."));
        assert!(shop_view.body.contains(
            "npc_key=42 | npc_name=Potion Merchant | tax_rate=5 | repair_shop=true | mode=repair"
        ));
        assert!(shop_view
            .body
            .contains("selling_item=false | standby_item_key=Some(16909060)"));

        assert!(npc_shop_shell_view(
            UiRoute::Npc,
            SessionPhase::Disconnected,
            manager.dialogue(),
            manager.shop(),
        )
        .is_none());
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shells() {
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, NpcShopShellPlugin, NpcPlugin));

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Npc);
        app.insert_resource(ui_shell);

        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        {
            let mut manager = app.world_mut().resource_mut::<NpcManager>();
            manager.open_dialogue(236, "Marlon", 18);
            manager.set_dialogue_page(1, 3);
            manager.open_shop(42, "Potion Merchant", 5, true);
            manager.set_shop_mode(NpcShopMode::Repair);
        }

        app.update();

        assert_eq!(npc_shell_root_count(app.world_mut()), 1);
        let first_root = npc_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Shop);
        app.update();

        assert_eq!(npc_shell_root_count(app.world_mut()), 1);
        let second_root = npc_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut().resource_mut::<SessionState>().disconnect();
        app.update();

        assert_eq!(npc_shell_root_count(app.world_mut()), 0);
    }
}
