use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

use crate::layout::UiShellLayout;
use crate::widgets::UiShellWidgetSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiRoute {
    Boot,
    Loading,
    Login,
    ServerSelect,
    Options,
    Messages,
    CharacterSelect,
    CharacterCreate,
    CharacterDelete,
    World,
    Hud,
    Chat,
    Minimap,
    Hotkeys,
    Inventory,
    Trade,
    Marketplace,
    Vault,
    Shop,
    Npc,
    Gate,
    Events,
    Combat,
    Duel,
    Siege,
    Party,
    Friend,
    Guild,
    Quests,
    MuHelper,
    GameShop,
    EditorAdmin,
    AdminCore,
    AdminItemEditor,
    AdminSkillEditor,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiRouteGroup {
    Startup,
    Authentication,
    Character,
    World,
    Gameplay,
    Admin,
    Error,
}

impl UiRouteGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::Authentication => "authentication",
            Self::Character => "character",
            Self::World => "world",
            Self::Gameplay => "gameplay",
            Self::Admin => "admin",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiRouteDescriptor {
    pub route: UiRoute,
    pub group: UiRouteGroup,
    pub slug: &'static str,
    pub label: &'static str,
    pub fixture_path: &'static str,
}

impl UiRouteDescriptor {
    pub const fn new(
        route: UiRoute,
        group: UiRouteGroup,
        slug: &'static str,
        label: &'static str,
        fixture_path: &'static str,
    ) -> Self {
        Self {
            route,
            group,
            slug,
            label,
            fixture_path,
        }
    }
}

macro_rules! descriptor {
    ($route:ident, $group:ident, $slug:literal, $label:literal, $fixture_path:literal) => {
        UiRouteDescriptor::new(
            UiRoute::$route,
            UiRouteGroup::$group,
            $slug,
            $label,
            $fixture_path,
        )
    };
}

pub const ROUTES: &[UiRouteDescriptor] = &[
    descriptor!(Boot, Startup, "boot", "Boot", "ui_fixtures/boot.snap"),
    descriptor!(
        Loading,
        Startup,
        "loading",
        "Loading",
        "ui_fixtures/loading.snap"
    ),
    descriptor!(
        Login,
        Authentication,
        "login",
        "Login",
        "ui_fixtures/login.snap"
    ),
    descriptor!(
        ServerSelect,
        Authentication,
        "server-select",
        "Server Select",
        "ui_fixtures/server-select.snap"
    ),
    descriptor!(
        Options,
        Authentication,
        "options",
        "Options",
        "ui_fixtures/options.snap"
    ),
    descriptor!(
        Messages,
        Authentication,
        "messages",
        "Messages",
        "ui_fixtures/messages.snap"
    ),
    descriptor!(
        CharacterSelect,
        Character,
        "character-select",
        "Character Select",
        "ui_fixtures/character-select.snap"
    ),
    descriptor!(
        CharacterCreate,
        Character,
        "character-create",
        "Character Create",
        "ui_fixtures/character-create.snap"
    ),
    descriptor!(
        CharacterDelete,
        Character,
        "character-delete",
        "Character Delete",
        "ui_fixtures/character-delete.snap"
    ),
    descriptor!(World, World, "world", "World", "ui_fixtures/world.snap"),
    descriptor!(Hud, World, "hud", "HUD", "ui_fixtures/hud.snap"),
    descriptor!(Chat, World, "chat", "Chat", "ui_fixtures/chat.snap"),
    descriptor!(
        Minimap,
        World,
        "minimap",
        "Minimap",
        "ui_fixtures/minimap.snap"
    ),
    descriptor!(
        Hotkeys,
        World,
        "hotkeys",
        "Hotkeys",
        "ui_fixtures/hotkeys.snap"
    ),
    descriptor!(
        Inventory,
        Gameplay,
        "inventory",
        "Inventory",
        "ui_fixtures/inventory.snap"
    ),
    descriptor!(Trade, Gameplay, "trade", "Trade", "ui_fixtures/trade.snap"),
    descriptor!(
        Marketplace,
        Gameplay,
        "marketplace",
        "Marketplace",
        "ui_fixtures/marketplace.snap"
    ),
    descriptor!(Vault, Gameplay, "vault", "Vault", "ui_fixtures/vault.snap"),
    descriptor!(Shop, Gameplay, "shop", "Shop", "ui_fixtures/shop.snap"),
    descriptor!(Npc, Gameplay, "npc", "NPC", "ui_fixtures/npc.snap"),
    descriptor!(Gate, Gameplay, "gate", "Gate", "ui_fixtures/gate.snap"),
    descriptor!(
        Events,
        Gameplay,
        "events",
        "Events",
        "ui_fixtures/events.snap"
    ),
    descriptor!(
        Combat,
        Gameplay,
        "combat",
        "Combat",
        "ui_fixtures/combat.snap"
    ),
    descriptor!(Duel, Gameplay, "duel", "Duel", "ui_fixtures/duel.snap"),
    descriptor!(Siege, Gameplay, "siege", "Siege", "ui_fixtures/siege.snap"),
    descriptor!(Party, Gameplay, "party", "Party", "ui_fixtures/party.snap"),
    descriptor!(
        Friend,
        Gameplay,
        "friend",
        "Friend",
        "ui_fixtures/friend.snap"
    ),
    descriptor!(Guild, Gameplay, "guild", "Guild", "ui_fixtures/guild.snap"),
    descriptor!(
        Quests,
        Gameplay,
        "quests",
        "Quests",
        "ui_fixtures/quests.snap"
    ),
    descriptor!(
        MuHelper,
        Gameplay,
        "mu-helper",
        "MU Helper",
        "ui_fixtures/mu-helper.snap"
    ),
    descriptor!(
        GameShop,
        Gameplay,
        "game-shop",
        "GameShop",
        "ui_fixtures/game-shop.snap"
    ),
    descriptor!(
        EditorAdmin,
        Admin,
        "editor-admin",
        "Editor Admin",
        "ui_fixtures/editor-admin.snap"
    ),
    descriptor!(
        AdminCore,
        Admin,
        "admin-core",
        "Admin Core",
        "ui_fixtures/admin-core.snap"
    ),
    descriptor!(
        AdminItemEditor,
        Admin,
        "admin-item-editor",
        "Admin Item Editor",
        "ui_fixtures/admin-item-editor.snap"
    ),
    descriptor!(
        AdminSkillEditor,
        Admin,
        "admin-skill-editor",
        "Admin Skill Editor",
        "ui_fixtures/admin-skill-editor.snap"
    ),
    descriptor!(Error, Error, "error", "Error", "ui_fixtures/error.snap"),
];

impl UiRoute {
    pub fn descriptor(self) -> &'static UiRouteDescriptor {
        ROUTES
            .iter()
            .find(|descriptor| descriptor.route == self)
            .expect("missing UI route descriptor")
    }

    pub fn group(self) -> UiRouteGroup {
        self.descriptor().group
    }

    pub fn slug(self) -> &'static str {
        self.descriptor().slug
    }

    pub fn label(self) -> &'static str {
        self.descriptor().label
    }

    pub fn fixture_path(self) -> &'static str {
        self.descriptor().fixture_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiRouteCatalog {
    routes: &'static [UiRouteDescriptor],
}

