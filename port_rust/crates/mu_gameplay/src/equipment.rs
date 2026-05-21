use crate::characters::CharacterSheet;
use crate::items::{EquipmentSlot, Item};
use bevy::prelude::{App, Plugin, Resource};
use thiserror::Error;

pub const MAX_EQUIPMENT_SLOTS: usize = 12;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EquipmentError {
    #[error("equipment slot is invalid")]
    InvalidSlot,
    #[error("equipment slot is occupied")]
    SlotOccupied,
    #[error("equipment slot is empty")]
    SlotEmpty,
    #[error("item does not have an equipment slot")]
    MissingEquipmentSlot,
    #[error("item does not fit the selected slot")]
    IncompatibleSlot,
    #[error("item requirements are not met")]
    RequirementMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentManager {
    slots: [Option<Item>; MAX_EQUIPMENT_SLOTS],
}

impl Default for EquipmentManager {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
        }
    }
}

impl Resource for EquipmentManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EquipmentManager>();
    }
}

impl EquipmentManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn slot(&self, slot: EquipmentSlot) -> Option<&Item> {
        self.slots[slot.as_index()].as_ref()
    }

    pub fn slot_mut(&mut self, slot: EquipmentSlot) -> Option<&mut Item> {
        self.slots[slot.as_index()].as_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    pub fn occupied_slots(&self) -> usize {
        self.slots.iter().filter(|item| item.is_some()).count()
    }

    pub fn take(&mut self, slot: EquipmentSlot) -> Result<Item, EquipmentError> {
        self.slots[slot.as_index()]
            .take()
            .ok_or(EquipmentError::SlotEmpty)
    }

    pub fn force_equip(
        &mut self,
        slot: EquipmentSlot,
        item: Item,
        sheet: &CharacterSheet,
    ) -> Result<(), EquipmentError> {
        if !item.can_be_used_by(sheet) {
            return Err(EquipmentError::RequirementMismatch);
        }

        self.force_equip_without_requirements(slot, item)
    }

    pub fn force_equip_without_requirements(
        &mut self,
        slot: EquipmentSlot,
        item: Item,
    ) -> Result<(), EquipmentError> {
        if !slot_allows_item(slot, &item) {
            return Err(EquipmentError::IncompatibleSlot);
        }

        let slot_index = slot.as_index();
        if self.slots[slot_index].is_some() {
            return Err(EquipmentError::SlotOccupied);
        }

        self.slots[slot_index] = Some(item);
        Ok(())
    }

    pub fn equip_without_requirements(
        &mut self,
        item: Item,
    ) -> Result<EquipmentSlot, EquipmentError> {
        let slot = self.resolve_slot(&item)?;
        self.force_equip_without_requirements(slot, item)?;
        Ok(slot)
    }

    pub fn equip(
        &mut self,
        item: Item,
        sheet: &CharacterSheet,
    ) -> Result<EquipmentSlot, EquipmentError> {
        if !item.can_be_used_by(sheet) {
            return Err(EquipmentError::RequirementMismatch);
        }

        let slot = self.resolve_slot(&item)?;
        self.force_equip(slot, item, sheet)?;
        Ok(slot)
    }

    pub fn unequip(&mut self, slot: EquipmentSlot) -> Result<Item, EquipmentError> {
        self.take(slot)
    }

    pub fn unequip_all(&mut self) -> Vec<Item> {
        let mut items = Vec::new();

        for slot in 0..MAX_EQUIPMENT_SLOTS {
            if let Some(item) = self.slots[slot].take() {
                items.push(item);
            }
        }

        items
    }

    pub fn resolve_slot(&self, item: &Item) -> Result<EquipmentSlot, EquipmentError> {
        let preferred = item
            .equipment_slot
            .ok_or(EquipmentError::MissingEquipmentSlot)?;

        if self.slots[preferred.as_index()].is_none() {
            return Ok(preferred);
        }

        match (preferred, item.two_handed) {
            (EquipmentSlot::WeaponRight, false) => {
                let alternate = EquipmentSlot::WeaponLeft;
                if self.slots[alternate.as_index()].is_none() {
                    Ok(alternate)
                } else {
                    Err(EquipmentError::SlotOccupied)
                }
            }
            (EquipmentSlot::RingRight, _) => {
                let alternate = EquipmentSlot::RingLeft;
                if self.slots[alternate.as_index()].is_none() {
                    Ok(alternate)
                } else {
                    Err(EquipmentError::SlotOccupied)
                }
            }
            _ => Err(EquipmentError::SlotOccupied),
        }
    }
}

