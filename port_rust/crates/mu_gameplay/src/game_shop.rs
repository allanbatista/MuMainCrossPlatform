use bevy::prelude::{App, Plugin, Resource};

use crate::game_shop_transaction::GameShopTransactionManager;

const EMPTY_NOTICE: &str = "No GameShop items are available.";
const CATALOG_NOTICE: &str = "Browse the cash shop catalog.";
const DETAILS_NOTICE: &str = "Review the selected package.";
const STORAGE_NOTICE: &str = "Review purchased items and gifts.";
const ERROR_NOTICE: &str = "GameShop sync failed.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameShopMode {
    #[default]
    Empty,
    Catalog,
    Details,
    Storage,
    Error,
}

impl GameShopMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Catalog => "catalog",
            Self::Details => "details",
            Self::Storage => "storage",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Empty => Some(EMPTY_NOTICE),
            Self::Catalog => Some(CATALOG_NOTICE),
            Self::Details => Some(DETAILS_NOTICE),
            Self::Storage => Some(STORAGE_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameShopWalletSummary {
    pub total_cash: f64,
    pub total_point: f64,
    pub cash_credit_card: f64,
    pub cash_prepaid: f64,
    pub total_mileage: f64,
}

impl Default for GameShopWalletSummary {
    fn default() -> Self {
        Self {
            total_cash: 0.0,
            total_point: 0.0,
            cash_credit_card: 0.0,
            cash_prepaid: 0.0,
            total_mileage: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GameShopCatalogState {
    pub zone_name: String,
    pub category_name: String,
    pub selected_page: u32,
    pub total_pages: u32,
    pub package_count: usize,
    pub display_package_count: usize,
    pub selected_package_name: Option<String>,
    pub selected_package_price: Option<u32>,
    pub selected_package_price_unit: Option<String>,
    pub selected_package_quantity: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GameShopStorageState {
    pub inventory_type: Option<u8>,
    pub current_page: u32,
    pub total_pages: u32,
    pub current_item_count: usize,
    pub total_item_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameShopVersion {
    pub zone: u16,
    pub year: u16,
    pub year_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameShopExecutionState {
    pub shop_open: bool,
    pub request_opening: bool,
    pub script_version: GameShopVersion,
    pub current_script_version: GameShopVersion,
    pub banner_version: GameShopVersion,
    pub current_banner_version: GameShopVersion,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameShopManager {
    mode: GameShopMode,
    execution: GameShopExecutionState,
    wallet: GameShopWalletSummary,
    catalog: GameShopCatalogState,
    storage: GameShopStorageState,
    transactions: GameShopTransactionManager,
    error: Option<String>,
}

impl Default for GameShopManager {
    fn default() -> Self {
        Self {
            mode: GameShopMode::Empty,
            execution: GameShopExecutionState::default(),
            wallet: GameShopWalletSummary::default(),
            catalog: GameShopCatalogState::default(),
            storage: GameShopStorageState::default(),
            transactions: GameShopTransactionManager::default(),
            error: None,
        }
    }
}

impl Resource for GameShopManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct GameShopPlugin;

impl Plugin for GameShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameShopManager>();
    }
}

impl GameShopManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> GameShopMode {
        self.mode
    }

    pub fn execution(&self) -> GameShopExecutionState {
        self.execution
    }

    pub fn wallet(&self) -> GameShopWalletSummary {
        self.wallet
    }

    pub fn catalog(&self) -> &GameShopCatalogState {
        &self.catalog
    }

    pub fn storage(&self) -> &GameShopStorageState {
        &self.storage
    }

    pub fn transaction(&self) -> &GameShopTransactionManager {
        &self.transactions
    }

    pub fn transaction_mut(&mut self) -> &mut GameShopTransactionManager {
        &mut self.transactions
    }

    pub fn status_detail(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn is_open(&self) -> bool {
        self.execution.shop_open
    }

    pub fn set_opening_requested(&mut self, opening_requested: bool) {
        self.execution.request_opening = opening_requested;
    }

    pub fn open_shop(&mut self) {
        self.execution.shop_open = true;
        self.execution.request_opening = false;
    }

    pub fn close_shop(&mut self) {
        self.execution.shop_open = false;
        self.execution.request_opening = false;
        self.mode = GameShopMode::Empty;
        self.error = None;
        self.transactions.cancel_active();
    }

    pub fn set_versions(
        &mut self,
        script_version: GameShopVersion,
        current_script_version: GameShopVersion,
        banner_version: GameShopVersion,
        current_banner_version: GameShopVersion,
    ) {
        self.execution.script_version = script_version;
        self.execution.current_script_version = current_script_version;
        self.execution.banner_version = banner_version;
        self.execution.current_banner_version = current_banner_version;
    }

    pub fn set_wallet(
        &mut self,
        total_cash: f64,
        total_point: f64,
        cash_credit_card: f64,
        cash_prepaid: f64,
        total_mileage: f64,
    ) {
        self.wallet = GameShopWalletSummary {
            total_cash,
            total_point,
            cash_credit_card,
            cash_prepaid,
            total_mileage,
        };
    }

    pub fn show_catalog(
        &mut self,
        zone_name: impl Into<String>,
        category_name: impl Into<String>,
        selected_page: u32,
        total_pages: u32,
        package_count: usize,
        display_package_count: usize,
    ) {
        self.open_shop();
        self.error = None;
        self.storage = GameShopStorageState::default();
        self.catalog = GameShopCatalogState {
            zone_name: zone_name.into(),
            category_name: category_name.into(),
            selected_page,
            total_pages,
            package_count,
            display_package_count,
            selected_package_name: None,
            selected_package_price: None,
            selected_package_price_unit: None,
            selected_package_quantity: None,
        };
        self.mode = if package_count > 0 {
            GameShopMode::Catalog
        } else {
            GameShopMode::Empty
        };
    }

    pub fn show_details(
        &mut self,
        package_name: impl Into<String>,
        price: u32,
        price_unit: impl Into<String>,
        quantity: u32,
    ) {
        self.open_shop();
        self.error = None;
        self.catalog.selected_package_name = None;
        self.catalog.selected_package_price = None;
        self.catalog.selected_package_price_unit = None;
        self.catalog.selected_package_quantity = None;
        self.catalog.selected_package_name = Some(package_name.into());
        self.catalog.selected_package_price = Some(price);
        self.catalog.selected_package_price_unit = Some(price_unit.into());
        self.catalog.selected_package_quantity = Some(quantity);
        self.mode = GameShopMode::Details;
    }

    pub fn show_storage(
        &mut self,
        inventory_type: Option<u8>,
        current_page: u32,
        total_pages: u32,
        current_item_count: usize,
        total_item_count: usize,
    ) {
        self.open_shop();
        self.error = None;
        self.catalog.selected_package_name = None;
        self.catalog.selected_package_price = None;
        self.catalog.selected_package_price_unit = None;
        self.catalog.selected_package_quantity = None;
        self.storage = GameShopStorageState {
            inventory_type,
            current_page,
            total_pages,
            current_item_count,
            total_item_count,
        };
        self.mode = if total_item_count > 0 {
            GameShopMode::Storage
        } else {
            GameShopMode::Empty
        };
    }

    pub fn show_empty(&mut self) {
        self.open_shop();
        self.error = None;
        self.catalog.selected_package_name = None;
        self.catalog.selected_package_price = None;
        self.catalog.selected_package_price_unit = None;
        self.catalog.selected_package_quantity = None;
        self.catalog.package_count = 0;
        self.catalog.display_package_count = 0;
        self.storage = GameShopStorageState::default();
        self.mode = GameShopMode::Empty;
    }

    pub fn show_error(&mut self, error: impl Into<String>) {
        self.open_shop();
        self.error = Some(error.into());
        self.mode = GameShopMode::Error;
    }

    pub fn clear_error(&mut self) {
        self.error = None;
        self.mode = self.visible_mode();
    }

    fn visible_mode(&self) -> GameShopMode {
        if !self.execution.shop_open {
            return GameShopMode::Empty;
        }

        if self.catalog.selected_package_name.is_some() {
            return GameShopMode::Details;
        }

        if self.storage.total_item_count > 0 {
            return GameShopMode::Storage;
        }

        if self.catalog.package_count > 0 {
            return GameShopMode::Catalog;
        }

        GameShopMode::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GameShopCatalogState, GameShopExecutionState, GameShopManager, GameShopMode,
        GameShopPlugin, GameShopStorageState, GameShopVersion, GameShopWalletSummary,
    };
    use bevy::prelude::App;
    use mu_network::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::cash_shop::{
        cash_shop_event_item_list_request, cash_shop_open_state, cash_shop_point_info_request,
        cash_shop_storage_list_request,
    };
    use std::net::SocketAddr;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;

    #[test]
    fn game_shop_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(GameShopPlugin);

        let manager = app.world().resource::<GameShopManager>();
        assert_eq!(manager.mode(), GameShopMode::Empty);
        assert_eq!(manager.execution(), GameShopExecutionState::default());
        assert_eq!(manager.wallet(), GameShopWalletSummary::default());
        assert!(!manager.transaction().is_pending());
    }

    #[test]
    fn game_shop_tracks_catalog_details_and_storage() {
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
        assert_eq!(manager.mode(), GameShopMode::Catalog);

        let catalog = manager.catalog();
        assert_eq!(catalog.zone_name, "Lorencia");
        assert_eq!(catalog.category_name, "Featured");
        assert_eq!(catalog.selected_page, 1);
        assert_eq!(catalog.total_pages, 4);
        assert_eq!(catalog.package_count, 12);
        assert_eq!(catalog.display_package_count, 8);

        manager.show_details("Wing Box", 1_500, "Cash", 1);
        assert_eq!(manager.mode(), GameShopMode::Details);
        assert_eq!(
            manager.catalog(),
            &GameShopCatalogState {
                zone_name: "Lorencia".into(),
                category_name: "Featured".into(),
                selected_page: 1,
                total_pages: 4,
                package_count: 12,
                display_package_count: 8,
                selected_package_name: Some("Wing Box".into()),
                selected_package_price: Some(1_500),
                selected_package_price_unit: Some("Cash".into()),
                selected_package_quantity: Some(1),
            }
        );

        manager.show_storage(Some(1), 2, 5, 4, 22);
        assert_eq!(manager.mode(), GameShopMode::Storage);
        assert_eq!(
            manager.storage(),
            &GameShopStorageState {
                inventory_type: Some(1),
                current_page: 2,
                total_pages: 5,
                current_item_count: 4,
                total_item_count: 22,
            }
        );
        assert_eq!(
            manager.wallet(),
            GameShopWalletSummary {
                total_cash: 1_200.0,
                total_point: 540.0,
                cash_credit_card: 150.0,
                cash_prepaid: 25.0,
                total_mileage: 90.0,
            }
        );
        assert_eq!(
            manager.execution(),
            GameShopExecutionState {
                shop_open: true,
                request_opening: false,
                script_version: GameShopVersion {
                    zone: 1,
                    year: 2024,
                    year_id: 5,
                },
                current_script_version: GameShopVersion {
                    zone: 1,
                    year: 2024,
                    year_id: 4,
                },
                banner_version: GameShopVersion {
                    zone: 2,
                    year: 2024,
                    year_id: 8,
                },
                current_banner_version: GameShopVersion {
                    zone: 2,
                    year: 2024,
                    year_id: 7,
                },
            }
        );
    }

    #[test]
    fn game_shop_empty_and_error_states_are_visible() {
        let mut manager = GameShopManager::new();
        manager.show_empty();
        assert_eq!(manager.mode(), GameShopMode::Empty);
        assert!(manager.status_detail().is_none());

        manager.show_error("GameShop catalog sync failed.");
        assert_eq!(manager.mode(), GameShopMode::Error);
        assert_eq!(
            manager.status_detail(),
            Some("GameShop catalog sync failed.")
        );

        manager.clear_error();
        assert_eq!(manager.mode(), GameShopMode::Empty);
        assert!(manager.status_detail().is_none());
    }

    #[tokio::test]
    async fn game_shop_fake_server_accepts_cash_shop_packet_sequence() {
        let point_info = cash_shop_point_info_request().unwrap();
        let storage_list = cash_shop_storage_list_request(1, 0).unwrap();
        let event_items = cash_shop_event_item_list_request(0x0102_0304).unwrap();
        let close_state = cash_shop_open_state(true).unwrap();

        let server = FakeServer::spawn(
            "127.0.0.1:0".parse::<SocketAddr>().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .expect_packet(point_info.clone())
                    .expect_packet(storage_list.clone())
                    .expect_packet(event_items.clone())
                    .expect_packet(close_state.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut client = TcpStream::connect(server.address()).await.unwrap();
        client.write_all(&point_info).await.unwrap();
        client.write_all(&storage_list).await.unwrap();
        client.write_all(&event_items).await.unwrap();
        client.write_all(&close_state).await.unwrap();

        server.finish().await.unwrap();
    }

    #[test]
    fn game_shop_closing_clears_pending_transactions() {
        let mut manager = GameShopManager::new();
        manager.transaction_mut().begin_purchase().unwrap();
        assert!(manager.transaction().is_pending());

        manager.close_shop();

        assert_eq!(manager.mode(), GameShopMode::Empty);
        assert!(!manager.transaction().is_pending());
        assert!(manager.transaction().last_completion().is_none());
    }
}
