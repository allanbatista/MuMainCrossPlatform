use crate::items::{GridState, Item, ItemError, ItemSize};
use bevy::prelude::{App, Plugin, Resource};
use thiserror::Error;

pub const VAULT_PAGE_COUNT: usize = 2;
pub const VAULT_PAGE_ROWS: usize = 15;
pub const VAULT_PAGE_COLUMNS: usize = 8;
pub const VAULT_PAGE_SLOTS: usize = VAULT_PAGE_ROWS * VAULT_PAGE_COLUMNS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VaultSlot {
    pub page: usize,
    pub slot: usize,
}

impl VaultSlot {
    pub const fn new(page: usize, slot: usize) -> Self {
        Self { page, slot }
    }

    pub fn from_linear(index: usize) -> Option<Self> {
        if index < VAULT_PAGE_COUNT * VAULT_PAGE_SLOTS {
            Some(Self::new(
                index / VAULT_PAGE_SLOTS,
                index % VAULT_PAGE_SLOTS,
            ))
        } else {
            None
        }
    }

    pub fn linear_index(self) -> Option<usize> {
        if self.page < VAULT_PAGE_COUNT && self.slot < VAULT_PAGE_SLOTS {
            Some(self.page * VAULT_PAGE_SLOTS + self.slot)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultMoneyDirection {
    VaultToInventory,
    InventoryToVault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultPendingTransfer {
    Money {
        direction: VaultMoneyDirection,
        amount: u32,
    },
    Item {
        from: VaultSlot,
        to: VaultSlot,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultSyncOutcome {
    Unlocked,
    Locked,
    Confirmed,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VaultError {
    #[error("vault is locked")]
    Locked,
    #[error("vault password was not verified")]
    PasswordNotVerified,
    #[error("vault slot is invalid")]
    InvalidSlot,
    #[error("vault slot is occupied")]
    SlotOccupied,
    #[error("vault slot is empty")]
    SlotEmpty,
    #[error("vault transfer is already pending")]
    PendingTransferAlreadyExists,
    #[error("vault transfer is missing")]
    NoPendingTransfer,
    #[error("vault money amount is invalid")]
    InvalidAmount,
    #[error("vault has insufficient money")]
    InsufficientMoney,
    #[error("vault money would overflow")]
    MoneyOverflow,
    #[error("vault storage rejected the operation")]
    StorageRejected,
    #[error("vault password was rejected")]
    PasswordRejected,
    #[error("vault auto-move failed")]
    AutoMoveFailed,
    #[error("item error: {0}")]
    Item(#[from] ItemError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultManager {
    pages: [GridState; VAULT_PAGE_COUNT],
    money: u32,
    locked: bool,
    password_verified: bool,
    pending_transfer: Option<VaultPendingTransfer>,
}

impl Default for VaultManager {
    fn default() -> Self {
        Self {
            pages: [
                GridState::new(VAULT_PAGE_COLUMNS, VAULT_PAGE_ROWS),
                GridState::new(VAULT_PAGE_COLUMNS, VAULT_PAGE_ROWS),
            ],
            money: 0,
            locked: false,
            password_verified: false,
            pending_transfer: None,
        }
    }
}

impl Resource for VaultManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct VaultPlugin;

impl Plugin for VaultPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VaultManager>();
    }
}

impl VaultManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn page(&self, page: usize) -> Option<&GridState> {
        self.pages.get(page)
    }

    pub(crate) fn page_mut(&mut self, page: usize) -> Option<&mut GridState> {
        self.pages.get_mut(page)
    }

    pub fn money(&self) -> u32 {
        self.money
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn password_verified(&self) -> bool {
        self.password_verified
    }

    pub fn pending_transfer(&self) -> Option<&VaultPendingTransfer> {
        self.pending_transfer.as_ref()
    }

    pub fn is_accessible(&self) -> bool {
        !self.locked || self.password_verified
    }

    pub fn find_empty_slot(&self, item: &Item) -> Option<VaultSlot> {
        for (page_index, page) in self.pages.iter().enumerate() {
            if let Some(slot) = page.find_first_fit(item) {
                return Some(VaultSlot::new(page_index, slot));
            }
        }

        None
    }

    pub fn item_at(&self, slot: VaultSlot) -> Option<&Item> {
        self.page(slot.page)?.item_at(slot.slot)
    }

    pub fn place(&mut self, slot: VaultSlot, item: Item) -> Result<(), VaultError> {
        if !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        self.place_unchecked(slot, item)
    }

    pub fn take(&mut self, slot: VaultSlot) -> Result<Item, VaultError> {
        if !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        self.take_unchecked(slot)
    }

    pub fn move_item(&mut self, from: VaultSlot, to: VaultSlot) -> Result<(), VaultError> {
        if !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        self.move_item_unchecked(from, to)
    }

    pub fn queue_item_transfer(
        &mut self,
        from: VaultSlot,
        to: VaultSlot,
    ) -> Result<(), VaultError> {
        self.queue_transfer(VaultPendingTransfer::Item { from, to })
    }

    pub fn queue_money_transfer(
        &mut self,
        direction: VaultMoneyDirection,
        amount: u32,
    ) -> Result<(), VaultError> {
        if amount == 0 {
            return Err(VaultError::InvalidAmount);
        }

        self.queue_transfer(VaultPendingTransfer::Money { direction, amount })
    }

    pub fn commit_pending_transfer(&mut self) -> Result<VaultSyncOutcome, VaultError> {
        let pending = self
            .pending_transfer
            .clone()
            .ok_or(VaultError::NoPendingTransfer)?;

        match pending {
            VaultPendingTransfer::Money { direction, amount } => {
                self.apply_money_transfer(direction, amount, true)?;
            }
            VaultPendingTransfer::Item { from, to } => {
                self.move_item_unchecked(from, to)?;
            }
        }

        self.pending_transfer = None;
        Ok(VaultSyncOutcome::Confirmed)
    }

    pub fn deposit_money(&mut self, amount: u32) -> Result<(), VaultError> {
        if amount == 0 {
            return Err(VaultError::InvalidAmount);
        }

        if !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        self.add_money(amount)
    }

    pub fn withdraw_money(&mut self, amount: u32) -> Result<(), VaultError> {
        if amount == 0 {
            return Err(VaultError::InvalidAmount);
        }

        if !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        self.remove_money(amount)
    }

    pub fn apply_status(&mut self, status: u8) -> Result<VaultSyncOutcome, VaultError> {
        match status {
            0 => {
                self.locked = false;
                self.password_verified = false;
                self.pending_transfer = None;
                Ok(VaultSyncOutcome::Unlocked)
            }
            1 => {
                self.locked = true;
                self.password_verified = false;
                self.pending_transfer = None;
                Ok(VaultSyncOutcome::Locked)
            }
            10 => {
                self.pending_transfer = None;
                Err(VaultError::AutoMoveFailed)
            }
            11 => {
                self.pending_transfer = None;
                Err(VaultError::PasswordRejected)
            }
            12 => {
                if self.pending_transfer.is_some() {
                    self.commit_pending_transfer()?;
                }

                self.locked = true;
                self.password_verified = true;
                Ok(VaultSyncOutcome::Confirmed)
            }
            13 => {
                self.pending_transfer = None;
                Err(VaultError::StorageRejected)
            }
            _ => Err(VaultError::StorageRejected),
        }
    }

    fn place_unchecked(&mut self, slot: VaultSlot, item: Item) -> Result<(), VaultError> {
        let page = self.page_mut(slot.page).ok_or(VaultError::InvalidSlot)?;
        page.place(slot.slot, item).map_err(map_grid_error)
    }

    fn take_unchecked(&mut self, slot: VaultSlot) -> Result<Item, VaultError> {
        let page = self.page_mut(slot.page).ok_or(VaultError::InvalidSlot)?;
        page.take(slot.slot).map_err(map_grid_error)
    }

    fn queue_transfer(&mut self, transfer: VaultPendingTransfer) -> Result<(), VaultError> {
        if self.pending_transfer.is_some() {
            return Err(VaultError::PendingTransferAlreadyExists);
        }

        self.pending_transfer = Some(transfer);
        Ok(())
    }

    fn move_item_unchecked(&mut self, from: VaultSlot, to: VaultSlot) -> Result<(), VaultError> {
        if from == to {
            return Ok(());
        }

        let item = self.take_unchecked(from)?;
        if let Err(error) = self.place_unchecked(to, item.clone()) {
            let _ = self.place_unchecked(from, item);
            return Err(error);
        }

        Ok(())
    }

    fn apply_money_transfer(
        &mut self,
        direction: VaultMoneyDirection,
        amount: u32,
        bypass_access: bool,
    ) -> Result<(), VaultError> {
        if amount == 0 {
            return Err(VaultError::InvalidAmount);
        }

        if !bypass_access && !self.is_accessible() {
            return Err(VaultError::Locked);
        }

        match direction {
            VaultMoneyDirection::VaultToInventory => self.remove_money(amount),
            VaultMoneyDirection::InventoryToVault => self.add_money(amount),
        }
    }

    fn add_money(&mut self, amount: u32) -> Result<(), VaultError> {
        self.money = self
            .money
            .checked_add(amount)
            .ok_or(VaultError::MoneyOverflow)?;
        Ok(())
    }

    fn remove_money(&mut self, amount: u32) -> Result<(), VaultError> {
        if self.money < amount {
            return Err(VaultError::InsufficientMoney);
        }

        self.money -= amount;
        Ok(())
    }
}

fn map_grid_error(error: crate::items::GridError) -> VaultError {
    match error {
        crate::items::GridError::SlotOutOfBounds => VaultError::InvalidSlot,
        crate::items::GridError::SlotEmpty => VaultError::SlotEmpty,
        crate::items::GridError::MissingAnchor => VaultError::SlotEmpty,
        crate::items::GridError::ItemDoesNotFit => VaultError::SlotOccupied,
    }
}

#[allow(dead_code)]
fn _vault_size() -> ItemSize {
    ItemSize::new(VAULT_PAGE_COLUMNS as u8, VAULT_PAGE_ROWS as u8)
}

#[cfg(test)]
mod tests {
    use super::{
        VaultManager, VaultMoneyDirection, VaultPlugin, VaultSlot, VaultSyncOutcome,
        VAULT_PAGE_COUNT, VAULT_PAGE_ROWS, VAULT_PAGE_SLOTS,
    };
    use crate::items::{Item, ItemPacketData, ItemSize};
    use bevy::prelude::App;

    fn item() -> Item {
        Item::new(ItemPacketData::new(1, 1), ItemSize::new(1, 1))
    }

    #[test]
    fn plugin_registers_vault_manager() {
        let mut app = App::new();
        app.add_plugins(VaultPlugin);

        let vault = app.world().resource::<VaultManager>();
        assert!(!vault.locked());
        assert_eq!(vault.page(0).unwrap().height(), VAULT_PAGE_ROWS);
        assert_eq!(vault.page(0).unwrap().slot_count(), VAULT_PAGE_SLOTS);
        assert_eq!(vault.pages.len(), VAULT_PAGE_COUNT);
    }

    #[test]
    fn vault_status_flow_matches_legacy_lock_and_confirmation_states() {
        let mut vault = VaultManager::new();

        assert_eq!(vault.apply_status(1).unwrap(), VaultSyncOutcome::Locked);
        assert!(vault.locked());
        assert!(!vault.password_verified());

        vault
            .queue_money_transfer(VaultMoneyDirection::InventoryToVault, 100)
            .unwrap();
        assert_eq!(vault.apply_status(12).unwrap(), VaultSyncOutcome::Confirmed);
        assert!(vault.locked());
        assert!(vault.password_verified());
    }

    #[test]
    fn vault_commits_item_transfers_and_money() {
        let mut vault = VaultManager::new();
        let slot_a = VaultSlot::new(0, 0);
        let slot_b = VaultSlot::new(1, 0);
        vault.place(slot_a, item()).unwrap();
        vault.queue_item_transfer(slot_a, slot_b).unwrap();
        vault.apply_status(12).unwrap();
        assert!(vault.item_at(slot_b).is_some());

        vault
            .queue_money_transfer(VaultMoneyDirection::InventoryToVault, 250)
            .unwrap();
        vault.apply_status(12).unwrap();
        assert_eq!(vault.money(), 250);
    }

    #[test]
    fn vault_slot_linear_index_rejects_out_of_bounds_slots() {
        assert_eq!(VaultSlot::new(0, VAULT_PAGE_SLOTS).linear_index(), None);
        assert_eq!(VaultSlot::new(VAULT_PAGE_COUNT, 0).linear_index(), None);
        assert_eq!(
            VaultSlot::from_linear(VAULT_PAGE_SLOTS + 4),
            Some(VaultSlot::new(1, 4))
        );
    }
}
