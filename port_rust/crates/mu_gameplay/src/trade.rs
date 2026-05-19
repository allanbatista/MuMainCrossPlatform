use bevy::prelude::{App, Plugin, Resource};

use crate::MAX_USERNAME_SIZE;

pub const MAX_TRADE_WAIT_TICKS: u16 = 150;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TradeMode {
    #[default]
    Idle,
    Requested,
    Active,
    Confirming,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradePartnerInfo {
    pub player_id: String,
    pub level: u16,
    pub guild_type: i32,
}

impl Default for TradePartnerInfo {
    fn default() -> Self {
        Self {
            player_id: String::new(),
            level: 0,
            guild_type: -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TradeManager {
    mode: TradeMode,
    partner: TradePartnerInfo,
    partner_trade_gold: u32,
    my_trade_gold: u32,
    partner_confirmed: bool,
    my_confirmed: bool,
    my_trade_wait: u16,
    alert: bool,
}

impl Default for TradeManager {
    fn default() -> Self {
        Self {
            mode: TradeMode::Idle,
            partner: TradePartnerInfo::default(),
            partner_trade_gold: 0,
            my_trade_gold: 0,
            partner_confirmed: false,
            my_confirmed: false,
            my_trade_wait: 0,
            alert: false,
        }
    }
}

impl Resource for TradeManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct TradePlugin;

impl Plugin for TradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TradeManager>();
    }
}

impl TradeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> TradeMode {
        self.mode
    }

    pub fn partner(&self) -> &TradePartnerInfo {
        &self.partner
    }

    pub fn partner_trade_gold(&self) -> u32 {
        self.partner_trade_gold
    }

    pub fn my_trade_gold(&self) -> u32 {
        self.my_trade_gold
    }

    pub fn partner_confirmed(&self) -> bool {
        self.partner_confirmed
    }

    pub fn my_confirmed(&self) -> bool {
        self.my_confirmed
    }

    pub fn my_trade_wait(&self) -> u16 {
        self.my_trade_wait
    }

    pub fn alert(&self) -> bool {
        self.alert
    }

    pub fn request_trade(&mut self, player_id: impl Into<String>, level: u16, guild_type: i32) {
        self.mode = TradeMode::Requested;
        self.partner = TradePartnerInfo {
            player_id: normalize_trade_name(player_id.into()),
            level,
            guild_type,
        };
        self.partner_trade_gold = 0;
        self.my_trade_gold = 0;
        self.partner_confirmed = false;
        self.my_confirmed = false;
        self.my_trade_wait = 0;
        self.alert = true;
    }

    pub fn accept_trade(&mut self) {
        if !matches!(self.mode, TradeMode::Requested) {
            return;
        }

        self.mode = TradeMode::Active;
        self.alert = false;
    }

    pub fn set_partner_trade_gold(&mut self, gold: u32) {
        self.partner_trade_gold = gold;
    }

    pub fn set_my_trade_gold(&mut self, gold: u32) {
        if self.my_trade_gold != gold && self.my_confirmed {
            self.my_confirmed = false;
            self.mode = TradeMode::Active;
            self.my_trade_wait = MAX_TRADE_WAIT_TICKS;
        }

        self.my_trade_gold = gold;
    }

    pub fn toggle_my_confirmed(&mut self) {
        if matches!(self.mode, TradeMode::Idle | TradeMode::Error) {
            return;
        }

        self.my_confirmed = !self.my_confirmed;
        self.mode = if self.my_confirmed {
            TradeMode::Confirming
        } else {
            TradeMode::Active
        };
        self.alert = true;
    }

    pub fn set_partner_confirmed(&mut self, confirmed: bool) {
        self.partner_confirmed = confirmed;
        if self.mode == TradeMode::Error {
            return;
        }

        if confirmed && self.my_confirmed {
            self.mode = TradeMode::Confirming;
            return;
        }

        if !confirmed && self.mode == TradeMode::Confirming {
            self.mode = TradeMode::Active;
        }
    }

    pub fn set_trade_alert(&mut self, alert: bool) {
        self.alert = alert;
    }

    pub fn tick_trade_wait(&mut self) {
        if self.my_trade_wait > 0 {
            self.my_trade_wait -= 1;
        }
    }

    pub fn mark_error(&mut self) {
        self.mode = TradeMode::Error;
    }

    pub fn close_trade(&mut self) {
        self.reset();
    }
}

fn normalize_trade_name(value: String) -> String {
    value.chars().take(MAX_USERNAME_SIZE).collect()
}

#[cfg(test)]
mod tests {
    use super::{TradeManager, TradeMode, TradePlugin, MAX_TRADE_WAIT_TICKS};
    use bevy::prelude::App;

    #[test]
    fn trade_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(TradePlugin);

        let manager = app.world().resource::<TradeManager>();
        assert_eq!(manager.mode(), TradeMode::Idle);
        assert_eq!(manager.partner().player_id, "");
    }

    #[test]
    fn trade_manager_tracks_partner_confirmation_and_wait() {
        let mut manager = TradeManager::new();
        manager.request_trade("VeryLongPartnerName", 320, 4);
        manager.accept_trade();
        manager.set_my_trade_gold(150_000);
        manager.toggle_my_confirmed();
        manager.set_my_trade_gold(175_000);

        assert_eq!(manager.mode(), TradeMode::Active);
        assert_eq!(manager.partner().player_id, "VeryLongPa");
        assert_eq!(manager.partner().level, 320);
        assert_eq!(manager.partner().guild_type, 4);
        assert_eq!(manager.my_trade_gold(), 175_000);
        assert_eq!(manager.my_trade_wait(), MAX_TRADE_WAIT_TICKS);
        assert!(!manager.my_confirmed());
        assert!(manager.alert());
    }
}
