use crate::characters::CharacterSheet;
use crate::classes::CharacterClass;
use crate::stats::BASE_CLASS_COUNT;
use bevy::prelude::{App, Plugin};
use thiserror::Error;

pub const MAX_ITEM_INDEX: u16 = 512;
pub const MAX_ITEM_SOCKETS: usize = 5;
pub const SOCKET_EMPTY: u8 = 0xFE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EquipmentSlot {
    WeaponRight = 0,
    WeaponLeft = 1,
    Helm = 2,
    Armor = 3,
    Pants = 4,
    Gloves = 5,
    Boots = 6,
    Wing = 7,
    Helper = 8,
    Amulet = 9,
    RingRight = 10,
    RingLeft = 11,
}

impl EquipmentSlot {
    pub const fn as_index(self) -> usize {
        self as usize
    }

    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::WeaponRight),
            1 => Some(Self::WeaponLeft),
            2 => Some(Self::Helm),
            3 => Some(Self::Armor),
            4 => Some(Self::Pants),
            5 => Some(Self::Gloves),
            6 => Some(Self::Boots),
            7 => Some(Self::Wing),
            8 => Some(Self::Helper),
            9 => Some(Self::Amulet),
            10 => Some(Self::RingRight),
            11 => Some(Self::RingLeft),
            _ => None,
        }
    }

    pub const fn alternate(self) -> Option<Self> {
        match self {
            Self::WeaponRight => Some(Self::WeaponLeft),
            Self::RingRight => Some(Self::RingLeft),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct ItemOptionFlags(u8);

impl ItemOptionFlags {
    pub const HAS_OPTION: u8 = 0x01;
    pub const HAS_LUCK: u8 = 0x02;
    pub const HAS_SKILL: u8 = 0x04;
    pub const HAS_EXCELLENT: u8 = 0x08;
    pub const HAS_ANCIENT: u8 = 0x10;
    pub const HAS_HARMONY: u8 = 0x20;
    pub const HAS_GUARDIAN: u8 = 0x40;
    pub const HAS_SOCKETS: u8 = 0x80;

    pub const fn new(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn with(self, flag: u8) -> Self {
        Self(self.0 | flag)
    }

    pub const fn has_option(self) -> bool {
        self.0 & Self::HAS_OPTION != 0
    }

    pub const fn has_luck(self) -> bool {
        self.0 & Self::HAS_LUCK != 0
    }

    pub const fn has_skill(self) -> bool {
        self.0 & Self::HAS_SKILL != 0
    }

    pub const fn has_excellent(self) -> bool {
        self.0 & Self::HAS_EXCELLENT != 0
    }

    pub const fn has_ancient(self) -> bool {
        self.0 & Self::HAS_ANCIENT != 0
    }

    pub const fn has_harmony(self) -> bool {
        self.0 & Self::HAS_HARMONY != 0
    }

    pub const fn has_guardian(self) -> bool {
        self.0 & Self::HAS_GUARDIAN != 0
    }

    pub const fn has_sockets(self) -> bool {
        self.0 & Self::HAS_SOCKETS != 0
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ItemPacketError {
    #[error("item packet is too short")]
    TooShort,
    #[error("item packet has trailing bytes")]
    TrailingBytes,
    #[error("item packet has an invalid field: {0}")]
    FieldOutOfRange(&'static str),
    #[error("item packet socket count exceeds the legacy limit: {0}")]
    SocketCountTooLarge(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSize {
    pub width: u8,
    pub height: u8,
}

impl ItemSize {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    pub const fn area(self) -> u16 {
        self.width as u16 * self.height as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemRequirements {
    pub level: u16,
    pub strength: u16,
    pub dexterity: u16,
    pub energy: u16,
    pub vitality: u16,
    pub charisma: u16,
    pub allowed_base_classes: [bool; BASE_CLASS_COUNT],
    pub max_step_by_base_class: [u8; BASE_CLASS_COUNT],
}

impl Default for ItemRequirements {
    fn default() -> Self {
        Self {
            level: 0,
            strength: 0,
            dexterity: 0,
            energy: 0,
            vitality: 0,
            charisma: 0,
            allowed_base_classes: [true; BASE_CLASS_COUNT],
            max_step_by_base_class: [3; BASE_CLASS_COUNT],
        }
    }
}

impl ItemRequirements {
    pub fn for_base_class(class_: CharacterClass) -> Self {
        let mut requirements = Self {
            allowed_base_classes: [false; BASE_CLASS_COUNT],
            max_step_by_base_class: [0; BASE_CLASS_COUNT],
            ..Self::default()
        };

        if let Some(index) = base_class_index(class_) {
            requirements.allowed_base_classes[index] = true;
            requirements.max_step_by_base_class[index] = class_.step_class();
        }

        requirements
    }

    pub fn can_be_used_by(&self, sheet: &CharacterSheet) -> bool {
        if sheet.level < self.level {
            return false;
        }

        let strength = sheet.strength + sheet.add_strength;
        let dexterity = sheet.dexterity + sheet.add_dexterity;
        let energy = sheet.energy + sheet.add_energy;
        let vitality = sheet.vitality + sheet.add_vitality;
        let charisma = sheet.charisma + sheet.add_charisma;

        if strength < self.strength
            || dexterity < self.dexterity
            || energy < self.energy
            || vitality < self.vitality
            || charisma < self.charisma
        {
            return false;
        }

        let base_class = sheet.class_.base_class();
        let Some(index) = base_class_index(base_class) else {
            return false;
        };

        self.allowed_base_classes[index]
            && sheet.class_.step_class() <= self.max_step_by_base_class[index]
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ItemError {
    #[error("item stack is empty")]
    EmptyStack,
    #[error("item stack is not compatible")]
    NotStackable,
    #[error("item stack limit would be exceeded")]
    StackLimitExceeded,
    #[error("item stack does not have enough quantity")]
    InsufficientQuantity,
    #[error("item does not have an equipment slot")]
    MissingEquipmentSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemPacketData {
    pub group: u8,
    pub number: u16,
    pub level: u8,
    pub durability: u8,
    pub flags: ItemOptionFlags,
    pub option_level: u8,
    pub option_type: u8,
    pub excellent_flags: u8,
    pub ancient_discriminator: u8,
    pub ancient_bonus_option: u8,
    pub harmony_option_type: u8,
    pub harmony_option_level: u8,
    pub socket_bonus_option: u8,
    pub socket_count: u8,
    pub socket_options: [u8; MAX_ITEM_SOCKETS],
}

impl Default for ItemPacketData {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl ItemPacketData {
    pub fn new(group: u8, number: u16) -> Self {
        Self {
            group,
            number,
            level: 0,
            durability: 0,
            flags: ItemOptionFlags::default(),
            option_level: 0,
            option_type: 0,
            excellent_flags: 0,
            ancient_discriminator: 0,
            ancient_bonus_option: 0,
            harmony_option_type: 0,
            harmony_option_level: 0,
            socket_bonus_option: 0,
            socket_count: 0,
            socket_options: [SOCKET_EMPTY; MAX_ITEM_SOCKETS],
        }
    }

    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    pub fn with_durability(mut self, durability: u8) -> Self {
        self.durability = durability;
        self
    }

    pub fn with_flag(mut self, flag: u8) -> Self {
        self.flags = self.flags.with(flag);
        self
    }

    pub fn with_option(mut self, option_type: u8, option_level: u8) -> Self {
        self.flags = self.flags.with(ItemOptionFlags::HAS_OPTION);
        self.option_type = option_type & 0x0F;
        self.option_level = option_level & 0x0F;
        self
    }

    pub fn with_excellent_flags(mut self, excellent_flags: u8) -> Self {
        self.flags = self.flags.with(ItemOptionFlags::HAS_EXCELLENT);
        self.excellent_flags = excellent_flags;
        self
    }

    pub fn with_ancient(mut self, ancient_discriminator: u8, ancient_bonus_option: u8) -> Self {
        self.flags = self.flags.with(ItemOptionFlags::HAS_ANCIENT);
        self.ancient_discriminator = ancient_discriminator & 0x0F;
        self.ancient_bonus_option = ancient_bonus_option & 0x0F;
        self
    }

    pub fn with_harmony(mut self, harmony_option_type: u8, harmony_option_level: u8) -> Self {
        self.flags = self.flags.with(ItemOptionFlags::HAS_HARMONY);
        self.harmony_option_type = harmony_option_type & 0x0F;
        self.harmony_option_level = harmony_option_level & 0x0F;
        self
    }

    pub fn with_sockets(
        mut self,
        socket_bonus_option: u8,
        socket_options: impl IntoIterator<Item = u8>,
    ) -> Self {
        self.flags = self.flags.with(ItemOptionFlags::HAS_SOCKETS);
        self.socket_bonus_option = socket_bonus_option & 0x0F;
        self.socket_options = [SOCKET_EMPTY; MAX_ITEM_SOCKETS];

        let sockets: Vec<u8> = socket_options.into_iter().take(MAX_ITEM_SOCKETS).collect();
        self.socket_count = sockets.len() as u8;
        for (index, socket) in sockets.into_iter().enumerate() {
            self.socket_options[index] = socket;
        }

        self
    }

    pub fn type_id(&self) -> u16 {
        u16::from(self.group) * MAX_ITEM_INDEX + self.number
    }

    pub fn can_stack_with(&self, other: &Self) -> bool {
        self.group == other.group
            && self.number == other.number
            && self.level == other.level
            && self.flags == other.flags
            && self.option_level == other.option_level
            && self.option_type == other.option_type
            && self.excellent_flags == other.excellent_flags
            && self.ancient_discriminator == other.ancient_discriminator
            && self.ancient_bonus_option == other.ancient_bonus_option
            && self.harmony_option_type == other.harmony_option_type
            && self.harmony_option_level == other.harmony_option_level
            && self.socket_bonus_option == other.socket_bonus_option
            && self.socket_count == other.socket_count
            && self.socket_options == other.socket_options
    }

    pub fn encoded_len(&self) -> usize {
        let mut len = 5usize;
        if self.flags.has_option() {
            len += 1;
        }
        if self.flags.has_excellent() {
            len += 1;
        }
        if self.flags.has_ancient() {
            len += 1;
        }
        if self.flags.has_harmony() {
            len += 1;
        }
        if self.flags.has_sockets() {
            len += 1 + usize::from(self.socket_count);
        }
        len
    }

    pub fn encode(&self) -> Result<Vec<u8>, ItemPacketError> {
        if self.group > 0x0F {
            return Err(ItemPacketError::FieldOutOfRange("group"));
        }
        if self.number > 0x0FFF {
            return Err(ItemPacketError::FieldOutOfRange("number"));
        }
        if self.socket_count > MAX_ITEM_SOCKETS as u8 {
            return Err(ItemPacketError::SocketCountTooLarge(self.socket_count));
        }

        let mut bytes = Vec::with_capacity(self.encoded_len());
        bytes.push((self.group << 4) | ((self.number >> 8) as u8 & 0x0F));
        bytes.push(self.number as u8);
        bytes.push(self.level);
        bytes.push(self.durability);
        bytes.push(self.flags.bits());

        if self.flags.has_option() {
            bytes.push((self.option_type << 4) | (self.option_level & 0x0F));
        }
        if self.flags.has_excellent() {
            bytes.push(self.excellent_flags);
        }
        if self.flags.has_ancient() {
            bytes.push((self.ancient_bonus_option << 4) | (self.ancient_discriminator & 0x0F));
        }
        if self.flags.has_harmony() {
            bytes.push((self.harmony_option_type << 4) | (self.harmony_option_level & 0x0F));
        }
        if self.flags.has_sockets() {
            bytes.push((self.socket_bonus_option << 4) | (self.socket_count & 0x0F));
            bytes.extend_from_slice(&self.socket_options[..usize::from(self.socket_count)]);
        }

        Ok(bytes)
    }

    pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ItemPacketError> {
        let bytes = bytes.as_ref();
        if bytes.len() < 5 {
            return Err(ItemPacketError::TooShort);
        }

        let group = (bytes[0] >> 4) & 0x0F;
        let number = (u16::from(bytes[0] & 0x0F) << 8) | u16::from(bytes[1]);
        let mut packet = Self::new(group, number)
            .with_level(bytes[2])
            .with_durability(bytes[3]);
        packet.flags = ItemOptionFlags::new(bytes[4]);

        let mut index = 5usize;
        if packet.flags.has_option() {
            let Some(byte) = bytes.get(index) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.option_type = byte >> 4;
            packet.option_level = byte & 0x0F;
            index += 1;
        }
        if packet.flags.has_excellent() {
            let Some(byte) = bytes.get(index) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.excellent_flags = *byte;
            index += 1;
        }
        if packet.flags.has_ancient() {
            let Some(byte) = bytes.get(index) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.ancient_discriminator = byte & 0x0F;
            packet.ancient_bonus_option = byte >> 4;
            index += 1;
        }
        if packet.flags.has_harmony() {
            let Some(byte) = bytes.get(index) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.harmony_option_level = byte & 0x0F;
            packet.harmony_option_type = byte >> 4;
            index += 1;
        }
        if packet.flags.has_sockets() {
            let Some(byte) = bytes.get(index) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.socket_bonus_option = byte >> 4;
            packet.socket_count = byte & 0x0F;
            if packet.socket_count > MAX_ITEM_SOCKETS as u8 {
                return Err(ItemPacketError::SocketCountTooLarge(packet.socket_count));
            }
            index += 1;

            let socket_len = usize::from(packet.socket_count);
            let end = index + socket_len;
            let Some(slice) = bytes.get(index..end) else {
                return Err(ItemPacketError::TooShort);
            };
            packet.socket_options = [SOCKET_EMPTY; MAX_ITEM_SOCKETS];
            packet.socket_options[..socket_len].copy_from_slice(slice);
            index = end;
        }

        if index != bytes.len() {
            return Err(ItemPacketError::TrailingBytes);
        }

        Ok(packet)
    }

    pub fn with_durability_override(&self, durability: u8) -> Self {
        let mut packet = self.clone();
        packet.durability = durability;
        packet
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub packet: ItemPacketData,
    pub size: ItemSize,
    pub stack_limit: u16,
    pub stack_count: u16,
    pub equipment_slot: Option<EquipmentSlot>,
    pub two_handed: bool,
    pub requirements: ItemRequirements,
}

impl Item {
    pub fn new(packet: ItemPacketData, size: ItemSize) -> Self {
        Self {
            packet,
            size,
            stack_limit: 1,
            stack_count: 1,
            equipment_slot: None,
            two_handed: false,
            requirements: ItemRequirements::default(),
        }
    }

    pub fn stackable(
        packet: ItemPacketData,
        size: ItemSize,
        stack_limit: u16,
        stack_count: u16,
    ) -> Self {
        let stack_limit = stack_limit.max(1);
        let stack_count = stack_count.min(stack_limit).max(1);

        Self {
            packet,
            size,
            stack_limit,
            stack_count,
            equipment_slot: None,
            two_handed: false,
            requirements: ItemRequirements::default(),
        }
    }

    pub fn equipment(
        packet: ItemPacketData,
        size: ItemSize,
        slot: EquipmentSlot,
        requirements: ItemRequirements,
        two_handed: bool,
    ) -> Self {
        Self {
            packet,
            size,
            stack_limit: 1,
            stack_count: 1,
            equipment_slot: Some(slot),
            two_handed,
            requirements,
        }
    }

    pub fn type_id(&self) -> u16 {
        self.packet.type_id()
    }

    pub fn is_stackable(&self) -> bool {
        self.stack_limit > 1
    }

    pub fn is_equipment(&self) -> bool {
        self.equipment_slot.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.stack_count == 0
    }

    pub fn sync_packet(&self) -> ItemPacketData {
        let durability = if self.is_stackable() {
            self.stack_count.min(u16::from(u8::MAX)) as u8
        } else {
            self.packet.durability
        };

        self.packet.with_durability_override(durability)
    }

    pub fn sync_bytes(&self) -> Result<Vec<u8>, ItemPacketError> {
        self.sync_packet().encode()
    }

    pub fn can_stack_with(&self, other: &Self) -> bool {
        self.is_stackable()
            && other.is_stackable()
            && self.stack_limit == other.stack_limit
            && self.size == other.size
            && self.packet.can_stack_with(&other.packet)
    }

    pub fn available_stack_space(&self) -> u16 {
        self.stack_limit.saturating_sub(self.stack_count)
    }

    pub fn consume(&mut self, amount: u16) -> Result<u16, ItemError> {
        if amount == 0 {
            return Err(ItemError::InsufficientQuantity);
        }

        if amount > self.stack_count {
            return Err(ItemError::InsufficientQuantity);
        }

        self.stack_count -= amount;
        Ok(self.stack_count)
    }

    pub fn merge_stack(&mut self, other: &mut Self) -> Result<u16, ItemError> {
        if !self.can_stack_with(other) {
            return Err(ItemError::NotStackable);
        }

        if self.is_empty() || other.is_empty() {
            return Err(ItemError::EmptyStack);
        }

        let moved = self.available_stack_space().min(other.stack_count);
        if moved == 0 {
            return Err(ItemError::StackLimitExceeded);
        }

        self.stack_count += moved;
        other.stack_count -= moved;
        Ok(moved)
    }

    pub fn can_be_used_by(&self, sheet: &CharacterSheet) -> bool {
        self.requirements.can_be_used_by(sheet)
    }

    pub fn with_equipment_slot(mut self, slot: EquipmentSlot) -> Self {
        self.equipment_slot = Some(slot);
        self
    }

    pub fn with_requirements(mut self, requirements: ItemRequirements) -> Self {
        self.requirements = requirements;
        self
    }

    pub fn with_two_handed(mut self, two_handed: bool) -> Self {
        self.two_handed = two_handed;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GridState {
    width: usize,
    height: usize,
    items: Vec<Option<Item>>,
    anchors: Vec<Option<usize>>,
}

#[allow(dead_code)]
impl GridState {
    pub fn new(width: usize, height: usize) -> Self {
        let slot_count = width * height;
        Self {
            width,
            height,
            items: vec![None; slot_count],
            anchors: vec![None; slot_count],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn slot_count(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.iter().all(Option::is_none)
    }

    pub fn item_at(&self, index: usize) -> Option<&Item> {
        let anchor = *self.anchors.get(index)?;
        let anchor = anchor?;
        self.items.get(anchor)?.as_ref()
    }

    pub fn occupied_slots(&self) -> usize {
        self.items.iter().filter(|item| item.is_some()).count()
    }

    pub fn can_place(&self, index: usize, item: &Item) -> bool {
        if item.is_empty() {
            return false;
        }

        let Some((x, y)) = self.coordinates(index) else {
            return false;
        };

        let width = usize::from(item.size.width);
        let height = usize::from(item.size.height);

        if width == 0 || height == 0 || x + width > self.width || y + height > self.height {
            return false;
        }

        for dy in 0..height {
            for dx in 0..width {
                let slot = self.offset(x + dx, y + dy);
                if self.anchors[slot].is_some() {
                    return false;
                }
            }
        }

        true
    }

    pub fn find_first_fit(&self, item: &Item) -> Option<usize> {
        (0..self.slot_count()).find(|&index| self.can_place(index, item))
    }

    pub fn place(&mut self, index: usize, item: Item) -> Result<(), GridError> {
        if !self.can_place(index, &item) {
            return Err(GridError::ItemDoesNotFit);
        }

        let (x, y) = self.coordinates(index).ok_or(GridError::SlotOutOfBounds)?;
        let width = usize::from(item.size.width);
        let height = usize::from(item.size.height);

        self.items[index] = Some(item);
        for dy in 0..height {
            for dx in 0..width {
                let slot = self.offset(x + dx, y + dy);
                self.anchors[slot] = Some(index);
            }
        }

        Ok(())
    }

    pub fn take(&mut self, index: usize) -> Result<Item, GridError> {
        let Some(anchor) = self.anchors.get(index).copied().flatten() else {
            return Err(GridError::SlotEmpty);
        };

        let Some(item) = self.items[anchor].take() else {
            return Err(GridError::MissingAnchor);
        };

        self.clear_footprint(anchor, &item);
        Ok(item)
    }

    pub fn move_item(&mut self, from: usize, to: usize) -> Result<(), GridError> {
        if from == to {
            return Ok(());
        }

        let item = self.take(from)?;
        match self.place(to, item.clone()) {
            Ok(()) => Ok(()),
            Err(error) => {
                let _ = self.place(from, item);
                Err(error)
            }
        }
    }

    pub fn clear(&mut self) {
        self.items.fill(None);
        self.anchors.fill(None);
    }

    fn clear_footprint(&mut self, anchor: usize, item: &Item) {
        let Some((x, y)) = self.coordinates(anchor) else {
            return;
        };

        for dy in 0..usize::from(item.size.height) {
            for dx in 0..usize::from(item.size.width) {
                let slot = self.offset(x + dx, y + dy);
                self.anchors[slot] = None;
            }
        }
    }

    fn coordinates(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.slot_count() {
            return None;
        }

        Some((index % self.width, index / self.width))
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GridError {
    SlotOutOfBounds,
    SlotEmpty,
    MissingAnchor,
    ItemDoesNotFit,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, _app: &mut App) {}
}

fn base_class_index(class_: CharacterClass) -> Option<usize> {
    match class_.base_class() {
        CharacterClass::Wizard => Some(0),
        CharacterClass::Knight => Some(1),
        CharacterClass::Elf => Some(2),
        CharacterClass::MagicGladiator => Some(3),
        CharacterClass::DarkLord => Some(4),
        CharacterClass::Summoner => Some(5),
        CharacterClass::RageFighter => Some(6),
        CharacterClass::Undefined => None,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GridState, Item, ItemOptionFlags, ItemPacketData, ItemPacketError, ItemRequirements,
        ItemSize, MAX_ITEM_SOCKETS,
    };
    use crate::characters::CharacterSheet;
    use crate::classes::CharacterClass;
    use crate::stats::BASE_CLASS_COUNT;
    use mu_network::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::{decode_packet, encode_packet};
    use proptest::prelude::*;
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpStream;

    fn packet_strategy() -> impl Strategy<Value = ItemPacketData> {
        prop_compose! {
            fn packet_strategy_inner()(
                group in 0u8..16,
                number in 0u16..0x0FFF,
                level in any::<u8>(),
                durability in any::<u8>(),
                has_option in any::<bool>(),
                has_luck in any::<bool>(),
                has_skill in any::<bool>(),
                option_type in 0u8..16,
                option_level in 0u8..16,
                has_excellent in any::<bool>(),
                excellent_flags in any::<u8>(),
                has_ancient in any::<bool>(),
                ancient_discriminator in 0u8..16,
                ancient_bonus_option in 0u8..16,
                has_harmony in any::<bool>(),
                harmony_option_type in 0u8..16,
                harmony_option_level in 0u8..16,
                has_guardian in any::<bool>(),
                socket_bonus_option in 0u8..16,
                socket_options in prop::collection::vec(any::<u8>(), 0..=MAX_ITEM_SOCKETS),
            ) -> ItemPacketData {
                let mut packet = ItemPacketData::new(group, number)
                    .with_level(level)
                    .with_durability(durability);

                if has_option {
                    packet = packet.with_option(option_type, option_level);
                }
                if has_luck {
                    packet = packet.with_flag(ItemOptionFlags::HAS_LUCK);
                }
                if has_skill {
                    packet = packet.with_flag(ItemOptionFlags::HAS_SKILL);
                }
                if has_excellent {
                    packet = packet.with_excellent_flags(excellent_flags);
                }
                if has_ancient {
                    packet = packet.with_ancient(ancient_discriminator, ancient_bonus_option);
                }
                if has_harmony {
                    packet = packet.with_harmony(harmony_option_type, harmony_option_level);
                }
                if has_guardian {
                    packet = packet.with_flag(ItemOptionFlags::HAS_GUARDIAN);
                }

                let socket_count = socket_options.len().min(MAX_ITEM_SOCKETS);
                if socket_count > 0 {
                    packet = packet.with_sockets(
                        socket_bonus_option,
                        socket_options.into_iter().take(socket_count),
                    );
                }

                packet
            }
        }

        packet_strategy_inner()
    }

    #[test]
    fn encodes_and_decodes_item_packets() {
        let packet = ItemPacketData::new(3, 17)
            .with_level(9)
            .with_durability(88)
            .with_option(4, 7)
            .with_excellent_flags(0x33)
            .with_ancient(2, 5)
            .with_harmony(6, 1)
            .with_sockets(3, [0x11, 0x22, 0x33]);

        let bytes = packet.encode().unwrap();
        let decoded = ItemPacketData::decode(&bytes).unwrap();

        assert_eq!(decoded, packet);
    }

    proptest! {
        #[test]
        fn item_packet_roundtrips(packet in packet_strategy()) {
            let bytes = packet.encode().unwrap();
            let decoded = ItemPacketData::decode(&bytes).unwrap();
            assert_eq!(decoded, packet);
        }
    }

    #[tokio::test]
    async fn fake_server_roundtrips_the_item_packet_payload() {
        let packet = ItemPacketData::new(2, 15)
            .with_level(5)
            .with_durability(42)
            .with_option(1, 9)
            .with_sockets(2, [0xFE, 0x12]);
        let payload = packet.encode().unwrap();
        let frame = encode_packet(0xC1, 0x8A, 0x00, &payload).unwrap();

        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(ConnectionScript::new().send_packet(frame).close()),
        )
        .await
        .unwrap();

        let mut client = TcpStream::connect(server.address()).await.unwrap();
        let mut buffer = vec![0u8; payload.len() + 4];
        client.read_exact(&mut buffer).await.unwrap();
        let frame = decode_packet(&buffer).unwrap();

        assert_eq!(frame.code, 0xC1);
        assert_eq!(frame.headcode, 0x8A);
        assert_eq!(ItemPacketData::decode(frame.payload).unwrap(), packet);

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn fake_server_surfaces_item_packet_sync_errors() {
        let mut packet = ItemPacketData::new(1, 1).with_sockets(1, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
        packet.flags = packet.flags.with(ItemOptionFlags::HAS_SOCKETS);
        packet.socket_count = 6;
        let mut payload = packet.encode().unwrap_or_else(|_| {
            let mut payload = vec![0x10, 0x01, 0x00, 0x2A, ItemOptionFlags::HAS_SOCKETS];
            payload.push((1 << 4) | 6);
            payload.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
            payload
        });
        if payload.len() < 12 {
            payload.resize(12, 0);
        }
        let frame = encode_packet(0xC1, 0x8A, 0x00, &payload).unwrap();

        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(ConnectionScript::new().send_packet(frame).close()),
        )
        .await
        .unwrap();

        let mut client = TcpStream::connect(server.address()).await.unwrap();
        let mut buffer = vec![0u8; payload.len() + 4];
        client.read_exact(&mut buffer).await.unwrap();
        let frame = decode_packet(&buffer).unwrap();

        assert!(matches!(
            ItemPacketData::decode(frame.payload),
            Err(ItemPacketError::SocketCountTooLarge(6))
        ));

        server.finish().await.unwrap();
    }

    #[test]
    fn grid_state_places_and_moves_items() {
        let mut grid = GridState::new(8, 8);
        let item = Item::new(ItemPacketData::new(1, 1), ItemSize::new(2, 2));

        assert!(grid.place(0, item.clone()).is_ok());
        assert!(grid.item_at(0).is_some());
        assert!(grid.place(1, item.clone()).is_err());
        assert!(grid.move_item(0, 16).is_ok());
        assert!(grid.item_at(16).is_some());
    }

    #[test]
    fn item_requirements_match_character_sheets() {
        let mut requirements = ItemRequirements {
            level: 10,
            strength: 50,
            allowed_base_classes: [false; BASE_CLASS_COUNT],
            ..ItemRequirements::default()
        };
        requirements.allowed_base_classes[1] = true;

        let sheet = CharacterSheet::new(CharacterClass::Knight);
        assert!(!requirements.can_be_used_by(&sheet));

        let mut sheet = sheet;
        sheet.level = 10;
        sheet.strength = 50;
        assert!(requirements.can_be_used_by(&sheet));
    }
}
