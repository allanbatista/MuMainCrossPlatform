use bevy::prelude::{
    AlignItems, App, BackgroundColor, Color, Commands, Component, Entity, FlexDirection,
    JustifyContent, Node, Plugin, PostUpdate, Res, ResMut, Resource, Text, TextColor, TextFont,
    UiRect, Val,
};
use mu_gameplay::{EquipmentManager, InventoryManager, VaultManager};
use mu_ui::{inventory_screen, InventoryScreenState, UiRoute, UiShellState};

use crate::{ClientRuntime, SessionPhase};

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
const INVENTORY_ACCENT: Color = Color::srgb(0.88, 0.64, 0.18);
const ERROR_ACCENT: Color = Color::srgb(0.95, 0.42, 0.42);

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct InventoryShellPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InventoryShellKey {
    route: UiRoute,
    phase: SessionPhase,
    summary: InventoryShellSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InventoryShellSummary {
    inventory_slots: usize,
    inventory_occupied: usize,
    inventory_empty: bool,
    equipment_occupied: usize,
    equipment_empty: bool,
    vault_money: u32,
    vault_locked: bool,
    vault_verified: bool,
}

#[derive(Debug, Default, Resource)]
struct InventoryShellState {
    root: Option<Entity>,
    key: Option<InventoryShellKey>,
}

#[derive(Debug, Clone)]
struct InventoryShellView {
    title: &'static str,
    status: String,
    body: String,
    accent: Color,
}

#[derive(Component)]
struct InventoryShellRoot;

impl Plugin for InventoryShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryShellState>()
            .add_systems(PostUpdate, sync_inventory_shell_system);
    }
}

fn sync_inventory_shell_system(
    mut commands: Commands,
    ui_shell: Res<UiShellState>,
    session_state: Res<crate::SessionState>,
    client_runtime: Res<ClientRuntime>,
    inventory: Res<InventoryManager>,
    equipment: Res<EquipmentManager>,
    vault: Res<VaultManager>,
    mut state: ResMut<InventoryShellState>,
) {
    let current = inventory_shell_key(
        ui_shell.current(),
        session_state.phase(),
        client_runtime.world_ready(),
        &inventory,
        &equipment,
        &vault,
    );

    let Some(key) = current else {
        clear_inventory_shell(&mut commands, &mut state);
        return;
    };

    if state.key == Some(key) && state.root.is_some() {
        return;
    }

    clear_inventory_shell(&mut commands, &mut state);

    let Some(view) = inventory_shell_view(key.route, key.phase, &inventory, &equipment, &vault)
    else {
        return;
    };

    let root = spawn_inventory_shell(&mut commands, &view);
    state.root = Some(root);
    state.key = Some(key);
}

fn inventory_shell_key(
    route: UiRoute,
    phase: SessionPhase,
    world_ready: bool,
    inventory: &InventoryManager,
    equipment: &EquipmentManager,
    vault: &VaultManager,
) -> Option<InventoryShellKey> {
    if inventory_shell_visible(route, world_ready) {
        Some(InventoryShellKey {
            route,
            phase,
            summary: inventory_shell_summary(inventory, equipment, vault),
        })
    } else {
        None
    }
}

fn inventory_shell_visible(route: UiRoute, world_ready: bool) -> bool {
    route == UiRoute::Inventory && world_ready
}

fn inventory_shell_view(
    route: UiRoute,
    phase: SessionPhase,
    inventory: &InventoryManager,
    equipment: &EquipmentManager,
    vault: &VaultManager,
) -> Option<InventoryShellView> {
    if route != UiRoute::Inventory {
        return None;
    }

    let screen = inventory_screen(inventory_screen_state(phase));
    let body = inventory_shell_body(&screen, phase, inventory, equipment, vault);

    Some(InventoryShellView {
        title: screen.title,
        status: status_line(screen.route, phase),
        body,
        accent: accent_for_state(screen.state),
    })
}

