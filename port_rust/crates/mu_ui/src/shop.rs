use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const SHOP_TITLE: &str = "Shop";
const BUY_SELL_NOTICE: &str = "Buy or sell items with the NPC.";
const REPAIR_NOTICE: &str = "Repair items or repair all.";
const SELLING_NOTICE: &str = "Confirm the sale before continuing.";
const ERROR_NOTICE: &str = "NPC shop sync failed.";

const BUY_SELL_ACTIONS: &[ShopAction] = &[
    ShopAction::Buy,
    ShopAction::Sell,
    ShopAction::ToggleMode,
    ShopAction::Close,
];

const REPAIR_ACTIONS: &[ShopAction] = &[
    ShopAction::Repair,
    ShopAction::RepairAll,
    ShopAction::ToggleMode,
    ShopAction::Close,
];

const SELLING_ACTIONS: &[ShopAction] =
    &[ShopAction::Confirm, ShopAction::Cancel, ShopAction::Close];

const ERROR_ACTIONS: &[ShopAction] = &[ShopAction::Retry, ShopAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopScreenState {
    BuySell,
    Repair,
    Selling,
    Error,
}

impl ShopScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuySell => "buy-sell",
            Self::Repair => "repair",
            Self::Selling => "selling",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::BuySell => Some(BUY_SELL_NOTICE),
            Self::Repair => Some(REPAIR_NOTICE),
            Self::Selling => Some(SELLING_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopAction {
    Buy,
    Sell,
    Repair,
    RepairAll,
    ToggleMode,
    Confirm,
    Cancel,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShopScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: ShopScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub npc_name: Option<&'static str>,
    pub tax_rate: Option<i32>,
    pub repair_shop: bool,
    pub selling_item: bool,
    pub standby_item_key: Option<u32>,
    pub actions: &'static [ShopAction],
}

pub fn shop_screen(state: ShopScreenState) -> ShopScreen {
    let (notice, npc_name, tax_rate, repair_shop, selling_item, standby_item_key, actions) =
        match state {
            ShopScreenState::BuySell => (
                state.notice(),
                Some("Potion Merchant"),
                Some(5),
                false,
                false,
                None,
                BUY_SELL_ACTIONS,
            ),
            ShopScreenState::Repair => (
                state.notice(),
                Some("Armor Merchant"),
                Some(10),
                true,
                false,
                None,
                REPAIR_ACTIONS,
            ),
            ShopScreenState::Selling => (
                state.notice(),
                Some("Potion Merchant"),
                Some(5),
                false,
                true,
                Some(0x0102_0304),
                SELLING_ACTIONS,
            ),
            ShopScreenState::Error => (
                state.notice(),
                None,
                None,
                false,
                false,
                None,
                ERROR_ACTIONS,
            ),
        };

    ShopScreen {
        route: UiRoute::Shop,
        layout: UiShellLayout::for_route(UiRoute::Shop),
        widgets: UiShellWidgetSet::for_route(UiRoute::Shop),
        state,
        title: SHOP_TITLE,
        notice,
        npc_name,
        tax_rate,
        repair_shop,
        selling_item,
        standby_item_key,
        actions,
    }
}

impl ShopScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|npc_name={:?}|tax_rate={:?}|repair_shop={}|selling_item={}|standby_item_key={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.npc_name,
            self.tax_rate,
            self.repair_shop,
            self.selling_item,
            self.standby_item_key,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{shop_screen, ShopScreenState};

    #[test]
    fn shop_buy_sell_and_repair_snapshots() {
        assert_eq!(
            shop_screen(ShopScreenState::BuySell).snapshot(),
            "route=shop|group=gameplay|state=buy-sell|title=Shop|notice=Some(\"Buy or sell items with the NPC.\")|npc_name=Some(\"Potion Merchant\")|tax_rate=Some(5)|repair_shop=false|selling_item=false|standby_item_key=None|actions=[Buy, Sell, ToggleMode, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            shop_screen(ShopScreenState::Repair).snapshot(),
            "route=shop|group=gameplay|state=repair|title=Shop|notice=Some(\"Repair items or repair all.\")|npc_name=Some(\"Armor Merchant\")|tax_rate=Some(10)|repair_shop=true|selling_item=false|standby_item_key=None|actions=[Repair, RepairAll, ToggleMode, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn shop_selling_and_error_snapshots() {
        assert_eq!(
            shop_screen(ShopScreenState::Selling).snapshot(),
            "route=shop|group=gameplay|state=selling|title=Shop|notice=Some(\"Confirm the sale before continuing.\")|npc_name=Some(\"Potion Merchant\")|tax_rate=Some(5)|repair_shop=false|selling_item=true|standby_item_key=Some(16909060)|actions=[Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            shop_screen(ShopScreenState::Error).snapshot(),
            "route=shop|group=gameplay|state=error|title=Shop|notice=Some(\"NPC shop sync failed.\")|npc_name=None|tax_rate=None|repair_shop=false|selling_item=false|standby_item_key=None|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
