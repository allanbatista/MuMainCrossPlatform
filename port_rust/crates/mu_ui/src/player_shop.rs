use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const PLAYER_SHOP_TITLE: &str = "Marketplace";
const EDITING_NOTICE: &str = "Prepare the personal shop.";
const OPEN_NOTICE: &str = "Personal shop is open.";
const PRICING_NOTICE: &str = "Set the item price.";
const ERROR_NOTICE: &str = "Player shop sync failed.";

const EDITING_ACTIONS: &[PlayerShopAction] = &[
    PlayerShopAction::OpenShop,
    PlayerShopAction::SetPrice,
    PlayerShopAction::CloseShop,
];
const OPEN_ACTIONS: &[PlayerShopAction] =
    &[PlayerShopAction::SetPrice, PlayerShopAction::CloseShop];
const PRICING_ACTIONS: &[PlayerShopAction] = &[
    PlayerShopAction::Confirm,
    PlayerShopAction::Cancel,
    PlayerShopAction::CloseShop,
];
const ERROR_ACTIONS: &[PlayerShopAction] = &[PlayerShopAction::Retry, PlayerShopAction::CloseShop];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerShopScreenState {
    Editing,
    Open,
    Pricing,
    Error,
}

impl PlayerShopScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editing => "editing",
            Self::Open => "open",
            Self::Pricing => "pricing",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Editing => Some(EDITING_NOTICE),
            Self::Open => Some(OPEN_NOTICE),
            Self::Pricing => Some(PRICING_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerShopAction {
    OpenShop,
    SetPrice,
    Confirm,
    Cancel,
    CloseShop,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerShopScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: PlayerShopScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub shop_title: Option<&'static str>,
    pub personal_shop_enabled: bool,
    pub source_index: Option<usize>,
    pub target_index: Option<usize>,
    pub input_value_text_box_enabled: bool,
    pub price: Option<u32>,
    pub item_count: Option<usize>,
    pub actions: &'static [PlayerShopAction],
}

pub fn player_shop_screen(state: PlayerShopScreenState) -> PlayerShopScreen {
    let (
        notice,
        shop_title,
        personal_shop_enabled,
        source_index,
        target_index,
        input_value_text_box_enabled,
        price,
        item_count,
        actions,
    ) = match state {
        PlayerShopScreenState::Editing => (
            state.notice(),
            Some("Ares Market"),
            false,
            None,
            None,
            false,
            None,
            Some(8),
            EDITING_ACTIONS,
        ),
        PlayerShopScreenState::Open => (
            state.notice(),
            Some("Ares Market"),
            true,
            None,
            None,
            false,
            None,
            Some(8),
            OPEN_ACTIONS,
        ),
        PlayerShopScreenState::Pricing => (
            state.notice(),
            Some("Ares Market"),
            true,
            Some(4),
            Some(0),
            true,
            Some(250_000),
            Some(8),
            PRICING_ACTIONS,
        ),
        PlayerShopScreenState::Error => (
            state.notice(),
            None,
            false,
            None,
            None,
            false,
            None,
            None,
            ERROR_ACTIONS,
        ),
    };

    PlayerShopScreen {
        route: UiRoute::Marketplace,
        layout: UiShellLayout::for_route(UiRoute::Marketplace),
        widgets: UiShellWidgetSet::for_route(UiRoute::Marketplace),
        state,
        title: PLAYER_SHOP_TITLE,
        notice,
        shop_title,
        personal_shop_enabled,
        source_index,
        target_index,
        input_value_text_box_enabled,
        price,
        item_count,
        actions,
    }
}

impl PlayerShopScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|shop_title={:?}|personal_shop_enabled={}|source_index={:?}|target_index={:?}|input_value_text_box_enabled={}|price={:?}|item_count={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.shop_title,
            self.personal_shop_enabled,
            self.source_index,
            self.target_index,
            self.input_value_text_box_enabled,
            self.price,
            self.item_count,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{player_shop_screen, PlayerShopScreenState};

    #[test]
    fn player_shop_editing_and_open_snapshots() {
        assert_eq!(
            player_shop_screen(PlayerShopScreenState::Editing).snapshot(),
            "route=marketplace|group=gameplay|state=editing|title=Marketplace|notice=Some(\"Prepare the personal shop.\")|shop_title=Some(\"Ares Market\")|personal_shop_enabled=false|source_index=None|target_index=None|input_value_text_box_enabled=false|price=None|item_count=Some(8)|actions=[OpenShop, SetPrice, CloseShop]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            player_shop_screen(PlayerShopScreenState::Open).snapshot(),
            "route=marketplace|group=gameplay|state=open|title=Marketplace|notice=Some(\"Personal shop is open.\")|shop_title=Some(\"Ares Market\")|personal_shop_enabled=true|source_index=None|target_index=None|input_value_text_box_enabled=false|price=None|item_count=Some(8)|actions=[SetPrice, CloseShop]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn player_shop_pricing_and_error_snapshots() {
        assert_eq!(
            player_shop_screen(PlayerShopScreenState::Pricing).snapshot(),
            "route=marketplace|group=gameplay|state=pricing|title=Marketplace|notice=Some(\"Set the item price.\")|shop_title=Some(\"Ares Market\")|personal_shop_enabled=true|source_index=Some(4)|target_index=Some(0)|input_value_text_box_enabled=true|price=Some(250000)|item_count=Some(8)|actions=[Confirm, Cancel, CloseShop]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            player_shop_screen(PlayerShopScreenState::Error).snapshot(),
            "route=marketplace|group=gameplay|state=error|title=Marketplace|notice=Some(\"Player shop sync failed.\")|shop_title=None|personal_shop_enabled=false|source_index=None|target_index=None|input_value_text_box_enabled=false|price=None|item_count=None|actions=[Retry, CloseShop]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