fn inventory_screen_state(phase: SessionPhase) -> InventoryScreenState {
    if phase == SessionPhase::Disconnected {
        InventoryScreenState::Error
    } else {
        InventoryScreenState::Ready
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

fn inventory_shell_body(
    screen: &mu_ui::InventoryScreen,
    phase: SessionPhase,
    inventory: &InventoryManager,
    equipment: &EquipmentManager,
    vault: &VaultManager,
) -> String {
    let mut body = String::new();

    push_paragraph(&mut body, "Status", "Inventory shell is active.");
    push_paragraph(&mut body, "Session", phase.as_str());

    if let Some(notice) = screen.notice {
        push_paragraph(&mut body, "Notice", notice);
    }

    push_paragraph(
        &mut body,
        "Inventory",
        &format!(
            "slots={} | occupied={} | empty={}",
            inventory.slot_count(),
            inventory.occupied_slots(),
            inventory.is_empty()
        ),
    );
    push_paragraph(
        &mut body,
        "Equipment",
        &format!(
            "occupied={} | empty={}",
            equipment.occupied_slots(),
            equipment.is_empty()
        ),
    );
    push_paragraph(
        &mut body,
        "Vault",
        &format!(
            "money={} | locked={} | verified={}",
            vault.money(),
            vault.locked(),
            vault.password_verified()
        ),
    );
    push_lines(
        &mut body,
        "Panels",
        screen.panels.iter().map(|panel| format!("{panel:?}")),
        None,
    );
    push_lines(
        &mut body,
        "Actions",
        screen.actions.iter().map(|action| format!("{action:?}")),
        None,
    );

    body
}

fn inventory_shell_summary(
    inventory: &InventoryManager,
    equipment: &EquipmentManager,
    vault: &VaultManager,
) -> InventoryShellSummary {
    InventoryShellSummary {
        inventory_slots: inventory.slot_count(),
        inventory_occupied: inventory.occupied_slots(),
        inventory_empty: inventory.is_empty(),
        equipment_occupied: equipment.occupied_slots(),
        equipment_empty: equipment.is_empty(),
        vault_money: vault.money(),
        vault_locked: vault.locked(),
        vault_verified: vault.password_verified(),
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

fn accent_for_state(state: InventoryScreenState) -> Color {
    match state {
        InventoryScreenState::Error => ERROR_ACCENT,
        InventoryScreenState::Ready | InventoryScreenState::Moving | InventoryScreenState::Full => {
            INVENTORY_ACCENT
        }
    }
}

fn spawn_inventory_shell(commands: &mut Commands, view: &InventoryShellView) -> Entity {
    let root = commands
        .spawn((
            InventoryShellRoot,
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

fn clear_inventory_shell(commands: &mut Commands, state: &mut InventoryShellState) {
    if let Some(root) = state.root.take() {
        commands.entity(root).despawn();
    }

    state.key = None;
}

#[cfg(test)]
mod tests {
    use super::{inventory_shell_view, InventoryShellPlugin, InventoryShellRoot};
    use crate::{ClientRuntime, SessionPhase};
    use bevy::prelude::App;
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
    use mu_gameplay::{
        EquipmentManager, EquipmentPlugin, InventoryManager, InventoryPlugin, Item, ItemPacketData,
        ItemSize, VaultManager, VaultPlugin,
    };
    use mu_ui::{UiRoute, UiShellState};

    fn repo_assets_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn runtime_with_world() -> ClientRuntime {
        let asset_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
        let mut runtime = ClientRuntime::new();
        runtime.load_world_bundle(bundle);
        runtime
    }

    fn inventory_shell_root_count(world: &mut bevy::prelude::World) -> usize {
        let mut query = world.query::<&InventoryShellRoot>();
        query.iter(world).count()
    }

    fn inventory_shell_root_entity(
        world: &mut bevy::prelude::World,
    ) -> Option<bevy::prelude::Entity> {
        let mut query = world
            .query_filtered::<bevy::prelude::Entity, bevy::prelude::With<InventoryShellRoot>>();
        query.iter(world).next()
    }

    #[test]
    fn inventory_shell_view_renders_ready_and_error_surfaces() {
        let inventory = InventoryManager::new();
        let equipment = EquipmentManager::new();
        let vault = VaultManager::new();
        let ready = inventory_shell_view(
            UiRoute::Inventory,
            SessionPhase::LoggedIn,
            &inventory,
            &equipment,
            &vault,
        )
        .unwrap();
        assert_eq!(ready.title, "Inventory");
        assert!(ready.status.contains("route=inventory | group=gameplay"));
        assert!(ready.body.contains("Inventory shell is active."));
        assert!(ready.body.contains("slots=192 | occupied=0 | empty=true"));
        assert!(ready.body.contains("occupied=0 | empty=true"));
        assert!(ready
            .body
            .contains("money=0 | locked=false | verified=false"));

        let error = inventory_shell_view(
            UiRoute::Inventory,
            SessionPhase::Disconnected,
            &inventory,
            &equipment,
            &vault,
        )
        .unwrap();
        assert!(error.body.contains("Inventory sync failed."));
    }

    #[test]
    fn plugin_spawns_and_clears_the_visible_shell() {
        let mut app = App::new();
        app.add_plugins((
            mu_ui::UiShellPlugin,
            InventoryShellPlugin,
            InventoryPlugin,
            EquipmentPlugin,
            VaultPlugin,
        ));

        let runtime = runtime_with_world();
        runtime
            .world_entities()
            .local_player()
            .expect("local player seeded");
        app.insert_resource(runtime);

        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::Inventory);
        app.insert_resource(ui_shell);

        let mut session_state = crate::SessionState::new();
        assert!(session_state.login_success());
        app.insert_resource(session_state);

        app.update();

        assert_eq!(inventory_shell_root_count(app.world_mut()), 1);
        let first_root = inventory_shell_root_entity(app.world_mut()).unwrap();

        app.world_mut()
            .resource_mut::<InventoryManager>()
            .add(Item::new(ItemPacketData::new(1, 1), ItemSize::new(1, 1)))
            .unwrap();
        app.update();

        assert_eq!(inventory_shell_root_count(app.world_mut()), 1);
        let second_root = inventory_shell_root_entity(app.world_mut()).unwrap();
        assert_ne!(first_root, second_root);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        assert_eq!(inventory_shell_root_count(app.world_mut()), 0);
        assert!(app.world().resource::<ClientRuntime>().world_ready());
    }
}
