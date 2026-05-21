use bevy::prelude::{App, Plugin, Resource};

pub const MAX_PLAYER_SHOP_TITLE_LENGTH: usize = 26;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerShopMode {
    #[default]
    Editing,
    Open,
    Pricing,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerShopManager {
    mode: PlayerShopMode,
    title: String,
    personal_shop_enabled: bool,
    source_index: Option<usize>,
    target_index: Option<usize>,
    price: Option<u32>,
    input_value_text_box_enabled: bool,
    item_count: usize,
}

impl Default for PlayerShopManager {
    fn default() -> Self {
        Self {
            mode: PlayerShopMode::Editing,
            title: String::new(),
            personal_shop_enabled: false,
            source_index: None,
            target_index: None,
            price: None,
            input_value_text_box_enabled: false,
            item_count: 0,
        }
    }
}

impl Resource for PlayerShopManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct PlayerShopPlugin;

impl Plugin for PlayerShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShopManager>();
    }
}

impl PlayerShopManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> PlayerShopMode {
        self.mode
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn personal_shop_enabled(&self) -> bool {
        self.personal_shop_enabled
    }

    pub fn source_index(&self) -> Option<usize> {
        self.source_index
    }

    pub fn target_index(&self) -> Option<usize> {
        self.target_index
    }

    pub fn price(&self) -> Option<u32> {
        self.price
    }

    pub fn input_value_text_box_enabled(&self) -> bool {
        self.input_value_text_box_enabled
    }

    pub fn item_count(&self) -> usize {
        self.item_count
    }

    pub fn can_open(&self) -> bool {
        self.personal_shop_enabled && !self.title.trim().is_empty()
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = normalize_player_shop_title(title.into());
    }

    pub fn set_personal_shop_enabled(&mut self, enabled: bool) {
        self.personal_shop_enabled = enabled;
        self.mode = if enabled {
            PlayerShopMode::Open
        } else {
            PlayerShopMode::Editing
        };

        if !enabled {
            self.price = None;
            self.input_value_text_box_enabled = false;
        }
    }

    pub fn set_item_selection(&mut self, source_index: Option<usize>, target_index: Option<usize>) {
        self.source_index = source_index;
        self.target_index = target_index;
    }

    pub fn set_price(&mut self, price: u32) {
        self.price = Some(price);
        self.input_value_text_box_enabled = true;
        self.mode = PlayerShopMode::Pricing;
    }

    pub fn clear_price(&mut self) {
        self.price = None;
        self.input_value_text_box_enabled = false;
        self.mode = if self.personal_shop_enabled {
            PlayerShopMode::Open
        } else {
            PlayerShopMode::Editing
        };
    }

    pub fn set_input_value_text_box_enabled(&mut self, enabled: bool) {
        self.input_value_text_box_enabled = enabled;
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        self.item_count = item_count;
    }

    pub fn mark_error(&mut self) {
        self.mode = PlayerShopMode::Error;
    }

    pub fn clear_error(&mut self) {
        self.mode = if self.personal_shop_enabled {
            PlayerShopMode::Open
        } else {
            PlayerShopMode::Editing
        };
    }
}

fn normalize_player_shop_title(value: String) -> String {
    value.chars().take(MAX_PLAYER_SHOP_TITLE_LENGTH).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        PlayerShopManager, PlayerShopMode, PlayerShopPlugin, MAX_PLAYER_SHOP_TITLE_LENGTH,
    };
    use bevy::prelude::App;

    #[test]
    fn player_shop_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(PlayerShopPlugin);

        let manager = app.world().resource::<PlayerShopManager>();
        assert_eq!(manager.mode(), PlayerShopMode::Editing);
        assert!(!manager.personal_shop_enabled());
    }

    #[test]
    fn player_shop_title_is_truncated_and_openable() {
        let mut manager = PlayerShopManager::new();
        manager.set_title("Marketplace for veteran heroes only");
        manager.set_item_count(4);
        manager.set_personal_shop_enabled(true);

        assert_eq!(manager.mode(), PlayerShopMode::Open);
        assert_eq!(
            manager.title().chars().count(),
            MAX_PLAYER_SHOP_TITLE_LENGTH
        );
        assert!(manager.can_open());
        assert_eq!(manager.item_count(), 4);
    }

    #[test]
    fn player_shop_pricing_tracks_selection_and_input_box() {
        let mut manager = PlayerShopManager::new();
        manager.set_title("Ares Market");
        manager.set_personal_shop_enabled(true);
        manager.set_item_selection(Some(4), Some(0));
        manager.set_price(250_000);

        assert_eq!(manager.mode(), PlayerShopMode::Pricing);
        assert_eq!(manager.source_index(), Some(4));
        assert_eq!(manager.target_index(), Some(0));
        assert_eq!(manager.price(), Some(250_000));
        assert!(manager.input_value_text_box_enabled());
    }
}