impl UiRouteCatalog {
    pub const fn routes(self) -> &'static [UiRouteDescriptor] {
        self.routes
    }

    pub fn find_by_slug(&self, slug: &str) -> Option<UiRouteDescriptor> {
        self.routes
            .iter()
            .find(|descriptor| descriptor.slug == slug)
            .copied()
    }

    pub fn find_by_route(&self, route: UiRoute) -> Option<UiRouteDescriptor> {
        self.routes
            .iter()
            .find(|descriptor| descriptor.route == route)
            .copied()
    }
}

impl Default for UiRouteCatalog {
    fn default() -> Self {
        Self { routes: ROUTES }
    }
}

impl Resource for UiRouteCatalog {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiShellState {
    current: UiRoute,
    previous: Option<UiRoute>,
}

impl UiShellState {
    pub fn current(self) -> UiRoute {
        self.current
    }

    pub fn previous(self) -> Option<UiRoute> {
        self.previous
    }

    pub fn set_route(&mut self, route: UiRoute) -> bool {
        if self.current == route {
            return false;
        }

        self.previous = Some(self.current);
        self.current = route;
        true
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn descriptor(self) -> &'static UiRouteDescriptor {
        self.current.descriptor()
    }

    pub fn layout(self) -> UiShellLayout {
        UiShellLayout::for_route(self.current)
    }

    pub fn widgets(self) -> UiShellWidgetSet {
        UiShellWidgetSet::for_route(self.current)
    }
}

impl Default for UiShellState {
    fn default() -> Self {
        Self {
            current: UiRoute::Boot,
            previous: None,
        }
    }
}

impl Resource for UiShellState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct UiShellPlugin;

impl Plugin for UiShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiRouteCatalog>();
        app.init_resource::<UiShellState>();
    }
}

#[cfg(test)]
mod tests {
    use super::{UiRoute, UiRouteCatalog, UiRouteGroup, UiShellPlugin, UiShellState, ROUTES};
    use crate::fixture_routes;
    use bevy::app::App;

    #[test]
    fn route_catalog_matches_fixture_routes() {
        assert_eq!(ROUTES, fixture_routes());
    }

    #[test]
    fn route_descriptors_cover_primary_shell_routes() {
        let primary_routes = [
            UiRoute::Boot,
            UiRoute::Login,
            UiRoute::ServerSelect,
            UiRoute::CharacterSelect,
            UiRoute::World,
            UiRoute::Options,
            UiRoute::Error,
        ];

        for route in primary_routes {
            let descriptor = route.descriptor();
            assert_eq!(descriptor.route, route);
            assert!(!descriptor.slug.is_empty());
            assert!(!descriptor.label.is_empty());
            assert!(!descriptor.fixture_path.is_empty());
        }
    }

    #[test]
    fn shell_state_tracks_route_history_and_helpers() {
        let mut state = UiShellState::default();

        assert_eq!(state.current(), UiRoute::Boot);
        assert_eq!(state.previous(), None);
        assert_eq!(state.descriptor().group, UiRouteGroup::Startup);
        assert_eq!(state.layout().group, UiRouteGroup::Startup);
        assert_eq!(state.widgets().group, UiRouteGroup::Startup);

        assert!(state.set_route(UiRoute::World));
        assert_eq!(state.current(), UiRoute::World);
        assert_eq!(state.previous(), Some(UiRoute::Boot));
        assert_eq!(state.layout().group, UiRouteGroup::World);
        assert_eq!(state.widgets().group, UiRouteGroup::World);
        assert!(!state.set_route(UiRoute::World));

        state.reset();
        assert_eq!(state.current(), UiRoute::Boot);
        assert_eq!(state.previous(), None);
    }

    #[test]
    fn plugin_registers_shell_resources() {
        let mut app = App::new();
        app.add_plugins(UiShellPlugin);

        let catalog = app.world().resource::<UiRouteCatalog>();
        assert_eq!(catalog.routes().len(), ROUTES.len());

        let state = app.world().resource::<UiShellState>();
        assert_eq!(state.current(), UiRoute::Boot);
    }
}
