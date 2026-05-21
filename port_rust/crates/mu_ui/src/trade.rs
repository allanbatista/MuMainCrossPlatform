use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const TRADE_TITLE: &str = "Trade";
const REQUESTED_NOTICE: &str = "Trade request received.";
const ACTIVE_NOTICE: &str = "Exchange items and zen.";
const CONFIRMING_NOTICE: &str = "Confirm the trade before closing.";
const ERROR_NOTICE: &str = "Trade sync failed.";

const REQUESTED_ACTIONS: &[TradeAction] = &[
    TradeAction::Accept,
    TradeAction::Decline,
    TradeAction::Close,
];
const ACTIVE_ACTIONS: &[TradeAction] = &[
    TradeAction::MoveItem,
    TradeAction::SetGold,
    TradeAction::Confirm,
    TradeAction::Cancel,
    TradeAction::Close,
];
const CONFIRMING_ACTIONS: &[TradeAction] = &[
    TradeAction::Confirm,
    TradeAction::Cancel,
    TradeAction::Close,
];
const ERROR_ACTIONS: &[TradeAction] = &[TradeAction::Retry, TradeAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeScreenState {
    Requested,
    Active,
    Confirming,
    Error,
}

impl TradeScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Active => "active",
            Self::Confirming => "confirming",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Requested => Some(REQUESTED_NOTICE),
            Self::Active => Some(ACTIVE_NOTICE),
            Self::Confirming => Some(CONFIRMING_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeAction {
    Accept,
    Decline,
    MoveItem,
    SetGold,
    Confirm,
    Cancel,
    Close,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradeScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: TradeScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub partner_id: Option<&'static str>,
    pub partner_level: Option<u16>,
    pub partner_guild_type: Option<i32>,
    pub partner_trade_gold: Option<u32>,
    pub my_trade_gold: Option<u32>,
    pub partner_confirmed: bool,
    pub my_confirmed: bool,
    pub trade_alert: bool,
    pub my_trade_wait: Option<u16>,
    pub actions: &'static [TradeAction],
}

pub fn trade_screen(state: TradeScreenState) -> TradeScreen {
    let (
        notice,
        partner_id,
        partner_level,
        partner_guild_type,
        partner_trade_gold,
        my_trade_gold,
        partner_confirmed,
        my_confirmed,
        trade_alert,
        my_trade_wait,
        actions,
    ) = match state {
        TradeScreenState::Requested => (
            state.notice(),
            Some("Blade"),
            Some(320),
            Some(4),
            Some(0),
            Some(0),
            false,
            false,
            true,
            Some(0),
            REQUESTED_ACTIONS,
        ),
        TradeScreenState::Active => (
            state.notice(),
            Some("Blade"),
            Some(320),
            Some(4),
            Some(250_000),
            Some(100_000),
            false,
            false,
            false,
            Some(150),
            ACTIVE_ACTIONS,
        ),
        TradeScreenState::Confirming => (
            state.notice(),
            Some("Blade"),
            Some(320),
            Some(4),
            Some(250_000),
            Some(100_000),
            true,
            true,
            true,
            Some(0),
            CONFIRMING_ACTIONS,
        ),
        TradeScreenState::Error => (
            state.notice(),
            None,
            None,
            None,
            None,
            None,
            false,
            false,
            false,
            None,
            ERROR_ACTIONS,
        ),
    };

    TradeScreen {
        route: UiRoute::Trade,
        layout: UiShellLayout::for_route(UiRoute::Trade),
        widgets: UiShellWidgetSet::for_route(UiRoute::Trade),
        state,
        title: TRADE_TITLE,
        notice,
        partner_id,
        partner_level,
        partner_guild_type,
        partner_trade_gold,
        my_trade_gold,
        partner_confirmed,
        my_confirmed,
        trade_alert,
        my_trade_wait,
        actions,
    }
}

impl TradeScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|partner_id={:?}|partner_level={:?}|partner_guild_type={:?}|partner_trade_gold={:?}|my_trade_gold={:?}|partner_confirmed={}|my_confirmed={}|trade_alert={}|my_trade_wait={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.partner_id,
            self.partner_level,
            self.partner_guild_type,
            self.partner_trade_gold,
            self.my_trade_gold,
            self.partner_confirmed,
            self.my_confirmed,
            self.trade_alert,
            self.my_trade_wait,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{trade_screen, TradeScreenState};

    #[test]
    fn trade_requested_and_active_snapshots() {
        assert_eq!(
            trade_screen(TradeScreenState::Requested).snapshot(),
            "route=trade|group=gameplay|state=requested|title=Trade|notice=Some(\"Trade request received.\")|partner_id=Some(\"Blade\")|partner_level=Some(320)|partner_guild_type=Some(4)|partner_trade_gold=Some(0)|my_trade_gold=Some(0)|partner_confirmed=false|my_confirmed=false|trade_alert=true|my_trade_wait=Some(0)|actions=[Accept, Decline, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            trade_screen(TradeScreenState::Active).snapshot(),
            "route=trade|group=gameplay|state=active|title=Trade|notice=Some(\"Exchange items and zen.\")|partner_id=Some(\"Blade\")|partner_level=Some(320)|partner_guild_type=Some(4)|partner_trade_gold=Some(250000)|my_trade_gold=Some(100000)|partner_confirmed=false|my_confirmed=false|trade_alert=false|my_trade_wait=Some(150)|actions=[MoveItem, SetGold, Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn trade_confirming_and_error_snapshots() {
        assert_eq!(
            trade_screen(TradeScreenState::Confirming).snapshot(),
            "route=trade|group=gameplay|state=confirming|title=Trade|notice=Some(\"Confirm the trade before closing.\")|partner_id=Some(\"Blade\")|partner_level=Some(320)|partner_guild_type=Some(4)|partner_trade_gold=Some(250000)|my_trade_gold=Some(100000)|partner_confirmed=true|my_confirmed=true|trade_alert=true|my_trade_wait=Some(0)|actions=[Confirm, Cancel, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            trade_screen(TradeScreenState::Error).snapshot(),
            "route=trade|group=gameplay|state=error|title=Trade|notice=Some(\"Trade sync failed.\")|partner_id=None|partner_level=None|partner_guild_type=None|partner_trade_gold=None|my_trade_gold=None|partner_confirmed=false|my_confirmed=false|trade_alert=false|my_trade_wait=None|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
