use crate::items::{GridState, Item, ItemError};
use bevy::prelude::{App, Plugin, Resource};
use thiserror::Error;

pub const INVENTORY_MAIN_PAGE_COLUMNS: usize = 8;
pub const INVENTORY_MAIN_PAGE_ROWS: usize = 8;
pub const INVENTORY_EXTENSION_PAGE_COUNT: usize = 4;
pub const INVENTORY_EXTENSION_PAGE_COLUMNS: usize = 8;
pub const INVENTORY_EXTENSION_PAGE_ROWS: usize = 4;
pub const INVENTORY_TOTAL_PAGE_COUNT: usize = 1 + INVENTORY_EXTENSION_PAGE_COUNT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InventorySlot {
    pub page: usize,
    pub slot: usize,
}

impl InventorySlot {
    pub const fn new(page: usize, slot: usize) -> Self {
        Self { page, slot }
    }

    pub const fn main(slot: usize) -> Self {
        Self { page: 0, slot }
    }

    pub const fn extension(page: usize, slot: usize) -> Self {
        Self {
            page: page + 1,
            slot,
        }
    }

    pub fn from_linear(index: usize) -> Option<Self> {
        let main_slots = INVENTORY_MAIN_PAGE_COLUMNS * INVENTORY_MAIN_PAGE_ROWS;
        if index < main_slots {
            return Some(Self::main(index));
        }

        let index = index - main_slots;
        let page_slots = INVENTORY_EXTENSION_PAGE_COLUMNS * INVENTORY_EXTENSION_PAGE_ROWS;
        let page = index / page_slots;
        let slot = index % page_slots;

        if page < INVENTORY_EXTENSION_PAGE_COUNT {
            Some(Self::extension(page, slot))
        } else {
            None
        }
    }