fn slot_allows_item(slot: EquipmentSlot, item: &Item) -> bool {
    match item.equipment_slot {
        Some(preferred) if preferred == slot => true,
        Some(EquipmentSlot::WeaponRight) if slot == EquipmentSlot::WeaponLeft => !item.two_handed,
        Some(EquipmentSlot::RingRight) if slot == EquipmentSlot::RingLeft => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{EquipmentManager, EquipmentPlugin};
    use crate::characters::CharacterSheet;
    use crate::classes::CharacterClass;
    use crate::items::{EquipmentSlot, Item, ItemPacketData, ItemRequirements, ItemSize};
    use bevy::prelude::App;

    fn weapon() -> Item {
        Item::equipment(
            ItemPacketData::new(0, 10),
            ItemSize::new(1, 2),
            EquipmentSlot::WeaponRight,
            ItemRequirements::default(),
            false,
        )
    }

    fn ring() -> Item {
        Item::equipment(
            ItemPacketData::new(0, 11),
            ItemSize::new(1, 1),
            EquipmentSlot::RingRight,
            ItemRequirements::default(),
            false,
        )
    }

    #[test]
    fn plugin_registers_equipment_manager() {
        let mut app = App::new();
        app.add_plugins(EquipmentPlugin);

        let equipment = app.world().resource::<EquipmentManager>();
        assert!(equipment.is_empty());
        assert_eq!(equipment.occupied_slots(), 0);
    }

    #[test]
    fn equipment_manager_uses_alternate_right_hand_and_ring_slots() {
        let mut equipment = EquipmentManager::new();
        let sheet = CharacterSheet::new(CharacterClass::Knight);

        assert_eq!(
            equipment.equip(weapon(), &sheet).unwrap(),
            EquipmentSlot::WeaponRight
        );
        assert_eq!(
            equipment.equip(ring(), &sheet).unwrap(),
            EquipmentSlot::RingRight
        );

        let alternate_weapon = weapon();
        assert!(equipment
            .force_equip(EquipmentSlot::WeaponLeft, alternate_weapon.clone(), &sheet)
            .is_ok());

        let mut equipment = EquipmentManager::new();
        equipment
            .force_equip(EquipmentSlot::WeaponRight, weapon(), &sheet)
            .unwrap();
        let slot = equipment.equip(
            Item::equipment(
                ItemPacketData::new(0, 12),
                ItemSize::new(1, 2),
                EquipmentSlot::WeaponRight,
                ItemRequirements::default(),
                false,
            ),
            &sheet,
        );
        assert_eq!(slot.unwrap(), EquipmentSlot::WeaponLeft);

        let mut equipment = EquipmentManager::new();
        equipment
            .force_equip(EquipmentSlot::RingRight, ring(), &sheet)
            .unwrap();
        let slot = equipment.equip(
            Item::equipment(
                ItemPacketData::new(0, 14),
                ItemSize::new(1, 1),
                EquipmentSlot::RingRight,
                ItemRequirements::default(),
                false,
            ),
            &sheet,
        );
        assert_eq!(slot.unwrap(), EquipmentSlot::RingLeft);
    }

    #[test]
    fn equipment_manager_rejects_items_without_slots() {
        let mut equipment = EquipmentManager::new();
        let sheet = CharacterSheet::new(CharacterClass::Knight);
        let item = Item::new(ItemPacketData::new(0, 13), ItemSize::new(1, 1));

        assert!(matches!(
            equipment.equip(item, &sheet),
            Err(super::EquipmentError::MissingEquipmentSlot)
        ));
    }

    #[test]
    fn equipment_manager_equips_without_requirements_using_the_legacy_slot_rules() {
        let mut equipment = EquipmentManager::new();
        let slot = equipment.equip_without_requirements(weapon()).unwrap();

        assert_eq!(slot, EquipmentSlot::WeaponRight);

        let slot = equipment.equip_without_requirements(ring()).unwrap();
        assert_eq!(slot, EquipmentSlot::RingRight);
    }
}
