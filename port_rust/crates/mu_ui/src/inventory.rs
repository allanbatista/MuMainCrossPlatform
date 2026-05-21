use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const INVENTORY_TITLE: &str = "Inventory";
const READY_NOTICE: &str = "Items can be moved, used, or equipped.";
const MOVING_NOTICE: &str = "Rearranging items...";
const FULL_NOTICE: &str = "Inventory is full.";
const ERROR_NOTICE: &str = "Inventory sync failed.";

const INVENTORY_PANELS: &[InventoryPanel] = &[
    InventoryPanel::Equipment,
    InventoryPanel::MainInventory,
    InventoryPanel::ExtensionPages,
    InventoryPanel::VaultAccess,
    InventoryPanel::ItemTooltip,
    InventoryPanel::ActionBar,
];

const INVENTORY_ACTIONS: &[InventoryAction] = &[
    InventoryAction::Move,
    InventoryAction::Use,
    InventoryAction::Equip,
    InventoryAction::Unequip,
    InventoryAction::Split,
    InventoryAction::Drop,
    InventoryAction::Store,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryScreenState {
    Ready,
    Moving,
    Full,
    Error,
}

impl InventoryScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Moving => "moving",
            Self::Full => "full",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Ready => Some(READY_NOTICE),
            Self::Moving => Some(MOVING_NOTICE),
            Self::Full => Some(FULL_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryPanel {
    Equipment,
    MainInventory,
    ExtensionPages,
    VaultAccess,
    ItemTooltip,
    ActionBar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryAction {
    Move,
    Use,
    Equip,
    Unequip,
    Split,
    Drop,
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InventoryScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: InventoryScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub panels: &'static [InventoryPanel],
    pub actions: &'static [InventoryAction],
}

pub fn inventory_screen(state: InventoryScreenState) -> InventoryScreen {
    InventoryScreen {
        route: UiRoute::Inventory,
        layout: UiShellLayout::for_route(UiRoute::Inventory),
        widgets: UiShellWidgetSet::for_route(UiRoute::Inventory),
        state,
        title: INVENTORY_TITLE,
        notice: state.notice(),
        panels: INVENTORY_PANELS,
        actions: INVENTORY_ACTIONS,
    }
}

impl InventoryScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|panels={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.panels,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{inventory_screen, InventoryScreenState};

    #[test]
    fn inventory_ready_snapshot_lists_panels_and_actions() {
        assert_eq!(
            inventory_screen(InventoryScreenState::Ready).snapshot(),
            "route=inventory|group=gameplay|state=ready|title=Inventory|notice=Some(\"Items can be moved, used, or equipped.\")|panels=[Equipment, MainInventory, ExtensionPages, VaultAccess, ItemTooltip, ActionBar]|actions=[Move, Use, Equip, Unequip, Split, Drop, Store]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn inventory_error_and_full_snapshots() {
        assert_eq!(
            inventory_screen(InventoryScreenState::Moving).snapshot(),
            "route=inventory|group=gameplay|state=moving|title=Inventory|notice=Some(\"Rearranging items...\")|panels=[Equipment, MainInventory, ExtensionPages, VaultAccess, ItemTooltip, ActionBar]|actions=[Move, Use, Equip, Unequip, Split, Drop, Store]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            inventory_screen(InventoryScreenState::Full).snapshot(),
            "route=inventory|group=gameplay|state=full|title=Inventory|notice=Some(\"Inventory is full.\")|panels=[Equipment, MainInventory, ExtensionPages, VaultAccess, ItemTooltip, ActionBar]|actions=[Move, Use, Equip, Unequip, Split, Drop, Store]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