    pub fn linear_index(self) -> Option<usize> {
        let main_slots = INVENTORY_MAIN_PAGE_COLUMNS * INVENTORY_MAIN_PAGE_ROWS;
        let extension_slots = INVENTORY_EXTENSION_PAGE_COLUMNS * INVENTORY_EXTENSION_PAGE_ROWS;

        if self.page == 0 {
            return Some(self.slot);
        }

        if self.page <= INVENTORY_EXTENSION_PAGE_COUNT {
            Some(main_slots + (self.page - 1) * extension_slots + self.slot)
        } else {
            None
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum InventoryError {
    #[error("inventory slot is invalid")]
    InvalidSlot,
    #[error("inventory slot is occupied")]
    SlotOccupied,
    #[error("inventory slot is empty")]
    SlotEmpty,
    #[error("item does not fit in the target slot")]
    ItemDoesNotFit,
    #[error("items are not stackable")]
    NotStackable,
    #[error("stack limit would be exceeded")]
    StackLimitExceeded,
    #[error("item stack is empty")]
    EmptyStack,
    #[error("item error: {0}")]
    Item(#[from] ItemError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryManager {
    pages: [GridState; INVENTORY_TOTAL_PAGE_COUNT],
}

impl Default for InventoryManager {
    fn default() -> Self {
        Self {
            pages: [
                GridState::new(INVENTORY_MAIN_PAGE_COLUMNS, INVENTORY_MAIN_PAGE_ROWS),
                GridState::new(
                    INVENTORY_EXTENSION_PAGE_COLUMNS,
                    INVENTORY_EXTENSION_PAGE_ROWS,
                ),
                GridState::new(
                    INVENTORY_EXTENSION_PAGE_COLUMNS,
                    INVENTORY_EXTENSION_PAGE_ROWS,
                ),
                GridState::new(
                    INVENTORY_EXTENSION_PAGE_COLUMNS,
                    INVENTORY_EXTENSION_PAGE_ROWS,
                ),
                GridState::new(
                    INVENTORY_EXTENSION_PAGE_COLUMNS,
                    INVENTORY_EXTENSION_PAGE_ROWS,
                ),
            ],
        }
    }
}

impl Resource for InventoryManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryManager>();
    }
}

impl InventoryManager {
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

    pub fn item_at(&self, slot: InventorySlot) -> Option<&Item> {
        self.page(slot.page)?.item_at(slot.slot)
    }

    pub fn slot_count(&self) -> usize {
        self.pages.iter().map(GridState::slot_count).sum()
    }

    pub fn occupied_slots(&self) -> usize {
        self.pages.iter().map(GridState::occupied_slots).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.iter().all(GridState::is_empty)
    }

    pub fn find_empty_slot(&self, item: &Item) -> Option<InventorySlot> {
        for (page_index, page) in self.pages.iter().enumerate() {
            if let Some(slot) = page.find_first_fit(item) {
                return Some(InventorySlot::new(page_index, slot));
            }
        }

        None
    }

    pub fn insert(&mut self, slot: InventorySlot, item: Item) -> Result<(), InventoryError> {
        let page = self
            .page_mut(slot.page)
            .ok_or(InventoryError::InvalidSlot)?;
        page.place(slot.slot, item).map_err(map_grid_error)
    }

    pub fn add(&mut self, item: Item) -> Result<InventorySlot, InventoryError> {
        let slot = self
            .find_empty_slot(&item)
            .ok_or(InventoryError::ItemDoesNotFit)?;
        self.insert(slot, item)?;
        Ok(slot)
    }

    pub fn take(&mut self, slot: InventorySlot) -> Result<Item, InventoryError> {
        let page = self
            .page_mut(slot.page)
            .ok_or(InventoryError::InvalidSlot)?;
        page.take(slot.slot).map_err(map_grid_error)
    }

    pub fn remove(&mut self, slot: InventorySlot) -> Result<Item, InventoryError> {
        self.take(slot)
    }

    pub fn move_item(
        &mut self,
        from: InventorySlot,
        to: InventorySlot,
    ) -> Result<(), InventoryError> {
        if from == to {
            return Ok(());
        }

        if self.item_at(to).is_some() {
            return self.stack_items(from, to).map(|_| ());
        }

        let item = self.take(from)?;
        if let Err(error) = self.insert(to, item.clone()) {
            let _ = self.insert(from, item);
            return Err(error);
        }

        Ok(())
    }

    pub fn stack_items(
        &mut self,
        source: InventorySlot,
        target: InventorySlot,
    ) -> Result<u16, InventoryError> {
        if source == target {
            return Ok(0);
        }

        let Some(source_snapshot) = self.item_at(source).cloned() else {
            return Err(InventoryError::SlotEmpty);
        };
        let Some(target_snapshot) = self.item_at(target).cloned() else {
            return Err(InventoryError::SlotEmpty);
        };

        if !source_snapshot.can_stack_with(&target_snapshot) {
            return Err(InventoryError::NotStackable);
        }

        let mut source_item = self.take(source)?;
        let mut target_item = self.take(target)?;
        let moved = target_item.merge_stack(&mut source_item)?;

        self.insert(target, target_item)?;
        if !source_item.is_empty() {
            self.insert(source, source_item)?;
        }

        Ok(moved)
    }

    pub fn use_item(&mut self, slot: InventorySlot, amount: u16) -> Result<Item, InventoryError> {
        let mut item = self.take(slot)?;
        item.consume(amount)?;
        let result = item.clone();

        if !item.is_empty() {
            self.insert(slot, item)?;
        }

        Ok(result)
    }

    pub fn stackable_capacity(&self, slot: InventorySlot) -> Option<u16> {
        self.item_at(slot).map(Item::available_stack_space)
    }
}

fn map_grid_error(error: crate::items::GridError) -> InventoryError {
    match error {
        crate::items::GridError::SlotOutOfBounds => InventoryError::InvalidSlot,
        crate::items::GridError::SlotEmpty => InventoryError::SlotEmpty,
        crate::items::GridError::MissingAnchor => InventoryError::SlotEmpty,
        crate::items::GridError::ItemDoesNotFit => InventoryError::ItemDoesNotFit,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InventoryManager, InventoryPlugin, InventorySlot, INVENTORY_EXTENSION_PAGE_COUNT,
        INVENTORY_EXTENSION_PAGE_ROWS, INVENTORY_MAIN_PAGE_COLUMNS, INVENTORY_MAIN_PAGE_ROWS,
        INVENTORY_TOTAL_PAGE_COUNT,
    };
    use crate::items::{Item, ItemPacketData, ItemSize};
    use bevy::prelude::App;

    fn stackable_item() -> Item {
        Item::stackable(ItemPacketData::new(1, 10), ItemSize::new(1, 1), 20, 5)
    }

    #[test]
    fn plugin_registers_inventory_manager() {
        let mut app = App::new();
        app.add_plugins(InventoryPlugin);

        let inventory = app.world().resource::<InventoryManager>();
        assert_eq!(inventory.slot_count(), 192);
        assert_eq!(inventory.pages.len(), INVENTORY_TOTAL_PAGE_COUNT);
        assert!(inventory.is_empty());
    }

    #[test]
    fn inventory_uses_the_legacy_page_geometry() {
        let inventory = InventoryManager::new();
        assert_eq!(
            inventory.page(0).unwrap().width(),
            INVENTORY_MAIN_PAGE_COLUMNS
        );
        assert_eq!(
            inventory.page(0).unwrap().height(),
            INVENTORY_MAIN_PAGE_ROWS
        );
        for page in 1..=INVENTORY_EXTENSION_PAGE_COUNT {
            assert_eq!(
                inventory.page(page).unwrap().height(),
                INVENTORY_EXTENSION_PAGE_ROWS
            );
        }
    }

    #[test]
    fn inventory_adds_moves_and_stacks_items() {
        let mut inventory = InventoryManager::new();
        let slot = inventory.add(stackable_item()).unwrap();
        assert_eq!(slot, InventorySlot::main(0));

        let second = inventory.add(stackable_item()).unwrap();
        assert_eq!(second, InventorySlot::main(1));
        assert_eq!(inventory.stack_items(second, slot).unwrap(), 5);
        assert!(inventory.item_at(slot).is_some());
    }

    #[test]
    fn inventory_use_item_consumes_stack_items() {
        let mut inventory = InventoryManager::new();
        let slot = inventory.add(stackable_item()).unwrap();
        let item = inventory.use_item(slot, 2).unwrap();
        assert_eq!(item.stack_count, 3);
        assert_eq!(inventory.item_at(slot).unwrap().stack_count, 3);
    }
}
