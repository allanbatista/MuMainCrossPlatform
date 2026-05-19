use bevy::prelude::{App, Plugin, Resource};
use mu_gameplay::{EquipmentSlot, Item, ItemPacketData, ItemRequirements, ItemSize};
use std::fmt::Write as _;

const EMPTY_ACTIONS: &[ItemEditorAction] = &[ItemEditorAction::Close];
const ACTIVE_ACTIONS: &[ItemEditorAction] = &[
    ItemEditorAction::Save,
    ItemEditorAction::ExportLegacy,
    ItemEditorAction::ExportCsv,
    ItemEditorAction::Cancel,
    ItemEditorAction::Close,
];
const CSV_HEADER: &str = "index,label,type_id,group,number,size_width,size_height,level,durability,stack_limit,stack_count,equipment_slot,two_handed,requirement_level,requirement_strength,requirement_dexterity,requirement_energy,requirement_vitality,requirement_charisma";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemEditorAction {
    Save,
    ExportLegacy,
    ExportCsv,
    Cancel,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemEditorColumn {
    Index,
    Label,
    TypeId,
    Level,
    Durability,
    Stack,
    EquipmentSlot,
    Requirements,
    TwoHanded,
}

impl ItemEditorColumn {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Index => "index",
            Self::Label => "label",
            Self::TypeId => "type_id",
            Self::Level => "level",
            Self::Durability => "durability",
            Self::Stack => "stack",
            Self::EquipmentSlot => "equipment_slot",
            Self::Requirements => "requirements",
            Self::TwoHanded => "two_handed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemEditorColumnVisibility {
    pub index: bool,
    pub label: bool,
    pub type_id: bool,
    pub level: bool,
    pub durability: bool,
    pub stack: bool,
    pub equipment_slot: bool,
    pub requirements: bool,
    pub two_handed: bool,
}

impl Default for ItemEditorColumnVisibility {
    fn default() -> Self {
        Self {
            index: true,
            label: true,
            type_id: true,
            level: true,
            durability: false,
            stack: true,
            equipment_slot: false,
            requirements: false,
            two_handed: false,
        }
    }
}

impl ItemEditorColumnVisibility {
    pub fn is_visible(&self, column: ItemEditorColumn) -> bool {
        match column {
            ItemEditorColumn::Index => self.index,
            ItemEditorColumn::Label => self.label,
            ItemEditorColumn::TypeId => self.type_id,
            ItemEditorColumn::Level => self.level,
            ItemEditorColumn::Durability => self.durability,
            ItemEditorColumn::Stack => self.stack,
            ItemEditorColumn::EquipmentSlot => self.equipment_slot,
            ItemEditorColumn::Requirements => self.requirements,
            ItemEditorColumn::TwoHanded => self.two_handed,
        }
    }

    pub fn set_visible(&mut self, column: ItemEditorColumn, visible: bool) {
        match column {
            ItemEditorColumn::Index => self.index = visible,
            ItemEditorColumn::Label => self.label = visible,
            ItemEditorColumn::TypeId => self.type_id = visible,
            ItemEditorColumn::Level => self.level = visible,
            ItemEditorColumn::Durability => self.durability = visible,
            ItemEditorColumn::Stack => self.stack = visible,
            ItemEditorColumn::EquipmentSlot => self.equipment_slot = visible,
            ItemEditorColumn::Requirements => self.requirements = visible,
            ItemEditorColumn::TwoHanded => self.two_handed = visible,
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "index={}|label={}|type_id={}|level={}|durability={}|stack={}|equipment_slot={}|requirements={}|two_handed={}",
            self.index,
            self.label,
            self.type_id,
            self.level,
            self.durability,
            self.stack,
            self.equipment_slot,
            self.requirements,
            self.two_handed,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemEditorRow {
    pub index: usize,
    pub label: String,
    pub item: Item,
}

impl ItemEditorRow {
    pub fn new(index: usize, label: impl Into<String>, item: Item) -> Self {
        Self {
            index,
            label: label.into(),
            item,
        }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.trim();
        if query.is_empty() {
            return true;
        }

        let query = query.to_lowercase();
        self.label.to_lowercase().contains(&query)
            || self.snapshot().to_lowercase().contains(&query)
    }

    pub fn snapshot(&self) -> String {
        let requirements = &self.item.requirements;
        format!(
            "index={}|label={}|type_id={}|packet={}:{}|size={}x{}|level={}|durability={}|stack={}/{}|slot={}|two_handed={}|requirements=level={}|str={}|dex={}|ene={}|vit={}|cha={}",
            self.index,
            self.label,
            self.item.type_id(),
            self.item.packet.group,
            self.item.packet.number,
            self.item.size.width,
            self.item.size.height,
            self.item.packet.level,
            self.item.packet.durability,
            self.item.stack_count,
            self.item.stack_limit,
            item_slot_label(self.item.equipment_slot),
            self.item.two_handed,
            requirements.level,
            requirements.strength,
            requirements.dexterity,
            requirements.energy,
            requirements.vitality,
            requirements.charisma,
        )
    }

    pub fn export_readable(&self) -> String {
        let requirements = &self.item.requirements;
        format!(
            "Row {}\nIndex = {}\nLabel = {}\nTypeId = {}\nPacket = {}:{}\nSize = {}x{}\nLevel = {}\nDurability = {}\nStackLimit = {}\nStackCount = {}\nEquipmentSlot = {}\nTwoHanded = {}\nRequirementLevel = {}\nRequirementStrength = {}\nRequirementDexterity = {}\nRequirementEnergy = {}\nRequirementVitality = {}\nRequirementCharisma = {}",
            self.index,
            self.index,
            self.label,
            self.item.type_id(),
            self.item.packet.group,
            self.item.packet.number,
            self.item.size.width,
            self.item.size.height,
            self.item.packet.level,
            self.item.packet.durability,
            self.item.stack_limit,
            self.item.stack_count,
            item_slot_label(self.item.equipment_slot),
            self.item.two_handed,
            requirements.level,
            requirements.strength,
            requirements.dexterity,
            requirements.energy,
            requirements.vitality,
            requirements.charisma,
        )
    }

    pub fn export_csv(&self) -> String {
        let requirements = &self.item.requirements;
        [
            self.index.to_string(),
            csv_escape(&self.label),
            self.item.type_id().to_string(),
            self.item.packet.group.to_string(),
            self.item.packet.number.to_string(),
            self.item.size.width.to_string(),
            self.item.size.height.to_string(),
            self.item.packet.level.to_string(),
            self.item.packet.durability.to_string(),
            self.item.stack_limit.to_string(),
            self.item.stack_count.to_string(),
            csv_escape(item_slot_label(self.item.equipment_slot)),
            self.item.two_handed.to_string(),
            requirements.level.to_string(),
            requirements.strength.to_string(),
            requirements.dexterity.to_string(),
            requirements.energy.to_string(),
            requirements.vitality.to_string(),
            requirements.charisma.to_string(),
        ]
        .join(",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemEditorWorkflowState {
    Empty,
    Ready,
    Dirty,
    Saved,
    Cancelled,
    Failed { message: String },
}

impl ItemEditorWorkflowState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Ready => "ready",
            Self::Dirty => "dirty",
            Self::Saved => "saved",
            Self::Cancelled => "cancelled",
            Self::Failed { .. } => "failed",
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Failed { message } => Some(message.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemEditorError {
    NoRowsLoaded,
    NoSelectedRow,
    SelectedRowMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemEditorState {
    pub rows: Vec<ItemEditorRow>,
    baseline_rows: Vec<ItemEditorRow>,
    pub selected_row: Option<usize>,
    pub search_query: String,
    pub freeze_columns: bool,
    pub column_visibility: ItemEditorColumnVisibility,
    pub audit_log: Vec<String>,
    pub workflow_state: ItemEditorWorkflowState,
}

impl Default for ItemEditorState {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            baseline_rows: Vec::new(),
            selected_row: None,
            search_query: String::new(),
            freeze_columns: false,
            column_visibility: ItemEditorColumnVisibility::default(),
            audit_log: Vec::new(),
            workflow_state: ItemEditorWorkflowState::Empty,
        }
    }
}

impl Resource for ItemEditorState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct ItemEditorPlugin;

impl Plugin for ItemEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemEditorState>();
    }
}

impl ItemEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rows(rows: Vec<ItemEditorRow>) -> Self {
        let mut state = Self::default();
        state.set_rows(rows);
        state
    }

    pub fn sample() -> Self {
        Self::with_rows(sample_rows())
    }

    pub fn set_rows(&mut self, rows: Vec<ItemEditorRow>) {
        self.rows = rows.clone();
        self.baseline_rows = rows;
        self.selected_row = self.rows.first().map(|row| row.index);
        self.workflow_state = if self.rows.is_empty() {
            ItemEditorWorkflowState::Empty
        } else {
            ItemEditorWorkflowState::Ready
        };
        self.audit_log.clear();
        if !self.rows.is_empty() {
            self.audit_log
                .push(format!("Loaded {} item row(s)", self.rows.len()));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        matches!(self.workflow_state, ItemEditorWorkflowState::Dirty)
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn filtered_row_count(&self) -> usize {
        self.filtered_rows().len()
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn toggle_freeze_columns(&mut self) {
        self.freeze_columns = !self.freeze_columns;
    }

    pub fn set_freeze_columns(&mut self, freeze: bool) {
        self.freeze_columns = freeze;
    }

    pub fn set_column_visible(&mut self, column: ItemEditorColumn, visible: bool) {
        self.column_visibility.set_visible(column, visible);
    }

    pub fn select_row(&mut self, index: usize) -> bool {
        if self.rows.iter().any(|row| row.index == index) {
            self.selected_row = Some(index);
            return true;
        }

        false
    }

    pub fn selected_row(&self) -> Option<&ItemEditorRow> {
        let selected_row = self.selected_row?;
        self.rows.iter().find(|row| row.index == selected_row)
    }

    pub fn selected_row_mut(&mut self) -> Option<&mut ItemEditorRow> {
        let selected_row = self.selected_row?;
        self.rows.iter_mut().find(|row| row.index == selected_row)
    }

    pub fn update_selected_row(
        &mut self,
        description: impl Into<String>,
        mut update: impl FnMut(&mut ItemEditorRow),
    ) -> Result<(), ItemEditorError> {
        if self.rows.is_empty() {
            return Err(ItemEditorError::NoRowsLoaded);
        }

        let Some(row) = self.selected_row_mut() else {
            return Err(ItemEditorError::NoSelectedRow);
        };

        update(row);
        self.workflow_state = ItemEditorWorkflowState::Dirty;
        self.audit_log.push(description.into());
        Ok(())
    }

    pub fn rename_selected_row(&mut self, label: impl Into<String>) -> Result<(), ItemEditorError> {
        let label = label.into();
        self.update_selected_row(
            format!("Renamed selected item row to {}", label),
            move |row| {
                row.label = label.clone();
            },
        )
    }

    pub fn mark_failed(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.workflow_state = ItemEditorWorkflowState::Failed {
            message: message.clone(),
        };
        self.audit_log
            .push(format!("Item editor failed: {}", message));
    }

    pub fn save(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let changed_rows = self.changed_row_count();
        let message = if changed_rows == 0 {
            format!("No item editor changes to save for {}", target_path)
        } else {
            format!("Saved {} item row(s) to {}", changed_rows, target_path)
        };

        self.baseline_rows = self.rows.clone();
        self.workflow_state = if self.rows.is_empty() {
            ItemEditorWorkflowState::Empty
        } else {
            ItemEditorWorkflowState::Saved
        };
        self.audit_log.push(message.clone());
        message
    }

    pub fn cancel(&mut self) -> String {
        let changed_rows = self.changed_row_count();
        let message = if changed_rows == 0 {
            "No pending item editor changes to cancel".to_string()
        } else {
            "Cancelled pending item editor changes".to_string()
        };

        self.rows = self.baseline_rows.clone();
        self.selected_row = self.rows.first().map(|row| row.index);
        self.workflow_state = if self.rows.is_empty() {
            ItemEditorWorkflowState::Empty
        } else {
            ItemEditorWorkflowState::Cancelled
        };
        self.audit_log.push(message.clone());
        message
    }

    pub fn export_legacy(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let text = self.export_legacy_text();
        self.audit_log.push(format!(
            "Exported item editor legacy data to {}",
            target_path
        ));
        text
    }

    pub fn export_csv(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let text = self.export_csv_text();
        self.audit_log
            .push(format!("Exported item editor CSV to {}", target_path));
        text
    }

    pub fn export_legacy_text(&self) -> String {
        if self.rows.is_empty() {
            return "No item rows loaded".to_string();
        }

        let mut output = String::new();
        for row in &self.rows {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&row.export_readable());
        }
        output
    }

    pub fn export_csv_text(&self) -> String {
        if self.rows.is_empty() {
            return format!("{CSV_HEADER}\n");
        }

        let mut output = String::new();
        output.push_str(CSV_HEADER);
        output.push('\n');
        for row in &self.rows {
            let _ = writeln!(&mut output, "{}", row.export_csv());
        }
        output
    }

    pub fn toolbar_actions(&self) -> &'static [ItemEditorAction] {
        if self.rows.is_empty() {
            EMPTY_ACTIONS
        } else {
            ACTIVE_ACTIONS
        }
    }

    pub fn filtered_rows(&self) -> Vec<&ItemEditorRow> {
        self.rows
            .iter()
            .filter(|row| row.matches_query(&self.search_query))
            .collect()
    }

    pub fn filtered_row_summaries(&self) -> Vec<String> {
        self.filtered_rows()
            .into_iter()
            .map(ItemEditorRow::snapshot)
            .collect()
    }

    pub fn selected_row_summary(&self) -> Option<String> {
        self.selected_row().map(ItemEditorRow::snapshot)
    }

    pub fn audit_log_tail(&self) -> Vec<String> {
        let tail_start = self.audit_log.len().saturating_sub(3);
        self.audit_log[tail_start..].to_vec()
    }

    pub fn snapshot(&self) -> String {
        format!(
            "state={}|rows={}|filtered_rows={}|selected_row={:?}|search_query={:?}|freeze_columns={}|columns={}|actions={:?}|audit_log_tail={:?}|selected_row_snapshot={:?}",
            self.workflow_state.as_str(),
            self.row_count(),
            self.filtered_row_count(),
            self.selected_row,
            self.search_query,
            self.freeze_columns,
            self.column_visibility.snapshot(),
            self.toolbar_actions(),
            self.audit_log_tail(),
            self.selected_row_summary(),
        )
    }

    fn changed_row_count(&self) -> usize {
        let max_len = self.rows.len().max(self.baseline_rows.len());
        let mut changed = 0usize;

        for index in 0..max_len {
            if self.rows.get(index) != self.baseline_rows.get(index) {
                changed += 1;
            }
        }

        changed
    }
}

fn sample_rows() -> Vec<ItemEditorRow> {
    let first_requirements = ItemRequirements {
        level: 10,
        strength: 20,
        ..ItemRequirements::default()
    };

    let second_requirements = ItemRequirements {
        level: 5,
        energy: 15,
        ..ItemRequirements::default()
    };

    vec![
        ItemEditorRow::new(
            0,
            "Short Sword",
            Item::equipment(
                ItemPacketData::new(0, 1).with_level(0).with_durability(255),
                ItemSize::new(1, 2),
                EquipmentSlot::WeaponRight,
                first_requirements,
                false,
            ),
        ),
        ItemEditorRow::new(
            1,
            "Mana Potion",
            Item::stackable(
                ItemPacketData::new(14, 15).with_durability(12),
                ItemSize::new(1, 1),
                10,
                4,
            )
            .with_requirements(second_requirements),
        ),
    ]
}

fn item_slot_label(slot: Option<EquipmentSlot>) -> &'static str {
    match slot {
        Some(EquipmentSlot::WeaponRight) => "WeaponRight",
        Some(EquipmentSlot::WeaponLeft) => "WeaponLeft",
        Some(EquipmentSlot::Helm) => "Helm",
        Some(EquipmentSlot::Armor) => "Armor",
        Some(EquipmentSlot::Pants) => "Pants",
        Some(EquipmentSlot::Gloves) => "Gloves",
        Some(EquipmentSlot::Boots) => "Boots",
        Some(EquipmentSlot::Wing) => "Wing",
        Some(EquipmentSlot::Helper) => "Helper",
        Some(EquipmentSlot::Amulet) => "Amulet",
        Some(EquipmentSlot::RingRight) => "RingRight",
        Some(EquipmentSlot::RingLeft) => "RingLeft",
        None => "None",
    }
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        let escaped = value.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        csv_escape, ItemEditorAction, ItemEditorColumn, ItemEditorColumnVisibility,
        ItemEditorPlugin, ItemEditorState, ItemEditorWorkflowState,
    };
    use bevy::prelude::App;

    #[test]
    fn item_editor_state_tracks_edit_save_cancel_and_failure() {
        let mut state = ItemEditorState::sample();

        assert_eq!(state.workflow_state, ItemEditorWorkflowState::Ready);
        assert_eq!(state.row_count(), 2);
        assert_eq!(
            state.toolbar_actions(),
            ItemEditorState::sample().toolbar_actions()
        );
        assert!(state.select_row(1));
        state.set_search_query("potion");
        state.toggle_freeze_columns();
        state
            .rename_selected_row("Greater Mana Potion")
            .expect("row rename should work");
        assert!(state.is_dirty());
        assert_eq!(state.filtered_row_count(), 1);
        assert_eq!(
            state.selected_row_summary().as_deref(),
            Some("index=1|label=Greater Mana Potion|type_id=7183|packet=14:15|size=1x1|level=0|durability=12|stack=4/10|slot=None|two_handed=false|requirements=level=5|str=0|dex=0|ene=15|vit=0|cha=0")
        );

        let save_message = state.save("Data/Local/en/Item_en.bmd");
        assert_eq!(
            save_message,
            "Saved 1 item row(s) to Data/Local/en/Item_en.bmd"
        );
        assert_eq!(state.workflow_state, ItemEditorWorkflowState::Saved);

        state
            .update_selected_row("Adjusted stack count", |row| row.item.stack_count = 6)
            .expect("row edit should work");
        assert_eq!(state.workflow_state, ItemEditorWorkflowState::Dirty);

        let cancel_message = state.cancel();
        assert_eq!(cancel_message, "Cancelled pending item editor changes");
        assert_eq!(state.workflow_state, ItemEditorWorkflowState::Cancelled);
        assert_eq!(
            state.selected_row_summary().as_deref(),
            Some("index=0|label=Short Sword|type_id=1|packet=0:1|size=1x2|level=0|durability=255|stack=1/1|slot=WeaponRight|two_handed=false|requirements=level=10|str=20|dex=0|ene=0|vit=0|cha=0")
        );

        state.mark_failed("disk full");
        assert_eq!(
            state.workflow_state,
            ItemEditorWorkflowState::Failed {
                message: "disk full".to_string(),
            }
        );
        assert_eq!(state.workflow_state.message(), Some("disk full"));
        assert!(state.snapshot().contains("state=failed"));
    }

    #[test]
    fn item_editor_exports_csv_and_legacy_text() {
        let mut state = ItemEditorState::sample();

        let csv = state.export_csv("Data/Local/en/Item.csv");
        let legacy = state.export_legacy("Data/Local/en/Item.bmd");

        assert!(csv.starts_with(
            "index,label,type_id,group,number,size_width,size_height,level,durability,stack_limit,stack_count,equipment_slot,two_handed,requirement_level,requirement_strength,requirement_dexterity,requirement_energy,requirement_vitality,requirement_charisma"
        ));
        assert!(csv.contains("Short Sword"));
        assert!(csv.contains("Mana Potion"));
        assert!(legacy.contains("Row 0"));
        assert!(legacy.contains("Label = Short Sword"));
        assert!(legacy.contains("Label = Mana Potion"));
        assert!(state
            .audit_log
            .iter()
            .any(|entry| entry.contains("Exported item editor CSV")));
        assert!(state
            .audit_log
            .iter()
            .any(|entry| entry.contains("Exported item editor legacy data")));
    }

    #[test]
    fn item_editor_column_visibility_snapshots_and_plugin_registers_resource() {
        let visibility = ItemEditorColumnVisibility::default();

        assert!(visibility.is_visible(ItemEditorColumn::Index));
        assert_eq!(
            visibility.snapshot(),
            "index=true|label=true|type_id=true|level=true|durability=false|stack=true|equipment_slot=false|requirements=false|two_handed=false"
        );
        assert_eq!(csv_escape("Mana Potion"), "Mana Potion");
        assert_eq!(csv_escape("Mana, Potion"), "\"Mana, Potion\"");

        let mut app = App::new();
        app.add_plugins(ItemEditorPlugin);

        let state = app.world().resource::<ItemEditorState>();

        assert!(state.rows.is_empty());
        assert_eq!(state.workflow_state, ItemEditorWorkflowState::Empty);
        assert_eq!(state.toolbar_actions(), &[ItemEditorAction::Close]);
        assert_eq!(state.selected_row, None);
    }
}
