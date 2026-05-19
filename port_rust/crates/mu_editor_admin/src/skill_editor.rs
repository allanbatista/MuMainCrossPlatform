use bevy::prelude::{App, Plugin, Resource};
use mu_gameplay::{SkillCatalog, SkillDefinition, SkillId, MAX_SKILLS};
use std::collections::BTreeMap;
use std::fmt::Write as _;

const EMPTY_ACTIONS: &[SkillEditorAction] = &[SkillEditorAction::Close];
const ACTIVE_ACTIONS: &[SkillEditorAction] = &[
    SkillEditorAction::Save,
    SkillEditorAction::ExportLegacy,
    SkillEditorAction::ExportCsv,
    SkillEditorAction::Cancel,
    SkillEditorAction::Close,
];
const CSV_HEADER: &str = concat!(
    "index,name,level,damage,mana,ability_gauge,distance,delay_ms,energy,charisma,",
    "mastery_type,skill_use_type,skill_brand,kill_count,require_duty_class_0,",
    "require_duty_class_1,require_duty_class_2,require_class_0,require_class_1,",
    "require_class_2,require_class_3,require_class_4,require_class_5,require_class_6,",
    "skill_rank,magic_icon,type_skill,strength,dexterity,item_skill,is_damage,effect"
);
const SKILL_COLUMN_ORDER: &[&str] = &[
    "index",
    "name",
    "level",
    "damage",
    "mana",
    "ability_gauge",
    "distance",
    "delay_ms",
    "energy",
    "charisma",
    "mastery_type",
    "skill_use_type",
    "skill_brand",
    "kill_count",
    "require_duty_class_0",
    "require_duty_class_1",
    "require_duty_class_2",
    "require_class_0",
    "require_class_1",
    "require_class_2",
    "require_class_3",
    "require_class_4",
    "require_class_5",
    "require_class_6",
    "skill_rank",
    "magic_icon",
    "type_skill",
    "strength",
    "dexterity",
    "item_skill",
    "is_damage",
    "effect",
];
const SKILL_DEFAULT_VISIBLE_COLUMNS: &[&str] = &[
    "index",
    "name",
    "level",
    "damage",
    "mana",
    "distance",
    "delay_ms",
    "energy",
    "strength",
    "dexterity",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEditorAction {
    Save,
    ExportLegacy,
    ExportCsv,
    Cancel,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillEditorColumnVisibility {
    columns: BTreeMap<String, bool>,
}

impl Default for SkillEditorColumnVisibility {
    fn default() -> Self {
        let mut columns = BTreeMap::new();

        for &column in SKILL_COLUMN_ORDER {
            columns.insert(
                column.to_string(),
                SKILL_DEFAULT_VISIBLE_COLUMNS.contains(&column),
            );
        }

        Self { columns }
    }
}

impl SkillEditorColumnVisibility {
    pub fn is_visible(&self, column: &str) -> bool {
        self.columns.get(column).copied().unwrap_or(false)
    }

    pub fn set_visible(&mut self, column: impl Into<String>, visible: bool) {
        self.columns.insert(column.into(), visible);
    }

    pub fn snapshot(&self) -> String {
        let mut parts = Vec::with_capacity(self.columns.len());

        for &column in SKILL_COLUMN_ORDER {
            parts.push(format!("{}={}", column, self.is_visible(column)));
        }

        for column in self
            .columns
            .keys()
            .filter(|column| !SKILL_COLUMN_ORDER.contains(&column.as_str()))
        {
            parts.push(format!("{}={}", column, self.is_visible(column)));
        }

        parts.join("|")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillEditorRow {
    pub index: SkillId,
    pub definition: SkillDefinition,
}

impl SkillEditorRow {
    pub fn new(index: SkillId, definition: SkillDefinition) -> Self {
        Self { index, definition }
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.trim();
        if query.is_empty() {
            return true;
        }

        let query = query.to_lowercase();
        self.index.to_string().contains(&query)
            || self.definition.name.to_lowercase().contains(&query)
            || self.snapshot().to_lowercase().contains(&query)
    }

    pub fn snapshot(&self) -> String {
        let definition = &self.definition;
        format!(
            "index={}|name={}|level={}|damage={}|mana={}|ability_gauge={}|distance={}|delay_ms={}|energy={}|charisma={}|mastery_type={}|skill_use_type={}|skill_brand={}|kill_count={}|require_duty_class=[{}]|require_class=[{}]|skill_rank={}|magic_icon={}|type_skill={}|strength={}|dexterity={}|item_skill={}|is_damage={}|effect={}",
            self.index,
            definition.name,
            definition.level,
            definition.damage,
            definition.mana,
            definition.ability_gauge,
            definition.distance,
            definition.delay_ms,
            definition.energy,
            definition.charisma,
            definition.mastery_type,
            definition.skill_use_type,
            definition.skill_brand,
            definition.kill_count,
            format_u8_array(&definition.require_duty_class),
            format_u8_array(&definition.require_class),
            definition.skill_rank,
            definition.magic_icon,
            definition.type_skill,
            definition.strength,
            definition.dexterity,
            definition.item_skill,
            definition.is_damage,
            definition.effect,
        )
    }

    pub fn export_readable(&self) -> String {
        let definition = &self.definition;
        format!(
            "Skill {}\nIndex = {}\nName = {}\nLevel = {}\nDamage = {}\nMana = {}\nAbilityGauge = {}\nDistance = {}\nDelay = {}\nEnergy = {}\nCharisma = {}\nMasteryType = {}\nSkillUseType = {}\nSkillBrand = {}\nKillCount = {}\nRequireDutyClass = [{}]\nRequireClass = [{}]\nSkillRank = {}\nMagicIcon = {}\nTypeSkill = {}\nStrength = {}\nDexterity = {}\nItemSkill = {}\nIsDamage = {}\nEffect = {}",
            self.index,
            self.index,
            definition.name,
            definition.level,
            definition.damage,
            definition.mana,
            definition.ability_gauge,
            definition.distance,
            definition.delay_ms,
            definition.energy,
            definition.charisma,
            definition.mastery_type,
            definition.skill_use_type,
            definition.skill_brand,
            definition.kill_count,
            format_u8_array(&definition.require_duty_class),
            format_u8_array(&definition.require_class),
            definition.skill_rank,
            definition.magic_icon,
            definition.type_skill,
            definition.strength,
            definition.dexterity,
            definition.item_skill,
            definition.is_damage,
            definition.effect,
        )
    }

    pub fn export_csv(&self) -> String {
        let definition = &self.definition;
        [
            self.index.to_string(),
            csv_escape(&definition.name),
            definition.level.to_string(),
            definition.damage.to_string(),
            definition.mana.to_string(),
            definition.ability_gauge.to_string(),
            definition.distance.to_string(),
            definition.delay_ms.to_string(),
            definition.energy.to_string(),
            definition.charisma.to_string(),
            definition.mastery_type.to_string(),
            definition.skill_use_type.to_string(),
            definition.skill_brand.to_string(),
            definition.kill_count.to_string(),
            definition.require_duty_class[0].to_string(),
            definition.require_duty_class[1].to_string(),
            definition.require_duty_class[2].to_string(),
            definition.require_class[0].to_string(),
            definition.require_class[1].to_string(),
            definition.require_class[2].to_string(),
            definition.require_class[3].to_string(),
            definition.require_class[4].to_string(),
            definition.require_class[5].to_string(),
            definition.require_class[6].to_string(),
            definition.skill_rank.to_string(),
            definition.magic_icon.to_string(),
            definition.type_skill.to_string(),
            definition.strength.to_string(),
            definition.dexterity.to_string(),
            definition.item_skill.to_string(),
            definition.is_damage.to_string(),
            definition.effect.to_string(),
        ]
        .join(",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillEditorWorkflowState {
    Empty,
    Ready,
    Dirty,
    Saved,
    Cancelled,
    Failed { message: String },
}

impl SkillEditorWorkflowState {
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
pub enum SkillEditorError {
    NoRowsLoaded,
    NoSelectedRow,
    SelectedRowMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillEditorState {
    pub rows: Vec<SkillEditorRow>,
    baseline_rows: Vec<SkillEditorRow>,
    pub selected_row: Option<SkillId>,
    pub search_query: String,
    pub freeze_columns: bool,
    pub column_visibility: SkillEditorColumnVisibility,
    pub audit_log: Vec<String>,
    pub workflow_state: SkillEditorWorkflowState,
}

impl Default for SkillEditorState {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            baseline_rows: Vec::new(),
            selected_row: None,
            search_query: String::new(),
            freeze_columns: false,
            column_visibility: SkillEditorColumnVisibility::default(),
            audit_log: Vec::new(),
            workflow_state: SkillEditorWorkflowState::Empty,
        }
    }
}

impl Resource for SkillEditorState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct SkillEditorPlugin;

impl Plugin for SkillEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkillEditorState>();
    }
}

impl SkillEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rows(rows: Vec<SkillEditorRow>) -> Self {
        let mut state = Self::default();
        state.set_rows(rows);
        state
    }

    pub fn from_catalog(catalog: &SkillCatalog) -> Self {
        let mut rows = Vec::new();

        for skill_index in 0..MAX_SKILLS {
            let skill_id = skill_index as SkillId;
            let Some(definition) = catalog.definition(skill_id) else {
                continue;
            };

            if definition.name.trim().is_empty() {
                continue;
            }

            rows.push(SkillEditorRow::new(skill_id, definition.clone()));
        }

        Self::with_rows(rows)
    }

    pub fn sample() -> Self {
        Self::with_rows(sample_rows())
    }

    pub fn set_rows(&mut self, rows: Vec<SkillEditorRow>) {
        self.rows = rows.clone();
        self.baseline_rows = rows;
        self.selected_row = self.rows.first().map(|row| row.index);
        self.workflow_state = if self.rows.is_empty() {
            SkillEditorWorkflowState::Empty
        } else {
            SkillEditorWorkflowState::Ready
        };
        self.audit_log.clear();
        if !self.rows.is_empty() {
            self.audit_log
                .push(format!("Loaded {} skill row(s)", self.rows.len()));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        matches!(self.workflow_state, SkillEditorWorkflowState::Dirty)
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

    pub fn set_column_visible(&mut self, column: impl Into<String>, visible: bool) {
        self.column_visibility.set_visible(column, visible);
    }

    pub fn select_row(&mut self, index: SkillId) -> bool {
        if self.rows.iter().any(|row| row.index == index) {
            self.selected_row = Some(index);
            return true;
        }

        false
    }

    pub fn selected_row(&self) -> Option<&SkillEditorRow> {
        let selected_row = self.selected_row?;
        self.rows.iter().find(|row| row.index == selected_row)
    }

    pub fn selected_row_mut(&mut self) -> Option<&mut SkillEditorRow> {
        let selected_row = self.selected_row?;
        self.rows.iter_mut().find(|row| row.index == selected_row)
    }

    pub fn update_selected_row(
        &mut self,
        description: impl Into<String>,
        mut update: impl FnMut(&mut SkillEditorRow),
    ) -> Result<(), SkillEditorError> {
        if self.rows.is_empty() {
            return Err(SkillEditorError::NoRowsLoaded);
        }

        let Some(row) = self.selected_row_mut() else {
            return Err(SkillEditorError::NoSelectedRow);
        };

        update(row);
        self.workflow_state = SkillEditorWorkflowState::Dirty;
        self.audit_log.push(description.into());
        Ok(())
    }

    pub fn rename_selected_row(&mut self, name: impl Into<String>) -> Result<(), SkillEditorError> {
        let name = name.into();
        self.update_selected_row(
            format!("Renamed selected skill row to {}", name),
            move |row| {
                row.definition.name = name.clone();
            },
        )
    }

    pub fn mark_failed(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.workflow_state = SkillEditorWorkflowState::Failed {
            message: message.clone(),
        };
        self.audit_log
            .push(format!("Skill editor failed: {}", message));
    }

    pub fn save(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let changed_rows = self.changed_row_count();
        let message = if changed_rows == 0 {
            format!("No skill editor changes to save for {}", target_path)
        } else {
            format!("Saved {} skill row(s) to {}", changed_rows, target_path)
        };

        self.baseline_rows = self.rows.clone();
        self.workflow_state = if self.rows.is_empty() {
            SkillEditorWorkflowState::Empty
        } else {
            SkillEditorWorkflowState::Saved
        };
        self.audit_log.push(message.clone());
        message
    }

    pub fn cancel(&mut self) -> String {
        let changed_rows = self.changed_row_count();
        let message = if changed_rows == 0 {
            "No pending skill editor changes to cancel".to_string()
        } else {
            "Cancelled pending skill editor changes".to_string()
        };

        self.rows = self.baseline_rows.clone();
        self.selected_row = self.rows.first().map(|row| row.index);
        self.workflow_state = if self.rows.is_empty() {
            SkillEditorWorkflowState::Empty
        } else {
            SkillEditorWorkflowState::Cancelled
        };
        self.audit_log.push(message.clone());
        message
    }

    pub fn export_legacy(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let text = self.export_legacy_text();
        self.audit_log.push(format!(
            "Exported skill editor legacy data to {}",
            target_path
        ));
        text
    }

    pub fn export_csv(&mut self, target_path: impl Into<String>) -> String {
        let target_path = target_path.into();
        let text = self.export_csv_text();
        self.audit_log
            .push(format!("Exported skill editor CSV to {}", target_path));
        text
    }

    pub fn export_legacy_text(&self) -> String {
        if self.rows.is_empty() {
            return "No skill rows loaded".to_string();
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

    pub fn toolbar_actions(&self) -> &'static [SkillEditorAction] {
        if self.rows.is_empty() {
            EMPTY_ACTIONS
        } else {
            ACTIVE_ACTIONS
        }
    }

    pub fn filtered_rows(&self) -> Vec<&SkillEditorRow> {
        self.rows
            .iter()
            .filter(|row| row.matches_query(&self.search_query))
            .collect()
    }

    pub fn filtered_row_summaries(&self) -> Vec<String> {
        self.filtered_rows()
            .into_iter()
            .map(SkillEditorRow::snapshot)
            .collect()
    }

    pub fn selected_row_summary(&self) -> Option<String> {
        self.selected_row().map(SkillEditorRow::snapshot)
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

fn sample_rows() -> Vec<SkillEditorRow> {
    let mut teleport = sample_definition("Teleport");
    teleport.level = 10;
    teleport.mana = 50;
    teleport.distance = 6;
    teleport.delay_ms = 300;
    teleport.energy = 20;
    teleport.charisma = 12;
    teleport.skill_use_type = 1;
    teleport.skill_brand = 1001;
    teleport.is_damage = 1;
    teleport.require_class = [1, 0, 0, 0, 0, 0, 0];

    let mut triple_shot = sample_definition("Triple Shot");
    triple_shot.level = 20;
    triple_shot.damage = 30;
    triple_shot.mana = 20;
    triple_shot.distance = 8;
    triple_shot.delay_ms = 250;
    triple_shot.energy = 15;
    triple_shot.skill_rank = 1;
    triple_shot.magic_icon = 3;
    triple_shot.type_skill = 1;
    triple_shot.dexterity = 30;
    triple_shot.is_damage = 1;

    let mut summon = sample_definition("Summon Explosion");
    summon.level = 30;
    summon.damage = 45;
    summon.mana = 80;
    summon.distance = 4;
    summon.delay_ms = 900;
    summon.energy = 25;
    summon.effect = 77;
    summon.charisma = 18;
    summon.mastery_type = 2;
    summon.skill_use_type = 3;
    summon.skill_brand = 2001;
    summon.kill_count = 2;
    summon.skill_rank = 3;
    summon.magic_icon = 12;
    summon.type_skill = 2;
    summon.strength = 10;
    summon.dexterity = 20;
    summon.require_duty_class = [1, 0, 0];
    summon.require_class = [1, 0, 0, 0, 0, 0, 0];
    summon.is_damage = 1;

    vec![
        SkillEditorRow::new(6, teleport),
        SkillEditorRow::new(24, triple_shot),
        SkillEditorRow::new(223, summon),
    ]
}

fn sample_definition(name: &str) -> SkillDefinition {
    SkillDefinition {
        name: name.to_string(),
        ..SkillDefinition::default()
    }
}

fn format_u8_array(values: &[u8]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(", ")
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
        csv_escape, SkillEditorAction, SkillEditorColumnVisibility, SkillEditorPlugin,
        SkillEditorState, SkillEditorWorkflowState,
    };
    use bevy::prelude::App;
    use mu_gameplay::{SkillCatalog, SkillDefinition};

    #[test]
    fn skill_editor_state_tracks_edit_save_cancel_and_failure() {
        let mut state = SkillEditorState::sample();

        assert_eq!(state.workflow_state, SkillEditorWorkflowState::Ready);
        assert_eq!(state.row_count(), 3);
        assert_eq!(
            state.toolbar_actions(),
            SkillEditorState::sample().toolbar_actions()
        );
        assert!(state.select_row(24));
        state.set_search_query("triple");
        state.toggle_freeze_columns();
        state
            .rename_selected_row("Greater Triple Shot")
            .expect("row rename should work");
        assert!(state.is_dirty());
        assert_eq!(state.filtered_row_count(), 1);
        assert_eq!(
            state.selected_row_summary().as_deref(),
            Some("index=24|name=Greater Triple Shot|level=20|damage=30|mana=20|ability_gauge=0|distance=8|delay_ms=250|energy=15|charisma=0|mastery_type=0|skill_use_type=0|skill_brand=0|kill_count=0|require_duty_class=[0, 0, 0]|require_class=[0, 0, 0, 0, 0, 0, 0]|skill_rank=1|magic_icon=3|type_skill=1|strength=0|dexterity=30|item_skill=0|is_damage=1|effect=0")
        );

        let save_message = state.save("Data/Local/en/Skill_en.bmd");
        assert_eq!(
            save_message,
            "Saved 1 skill row(s) to Data/Local/en/Skill_en.bmd"
        );
        assert_eq!(state.workflow_state, SkillEditorWorkflowState::Saved);

        state
            .update_selected_row("Adjusted skill damage", |row| row.definition.damage = 36)
            .expect("row edit should work");
        assert_eq!(state.workflow_state, SkillEditorWorkflowState::Dirty);

        let cancel_message = state.cancel();
        assert_eq!(cancel_message, "Cancelled pending skill editor changes");
        assert_eq!(state.workflow_state, SkillEditorWorkflowState::Cancelled);
        assert_eq!(
            state.selected_row_summary().as_deref(),
            Some("index=6|name=Teleport|level=10|damage=0|mana=50|ability_gauge=0|distance=6|delay_ms=300|energy=20|charisma=12|mastery_type=0|skill_use_type=1|skill_brand=1001|kill_count=0|require_duty_class=[0, 0, 0]|require_class=[1, 0, 0, 0, 0, 0, 0]|skill_rank=0|magic_icon=0|type_skill=0|strength=0|dexterity=0|item_skill=0|is_damage=1|effect=0")
        );

        state.mark_failed("disk full");
        assert_eq!(
            state.workflow_state,
            SkillEditorWorkflowState::Failed {
                message: "disk full".to_string(),
            }
        );
        assert_eq!(state.workflow_state.message(), Some("disk full"));
        assert!(state.snapshot().contains("state=failed"));
    }

    #[test]
    fn skill_editor_exports_csv_and_legacy_text() {
        let mut state = SkillEditorState::sample();

        let csv = state.export_csv("Data/Local/en/Skills.csv");
        let legacy = state.export_legacy("Data/Local/en/Skills.bmd");

        assert!(csv.starts_with(
            "index,name,level,damage,mana,ability_gauge,distance,delay_ms,energy,charisma,mastery_type,skill_use_type,skill_brand,kill_count,require_duty_class_0,require_duty_class_1,require_duty_class_2,require_class_0,require_class_1,require_class_2,require_class_3,require_class_4,require_class_5,require_class_6,skill_rank,magic_icon,type_skill,strength,dexterity,item_skill,is_damage,effect"
        ));
        assert!(csv.contains("Teleport"));
        assert!(csv.contains("Summon Explosion"));
        assert!(legacy.contains("Skill 6"));
        assert!(legacy.contains("Name = Teleport"));
        assert!(legacy.contains("Name = Summon Explosion"));
        assert!(state
            .audit_log
            .iter()
            .any(|entry| entry.contains("Exported skill editor CSV")));
        assert!(state
            .audit_log
            .iter()
            .any(|entry| entry.contains("Exported skill editor legacy data")));
    }

    #[test]
    fn skill_editor_column_visibility_snapshots_and_plugin_registers_resource() {
        let visibility = SkillEditorColumnVisibility::default();

        assert!(visibility.is_visible("index"));
        assert_eq!(
            visibility.snapshot(),
            "index=true|name=true|level=true|damage=true|mana=true|ability_gauge=false|distance=true|delay_ms=true|energy=true|charisma=false|mastery_type=false|skill_use_type=false|skill_brand=false|kill_count=false|require_duty_class_0=false|require_duty_class_1=false|require_duty_class_2=false|require_class_0=false|require_class_1=false|require_class_2=false|require_class_3=false|require_class_4=false|require_class_5=false|require_class_6=false|skill_rank=false|magic_icon=false|type_skill=false|strength=true|dexterity=true|item_skill=false|is_damage=false|effect=false"
        );
        assert_eq!(csv_escape("Summon Explosion"), "Summon Explosion");
        assert_eq!(csv_escape("Summon, Explosion"), "\"Summon, Explosion\"");

        let mut app = App::new();
        app.add_plugins(SkillEditorPlugin);

        let state = app.world().resource::<SkillEditorState>();

        assert!(state.rows.is_empty());
        assert_eq!(state.workflow_state, SkillEditorWorkflowState::Empty);
        assert_eq!(state.toolbar_actions(), &[SkillEditorAction::Close]);
        assert_eq!(state.selected_row, None);
    }

    #[test]
    fn skill_editor_from_catalog_skips_empty_rows() {
        let mut catalog = SkillCatalog::new();
        catalog.insert(
            6,
            SkillDefinition {
                name: "Teleport".to_string(),
                level: 10,
                mana: 50,
                distance: 6,
                delay_ms: 300,
                energy: 20,
                ..SkillDefinition::default()
            },
        );
        catalog.insert(24, SkillDefinition::default());

        let state = SkillEditorState::from_catalog(&catalog);

        assert_eq!(state.row_count(), 1);
        assert_eq!(state.selected_row, Some(6));
        assert_eq!(
            state.selected_row_summary().as_deref(),
            Some("index=6|name=Teleport|level=10|damage=0|mana=50|ability_gauge=0|distance=6|delay_ms=300|energy=20|charisma=0|mastery_type=0|skill_use_type=0|skill_brand=0|kill_count=0|require_duty_class=[0, 0, 0]|require_class=[0, 0, 0, 0, 0, 0, 0]|skill_rank=0|magic_icon=0|type_skill=0|strength=0|dexterity=0|item_skill=0|is_damage=0|effect=0")
        );
    }
}
