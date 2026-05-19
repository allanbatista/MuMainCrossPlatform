use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{
    GameShopCatalogState, GameShopExecutionState, GameShopManager, GameShopMode,
    GameShopStorageState, GameShopWalletSummary,
};

const GAME_SHOP_TITLE: &str = "GameShop";
const CATALOG_ACTIONS: &[GameShopAction] = &[
    GameShopAction::Buy,
    GameShopAction::ViewDetails,
    GameShopAction::Storage,
    GameShopAction::Refresh,
    GameShopAction::Close,
];

const DETAILS_ACTIONS: &[GameShopAction] = &[
    GameShopAction::Confirm,
    GameShopAction::Cancel,
    GameShopAction::Close,
];

const STORAGE_ACTIONS: &[GameShopAction] = &[
    GameShopAction::Use,
    GameShopAction::PrevPage,
    GameShopAction::NextPage,
    GameShopAction::Refresh,
    GameShopAction::Close,
];

const EMPTY_ACTIONS: &[GameShopAction] = &[GameShopAction::Refresh, GameShopAction::Close];
const ERROR_ACTIONS: &[GameShopAction] = &[GameShopAction::Retry, GameShopAction::Close];

pub use mu_gameplay::GameShopMode as GameShopScreenState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameShopAction {
    Buy,
    ViewDetails,
    Storage,
    Use,
    PrevPage,
    NextPage,
    Refresh,
    Confirm,
    Cancel,
    Close,
    Retry,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameShopScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: GameShopScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub execution: GameShopExecutionState,
    pub wallet: GameShopWalletSummary,
    pub catalog: Option<GameShopCatalogState>,
    pub storage: Option<GameShopStorageState>,
    pub status_detail: Option<String>,
    pub actions: &'static [GameShopAction],
}

pub fn game_shop_screen(manager: &GameShopManager) -> GameShopScreen {
    let state = manager.mode();
    let (notice, actions) = match state {
        GameShopMode::Catalog => (state.notice(), CATALOG_ACTIONS),
        GameShopMode::Details => (state.notice(), DETAILS_ACTIONS),
        GameShopMode::Storage => (state.notice(), STORAGE_ACTIONS),
        GameShopMode::Empty => (state.notice(), EMPTY_ACTIONS),
        GameShopMode::Error => (state.notice(), ERROR_ACTIONS),
    };

    GameShopScreen {
        route: UiRoute::GameShop,
        layout: UiShellLayout::for_route(UiRoute::GameShop),
        widgets: UiShellWidgetSet::for_route(UiRoute::GameShop),
        state,
        title: GAME_SHOP_TITLE,
        notice,
        execution: manager.execution(),
        wallet: manager.wallet(),
        catalog: match state {
            GameShopMode::Catalog | GameShopMode::Details => Some(manager.catalog().clone()),
            _ => None,
        },
        storage: match state {
            GameShopMode::Storage => Some(manager.storage().clone()),
            _ => None,
        },
        status_detail: manager.status_detail().map(str::to_owned),
        actions,
    }
}

