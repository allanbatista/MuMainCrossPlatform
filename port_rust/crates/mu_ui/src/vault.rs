use bevy::app::{App, Plugin};

use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{VaultManager, VAULT_PAGE_COLUMNS, VAULT_PAGE_COUNT, VAULT_PAGE_ROWS};

const VAULT_TITLE: &str = "Storage";
const OPEN_NOTICE: &str = "Manage stored items and zen.";
const LOCKED_NOTICE: &str = "Enter the vault pin.";
const EXTENDED_NOTICE: &str = "Browse the extended vault storage.";
const ERROR_NOTICE: &str = "Vault sync failed.";

const OPEN_ACTIONS: &[VaultAction] = &[
    VaultAction::DepositZen,
    VaultAction::WithdrawZen,
    VaultAction::LockVault,
    VaultAction::Close,
];
const OPEN_EXTENDED_ACTIONS: &[VaultAction] = &[
    VaultAction::DepositZen,
    VaultAction::WithdrawZen,
    VaultAction::LockVault,
    VaultAction::ToggleExtended,
    VaultAction::Close,
];
const LOCKED_ACTIONS: &[VaultAction] = &[
    VaultAction::DepositZen,
    VaultAction::WithdrawZen,
    VaultAction::UnlockVault,
    VaultAction::Close,
];
const LOCKED_EXTENDED_ACTIONS: &[VaultAction] = &[
    VaultAction::DepositZen,
    VaultAction::WithdrawZen,
    VaultAction::UnlockVault,
    VaultAction::ToggleExtended,
    VaultAction::Close,
];
const EXTENDED_ACTIONS: &[VaultAction] = &[VaultAction::Close];
const ERROR_ACTIONS: &[VaultAction] = &[VaultAction::Retry, VaultAction::Close];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultScreenState {
    Open,
    Locked,
    Verified,
    Extended,
    Error,
}

