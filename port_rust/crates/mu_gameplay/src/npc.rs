use bevy::prelude::{App, Plugin, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NpcShopMode {
    #[default]
    BuySell,
    Repair,
}

impl NpcShopMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuySell => "buy-sell",
            Self::Repair => "repair",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NpcDialogueState {
    pub npc_key: u32,
    pub npc_name: String,
    pub dialogue_index: u32,
    pub quest_list_mode: bool,
    pub can_click: bool,
    pub selected_answer: Option<u8>,
    pub page_index: u8,
    pub max_page: u8,
    pub contribute_point: u32,
}

impl NpcDialogueState {
    pub fn reset_for(&mut self, npc_key: u32, npc_name: impl Into<String>, dialogue_index: u32) {
        self.npc_key = npc_key;
        self.npc_name = npc_name.into();
        self.dialogue_index = dialogue_index;
        self.quest_list_mode = false;
        self.can_click = true;
        self.selected_answer = None;
        self.page_index = 0;
        self.max_page = 0;
        self.contribute_point = 0;
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NpcShopState {
    pub npc_key: u32,
    pub npc_name: String,
    pub tax_rate: i32,
    pub repair_shop: bool,
    pub mode: NpcShopMode,
    pub standby_item_key: Option<u32>,
    pub selling_item: bool,
}

impl NpcShopState {
    pub fn reset_for(
        &mut self,
        npc_key: u32,
        npc_name: impl Into<String>,
        tax_rate: i32,
        repair_shop: bool,
    ) {
        self.npc_key = npc_key;
        self.npc_name = npc_name.into();
        self.tax_rate = tax_rate;
        self.repair_shop = repair_shop;
        self.mode = NpcShopMode::BuySell;
        self.standby_item_key = None;
        self.selling_item = false;
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn set_mode(&mut self, mode: NpcShopMode) {
        self.mode = if self.repair_shop || matches!(mode, NpcShopMode::BuySell) {
            mode
        } else {
            NpcShopMode::BuySell
        };
    }

    pub fn toggle_mode(&mut self) {
        if !self.repair_shop {
            return;
        }

        self.mode = match self.mode {
            NpcShopMode::BuySell => NpcShopMode::Repair,
            NpcShopMode::Repair => NpcShopMode::BuySell,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NpcManager {
    dialogue: NpcDialogueState,
    shop: NpcShopState,
}

impl Resource for NpcManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NpcManager>();
    }
}

impl NpcManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn dialogue(&self) -> &NpcDialogueState {
        &self.dialogue
    }

    pub fn dialogue_mut(&mut self) -> &mut NpcDialogueState {
        &mut self.dialogue
    }

    pub fn shop(&self) -> &NpcShopState {
        &self.shop
    }

    pub fn shop_mut(&mut self) -> &mut NpcShopState {
        &mut self.shop
    }

    pub fn open_dialogue(
        &mut self,
        npc_key: u32,
        npc_name: impl Into<String>,
        dialogue_index: u32,
    ) {
        self.dialogue.reset_for(npc_key, npc_name, dialogue_index);
    }

    pub fn close_dialogue(&mut self) {
        self.dialogue.clear();
    }

    pub fn set_dialogue_page(&mut self, page_index: u8, max_page: u8) {
        self.dialogue.max_page = max_page;
        self.dialogue.page_index = page_index.min(max_page);
    }

    pub fn set_quest_list_mode(&mut self, enabled: bool) {
        self.dialogue.quest_list_mode = enabled;
        if !enabled {
            self.dialogue.selected_answer = None;
        }
    }

    pub fn set_dialogue_clickable(&mut self, can_click: bool) {
        self.dialogue.can_click = can_click;
    }

    pub fn set_selected_answer(&mut self, answer: Option<u8>) {
        self.dialogue.selected_answer = answer;
    }

    pub fn set_contribute_point(&mut self, contribute_point: u32) {
        self.dialogue.contribute_point = contribute_point;
    }

    pub fn open_shop(
        &mut self,
        npc_key: u32,
        npc_name: impl Into<String>,
        tax_rate: i32,
        repair_shop: bool,
    ) {
        self.shop
            .reset_for(npc_key, npc_name, tax_rate, repair_shop);
    }

    pub fn close_shop(&mut self) {
        self.shop.clear();
    }

    pub fn set_shop_tax_rate(&mut self, tax_rate: i32) {
        self.shop.tax_rate = tax_rate;
    }

    pub fn set_shop_repair_capability(&mut self, repair_shop: bool) {
        self.shop.repair_shop = repair_shop;
        if !repair_shop && matches!(self.shop.mode, NpcShopMode::Repair) {
            self.shop.mode = NpcShopMode::BuySell;
        }
    }

    pub fn set_shop_mode(&mut self, mode: NpcShopMode) {
        self.shop.set_mode(mode);
    }

    pub fn toggle_shop_mode(&mut self) {
        self.shop.toggle_mode();
    }

    pub fn set_shop_standby_item_key(&mut self, standby_item_key: Option<u32>) {
        self.shop.standby_item_key = standby_item_key;
    }

    pub fn set_shop_selling_item(&mut self, selling_item: bool) {
        self.shop.selling_item = selling_item;
    }
}

#[cfg(test)]
mod tests {
    use super::{NpcDialogueState, NpcManager, NpcPlugin, NpcShopMode, NpcShopState};
    use bevy::prelude::App;

    #[test]
    fn npc_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(NpcPlugin);

        let manager = app.world().resource::<NpcManager>();
        assert_eq!(manager.dialogue(), &NpcDialogueState::default());
        assert_eq!(manager.shop(), &NpcShopState::default());
    }

    #[test]
    fn npc_manager_tracks_dialogue_state() {
        let mut manager = NpcManager::new();
        manager.open_dialogue(236, "Marlon", 18);
        manager.set_dialogue_page(2, 4);
        manager.set_quest_list_mode(true);
        manager.set_selected_answer(Some(3));
        manager.set_contribute_point(1200);
        manager.set_dialogue_clickable(false);

        let dialogue = manager.dialogue();
        assert_eq!(dialogue.npc_key, 236);
        assert_eq!(dialogue.npc_name, "Marlon");
        assert_eq!(dialogue.dialogue_index, 18);
        assert!(dialogue.quest_list_mode);
        assert!(!dialogue.can_click);
        assert_eq!(dialogue.selected_answer, Some(3));
        assert_eq!(dialogue.page_index, 2);
        assert_eq!(dialogue.max_page, 4);
        assert_eq!(dialogue.contribute_point, 1200);
    }

    #[test]
    fn npc_manager_tracks_shop_state_and_mode_toggles() {
        let mut manager = NpcManager::new();
        manager.open_shop(236, "Potion Merchant", 10, true);

        let shop = manager.shop();
        assert_eq!(shop.npc_key, 236);
        assert_eq!(shop.npc_name, "Potion Merchant");
        assert_eq!(shop.tax_rate, 10);
        assert!(shop.repair_shop);
        assert_eq!(shop.mode, NpcShopMode::BuySell);

        manager.toggle_shop_mode();
        assert_eq!(manager.shop().mode, NpcShopMode::Repair);
        manager.toggle_shop_mode();
        assert_eq!(manager.shop().mode, NpcShopMode::BuySell);

        manager.set_shop_selling_item(true);
        manager.set_shop_standby_item_key(Some(0x0102_0304));
        assert!(manager.shop().selling_item);
        assert_eq!(manager.shop().standby_item_key, Some(0x0102_0304));

        manager.set_shop_repair_capability(false);
        manager.set_shop_mode(NpcShopMode::Repair);
        assert_eq!(manager.shop().mode, NpcShopMode::BuySell);
    }

    #[test]
    fn npc_manager_reset_clears_both_states() {
        let mut manager = NpcManager::new();
        manager.open_dialogue(1, "Wizard", 3);
        manager.open_shop(2, "Armor Merchant", 5, false);

        manager.reset();

        assert_eq!(manager.dialogue(), &NpcDialogueState::default());
        assert_eq!(manager.shop(), &NpcShopState::default());
    }
}