impl GameShopScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|execution={:?}|wallet={:?}|catalog={:?}|storage={:?}|status_detail={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.execution,
            self.wallet,
            self.catalog,
            self.storage,
            self.status_detail,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        game_shop_screen, GameShopScreenState, CATALOG_ACTIONS, DETAILS_ACTIONS, EMPTY_ACTIONS,
        ERROR_ACTIONS, STORAGE_ACTIONS,
    };
    use mu_gameplay::{GameShopManager, GameShopVersion};

    #[test]
    fn game_shop_catalog_and_details_snapshots() {
        let mut manager = GameShopManager::new();
        manager.set_versions(
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
        manager.set_wallet(1_200.0, 540.0, 150.0, 25.0, 90.0);
        manager.show_catalog("Lorencia", "Featured", 1, 4, 12, 8);

        let catalog = game_shop_screen(&manager);
        assert_eq!(
            catalog.clone().snapshot(),
            "route=game-shop|group=gameplay|state=catalog|title=GameShop|notice=Some(\"Browse the cash shop catalog.\")|execution=GameShopExecutionState { shop_open: true, request_opening: false, script_version: GameShopVersion { zone: 1, year: 2024, year_id: 5 }, current_script_version: GameShopVersion { zone: 1, year: 2024, year_id: 4 }, banner_version: GameShopVersion { zone: 2, year: 2024, year_id: 8 }, current_banner_version: GameShopVersion { zone: 2, year: 2024, year_id: 7 } }|wallet=GameShopWalletSummary { total_cash: 1200.0, total_point: 540.0, cash_credit_card: 150.0, cash_prepaid: 25.0, total_mileage: 90.0 }|catalog=Some(GameShopCatalogState { zone_name: \"Lorencia\", category_name: \"Featured\", selected_page: 1, total_pages: 4, package_count: 12, display_package_count: 8, selected_package_name: None, selected_package_price: None, selected_package_price_unit: None, selected_package_quantity: None })|storage=None|status_detail=None|actions=[Buy, ViewDetails, Storage, Refresh, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(catalog.state, GameShopScreenState::Catalog);
        assert_eq!(catalog.actions, CATALOG_ACTIONS);

        manager.show_details("Wing Box", 1_500, "Cash", 1);
        let details = game_shop_screen(&manager);
        assert_eq!(details.state, GameShopScreenState::Details);
        assert_eq!(
            details
                .catalog
                .as_ref()
                .unwrap()
                .selected_package_name
                .as_deref(),
            Some("Wing Box")
        );
        assert_eq!(details.actions, DETAILS_ACTIONS);
    }

    #[test]
    fn game_shop_storage_empty_and_error_snapshots() {
        let mut manager = GameShopManager::new();
        manager.show_storage(Some(1), 2, 5, 4, 22);

        let storage = game_shop_screen(&manager);
        assert_eq!(
            storage.clone().snapshot(),
            "route=game-shop|group=gameplay|state=storage|title=GameShop|notice=Some(\"Review purchased items and gifts.\")|execution=GameShopExecutionState { shop_open: true, request_opening: false, script_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, current_script_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, banner_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, current_banner_version: GameShopVersion { zone: 0, year: 0, year_id: 0 } }|wallet=GameShopWalletSummary { total_cash: 0.0, total_point: 0.0, cash_credit_card: 0.0, cash_prepaid: 0.0, total_mileage: 0.0 }|catalog=None|storage=Some(GameShopStorageState { inventory_type: Some(1), current_page: 2, total_pages: 5, current_item_count: 4, total_item_count: 22 })|status_detail=None|actions=[Use, PrevPage, NextPage, Refresh, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(storage.state, GameShopScreenState::Storage);
        assert_eq!(storage.actions, STORAGE_ACTIONS);

        manager.show_empty();
        let empty = game_shop_screen(&manager);
        assert_eq!(empty.state, GameShopScreenState::Empty);
        assert_eq!(empty.notice, Some("No GameShop items are available."));
        assert_eq!(empty.actions, EMPTY_ACTIONS);

        manager.show_error("GameShop catalog sync failed.");
        let error = game_shop_screen(&manager);
        assert_eq!(
            error.clone().snapshot(),
            "route=game-shop|group=gameplay|state=error|title=GameShop|notice=Some(\"GameShop sync failed.\")|execution=GameShopExecutionState { shop_open: true, request_opening: false, script_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, current_script_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, banner_version: GameShopVersion { zone: 0, year: 0, year_id: 0 }, current_banner_version: GameShopVersion { zone: 0, year: 0, year_id: 0 } }|wallet=GameShopWalletSummary { total_cash: 0.0, total_point: 0.0, cash_credit_card: 0.0, cash_prepaid: 0.0, total_mileage: 0.0 }|catalog=None|storage=None|status_detail=Some(\"GameShop catalog sync failed.\")|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(error.state, GameShopScreenState::Error);
        assert_eq!(error.actions, ERROR_ACTIONS);
    }
}