impl VaultScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Locked => "locked",
            Self::Verified => "verified",
            Self::Extended => "extended",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Open => Some(OPEN_NOTICE),
            Self::Locked => Some(LOCKED_NOTICE),
            Self::Verified => Some(OPEN_NOTICE),
            Self::Extended => Some(EXTENDED_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }

    pub const fn actions(self, expand_button_visible: bool) -> &'static [VaultAction] {
        match self {
            Self::Open => {
                if expand_button_visible {
                    OPEN_EXTENDED_ACTIONS
                } else {
                    OPEN_ACTIONS
                }
            }
            Self::Locked | Self::Verified => {
                if expand_button_visible {
                    LOCKED_EXTENDED_ACTIONS
                } else {
                    LOCKED_ACTIONS
                }
            }
            Self::Extended => EXTENDED_ACTIONS,
            Self::Error => ERROR_ACTIONS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultAction {
    DepositZen,
    WithdrawZen,
    LockVault,
    UnlockVault,
    ToggleExtended,
    Retry,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VaultScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: VaultScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub storage_gold: u32,
    pub locked: bool,
    pub password_verified: bool,
    pub expand_button_visible: bool,
    pub extended_visible: bool,
    pub page_count: usize,
    pub page_rows: usize,
    pub page_columns: usize,
    pub slot_count: usize,
    pub actions: &'static [VaultAction],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct VaultUiPlugin;

impl Plugin for VaultUiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn vault_screen(
    state: VaultScreenState,
    manager: &VaultManager,
    expand_button_visible: bool,
) -> VaultScreen {
    let extended_visible = matches!(state, VaultScreenState::Extended);
    let expand_button_visible = expand_button_visible && !extended_visible;

    VaultScreen {
        route: UiRoute::Vault,
        layout: UiShellLayout::for_route(UiRoute::Vault),
        widgets: UiShellWidgetSet::for_route(UiRoute::Vault),
        state,
        title: VAULT_TITLE,
        notice: state.notice(),
        storage_gold: manager.money(),
        locked: manager.locked(),
        password_verified: manager.password_verified(),
        expand_button_visible,
        extended_visible,
        page_count: VAULT_PAGE_COUNT,
        page_rows: VAULT_PAGE_ROWS,
        page_columns: VAULT_PAGE_COLUMNS,
        slot_count: VAULT_PAGE_COUNT * VAULT_PAGE_ROWS * VAULT_PAGE_COLUMNS,
        actions: state.actions(expand_button_visible),
    }
}

impl VaultScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|storage_gold={}|locked={}|password_verified={}|expand_button_visible={}|extended_visible={}|page_count={}|page_rows={}|page_columns={}|slot_count={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.storage_gold,
            self.locked,
            self.password_verified,
            self.expand_button_visible,
            self.extended_visible,
            self.page_count,
            self.page_rows,
            self.page_columns,
            self.slot_count,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{vault_screen, VaultScreenState};
    use mu_gameplay::VaultManager;

    fn manager_with_storage_gold(storage_gold: u32) -> VaultManager {
        let mut manager = VaultManager::new();
        manager.deposit_money(storage_gold).unwrap();
        manager
    }

    #[test]
    fn vault_open_and_locked_snapshots() {
        let open = vault_screen(
            VaultScreenState::Open,
            &manager_with_storage_gold(1_250_000),
            false,
        );
        assert_eq!(
            open.snapshot(),
            "route=vault|group=gameplay|state=open|title=Storage|notice=Some(\"Manage stored items and zen.\")|storage_gold=1250000|locked=false|password_verified=false|expand_button_visible=false|extended_visible=false|page_count=2|page_rows=15|page_columns=8|slot_count=240|actions=[DepositZen, WithdrawZen, LockVault, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        let mut locked_manager = manager_with_storage_gold(1_250_000);
        locked_manager.apply_status(1).unwrap();
        let locked = vault_screen(VaultScreenState::Locked, &locked_manager, false);
        assert_eq!(
            locked.snapshot(),
            "route=vault|group=gameplay|state=locked|title=Storage|notice=Some(\"Enter the vault pin.\")|storage_gold=1250000|locked=true|password_verified=false|expand_button_visible=false|extended_visible=false|page_count=2|page_rows=15|page_columns=8|slot_count=240|actions=[DepositZen, WithdrawZen, UnlockVault, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn vault_verified_and_extended_snapshots() {
        let mut verified_manager = manager_with_storage_gold(1_250_000);
        verified_manager.apply_status(12).unwrap();

        let verified = vault_screen(VaultScreenState::Verified, &verified_manager, true);
        assert_eq!(
            verified.snapshot(),
            "route=vault|group=gameplay|state=verified|title=Storage|notice=Some(\"Manage stored items and zen.\")|storage_gold=1250000|locked=true|password_verified=true|expand_button_visible=true|extended_visible=false|page_count=2|page_rows=15|page_columns=8|slot_count=240|actions=[DepositZen, WithdrawZen, UnlockVault, ToggleExtended, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        let extended = vault_screen(VaultScreenState::Extended, &verified_manager, true);
        assert_eq!(
            extended.snapshot(),
            "route=vault|group=gameplay|state=extended|title=Storage|notice=Some(\"Browse the extended vault storage.\")|storage_gold=1250000|locked=true|password_verified=true|expand_button_visible=false|extended_visible=true|page_count=2|page_rows=15|page_columns=8|slot_count=240|actions=[Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn vault_error_snapshot() {
        let error = vault_screen(VaultScreenState::Error, &VaultManager::new(), false);
        assert_eq!(
            error.snapshot(),
            "route=vault|group=gameplay|state=error|title=Storage|notice=Some(\"Vault sync failed.\")|storage_gold=0|locked=false|password_verified=false|expand_button_visible=false|extended_visible=false|page_count=2|page_rows=15|page_columns=8|slot_count=240|actions=[Retry, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
