use bevy::prelude::{App, Plugin, Resource};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuffState(u16);

impl BuffState {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BuffTimeType(u16);

impl BuffTimeType {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u16 {
        self.0
    }

    pub const fn from_effect_type(effect_type: u8) -> Self {
        Self(1005 + effect_type as u16)
    }

    pub const fn is_timed(self) -> bool {
        self.0 >= BUFF_TIME_HELLOWIN.raw()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuffClass {
    Buff,
    Debuff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuffValueLoadType {
    None,
    Skill,
    Item,
    Text,
    ItemAddOption,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuffDefinition {
    pub state: BuffState,
    pub effect_type: u8,
    pub item_type: Option<u8>,
    pub item_index: Option<u8>,
    pub name: String,
    pub class: BuffClass,
    pub notice_type: u8,
    pub clear_type: u8,
    pub description_lines: Vec<String>,
}

impl BuffDefinition {
    pub fn new(state: BuffState, effect_type: u8, class: BuffClass) -> Self {
        Self {
            state,
            effect_type,
            item_type: None,
            item_index: None,
            name: String::new(),
            class,
            notice_type: 0,
            clear_type: 0,
            description_lines: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_item(mut self, item_type: u8, item_index: u8) -> Self {
        self.item_type = Some(item_type);
        self.item_index = Some(item_index);
        self
    }

    pub fn with_notice_type(mut self, notice_type: u8) -> Self {
        self.notice_type = notice_type;
        self
    }

    pub fn with_clear_type(mut self, clear_type: u8) -> Self {
        self.clear_type = clear_type;
        self
    }

    pub fn with_description_text(mut self, text: &str) -> Self {
        self.description_lines = split_description_tokens(text);
        self
    }

    pub fn time_type(&self) -> BuffTimeType {
        BuffTimeType::from_effect_type(self.effect_type)
    }

    pub fn value_load_type(&self) -> BuffValueLoadType {
        value_load_type_for_state(self.state)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuffCatalog {
    entries: BTreeMap<BuffState, BuffDefinition>,
}

impl Resource for BuffCatalog {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuffRegistry {
    states: BTreeMap<BuffState, u32>,
}

impl Resource for BuffRegistry {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BuffTimerEntry {
    pub state: BuffState,
    pub remaining_millis: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuffTimerRegistry {
    entries: BTreeMap<BuffTimeType, BuffTimerEntry>,
}

impl Resource for BuffTimerRegistry {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DurationParts {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

pub const BUFF_STATE_ATTACK: BuffState = BuffState::new(1);
pub const BUFF_STATE_GM_EFFECT: BuffState = BuffState::new(28);

pub const BUFF_TIME_NONE: BuffTimeType = BuffTimeType::new(0);
pub const BUFF_TIME_HELLOWIN: BuffTimeType = BuffTimeType::new(1006);
pub const BUFF_TIME_PC_ROOM_SEAL: BuffTimeType = BuffTimeType::new(1007);
pub const BUFF_TIME_SEAL: BuffTimeType = BuffTimeType::new(1008);
pub const BUFF_TIME_SCROLL: BuffTimeType = BuffTimeType::new(1009);
pub const BUFF_TIME_SECRET: BuffTimeType = BuffTimeType::new(1010);
pub const BUFF_TIME_CHERRY_BLOSSOM: BuffTimeType = BuffTimeType::new(1011);
pub const BUFF_TIME_SWELL_OF_MP: BuffTimeType = BuffTimeType::new(1012);
pub const BUFF_TIME_CHRISTMAX: BuffTimeType = BuffTimeType::new(1013);
pub const BUFF_TIME_HONOR_OF_GLADIATOR: BuffTimeType = BuffTimeType::new(1014);
pub const BUFF_TIME_GUARD_CHARM: BuffTimeType = BuffTimeType::new(1015);
pub const BUFF_TIME_ITEM_GUARD_CHARM: BuffTimeType = BuffTimeType::new(1016);
pub const BUFF_TIME_AG_ADDITION: BuffTimeType = BuffTimeType::new(1017);
pub const BUFF_TIME_SD_ADDITION: BuffTimeType = BuffTimeType::new(1018);
pub const BUFF_TIME_PARTY_EXP_BONUS: BuffTimeType = BuffTimeType::new(1019);
pub const BUFF_TIME_COUNT: BuffTimeType = BuffTimeType::new(1020);

pub const BUFF_STATE_CASTLE_REGIMENT_DEFENSE: BuffState = BuffState::new(14);
pub const BUFF_STATE_CASTLE_REGIMENT_ATTACK1: BuffState = BuffState::new(15);
pub const BUFF_STATE_CASTLE_REGIMENT_ATTACK2: BuffState = BuffState::new(16);
pub const BUFF_STATE_CASTLE_REGIMENT_ATTACK3: BuffState = BuffState::new(17);
pub const BUFF_STATE_CRYWOLF_ALTAR_ENABLE: BuffState = BuffState::new(21);
pub const BUFF_STATE_CRYWOLF_NPC_HIDE: BuffState = BuffState::new(27);
pub const BUFF_STATE_PC_ROOM_SEAL1: BuffState = BuffState::new(29);
pub const BUFF_STATE_PC_ROOM_SEAL2: BuffState = BuffState::new(30);
pub const BUFF_STATE_PC_ROOM_SEAL3: BuffState = BuffState::new(31);
pub const BUFF_STATE_NEW_WEALTH_SEAL: BuffState = BuffState::new(119);
pub const BUFF_STATE_SEAL1: BuffState = BuffState::new(40);
pub const BUFF_STATE_SEAL4: BuffState = BuffState::new(43);
pub const BUFF_STATE_SEAL_HP_RECOVERY: BuffState = BuffState::new(87);
pub const BUFF_STATE_SEAL_MP_RECOVERY: BuffState = BuffState::new(88);
pub const BUFF_STATE_ASCENSION_SEAL_MASTER: BuffState = BuffState::new(101);
pub const BUFF_STATE_WEALTH_SEAL_MASTER: BuffState = BuffState::new(102);
pub const BUFF_STATE_ELITE_SCROLL1: BuffState = BuffState::new(44);
pub const BUFF_STATE_ELITE_SCROLL6: BuffState = BuffState::new(49);
pub const BUFF_STATE_SCROLL_BATTLE: BuffState = BuffState::new(89);
pub const BUFF_STATE_SCROLL_STRENGTHEN: BuffState = BuffState::new(90);
pub const BUFF_STATE_SECRET_POTION1: BuffState = BuffState::new(50);
pub const BUFF_STATE_SECRET_POTION5: BuffState = BuffState::new(54);

const EMPTY_TOKEN_GROUP: [BuffState; 0] = [];
const CASTLE_REGIMENT_TOKEN_GROUP: [BuffState; 4] = [
    BUFF_STATE_CASTLE_REGIMENT_DEFENSE,
    BUFF_STATE_CASTLE_REGIMENT_ATTACK1,
    BUFF_STATE_CASTLE_REGIMENT_ATTACK2,
    BUFF_STATE_CASTLE_REGIMENT_ATTACK3,
];
const CRYWOLF_TOKEN_GROUP: [BuffState; 7] = [
    BUFF_STATE_CRYWOLF_ALTAR_ENABLE,
    BuffState::new(22),
    BuffState::new(23),
    BuffState::new(24),
    BuffState::new(25),
    BuffState::new(26),
    BUFF_STATE_CRYWOLF_NPC_HIDE,
];
const PC_ROOM_SEAL_TOKEN_GROUP: [BuffState; 4] = [
    BUFF_STATE_NEW_WEALTH_SEAL,
    BUFF_STATE_PC_ROOM_SEAL1,
    BUFF_STATE_PC_ROOM_SEAL2,
    BUFF_STATE_PC_ROOM_SEAL3,
];
const SEAL_TOKEN_GROUP: [BuffState; 8] = [
    BUFF_STATE_SEAL1,
    BuffState::new(41),
    BuffState::new(42),
    BUFF_STATE_SEAL4,
    BUFF_STATE_SEAL_HP_RECOVERY,
    BUFF_STATE_SEAL_MP_RECOVERY,
    BUFF_STATE_ASCENSION_SEAL_MASTER,
    BUFF_STATE_WEALTH_SEAL_MASTER,
];
const ELITE_SCROLL_TOKEN_GROUP: [BuffState; 8] = [
    BUFF_STATE_ELITE_SCROLL1,
    BuffState::new(45),
    BuffState::new(46),
    BuffState::new(47),
    BuffState::new(48),
    BUFF_STATE_ELITE_SCROLL6,
    BUFF_STATE_SCROLL_BATTLE,
    BUFF_STATE_SCROLL_STRENGTHEN,
];
const SECRET_POTION_TOKEN_GROUP: [BuffState; 5] = [
    BUFF_STATE_SECRET_POTION1,
    BuffState::new(51),
    BuffState::new(52),
    BuffState::new(53),
    BUFF_STATE_SECRET_POTION5,
];

#[derive(Debug, Default, Clone, Copy)]
pub struct BuffPlugin;

impl Plugin for BuffPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuffCatalog>();
        app.init_resource::<BuffRegistry>();
        app.init_resource::<BuffTimerRegistry>();
    }
}

impl BuffCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, definition: BuffDefinition) -> Option<BuffDefinition> {
        self.entries.insert(definition.state, definition)
    }

    pub fn definition(&self, state: BuffState) -> Option<&BuffDefinition> {
        self.entries.get(&state)
    }

    pub fn definition_mut(&mut self, state: BuffState) -> Option<&mut BuffDefinition> {
        self.entries.get_mut(&state)
    }

    pub fn get_by_effect_type(&self, effect_type: u8) -> Option<&BuffDefinition> {
        self.entries
            .values()
            .find(|definition| definition.effect_type == effect_type)
    }

    pub fn time_type(&self, state: BuffState) -> Option<BuffTimeType> {
        self.definition(state).map(BuffDefinition::time_type)
    }

    pub fn value_load_type(&self, state: BuffState) -> Option<BuffValueLoadType> {
        self.definition(state).map(BuffDefinition::value_load_type)
    }

    pub fn definition_for_effect_type(&self, effect_type: u8) -> Option<&BuffDefinition> {
        self.get_by_effect_type(effect_type)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BuffState, &BuffDefinition)> {
        self.entries.iter()
    }
}

impl BuffRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.states.clear();
    }

    pub fn is_active(&self) -> bool {
        !self.states.is_empty()
    }

    pub fn contains(&self, state: BuffState) -> bool {
        self.states.contains_key(&state)
    }

    pub fn count(&self, state: BuffState) -> u32 {
        self.states.get(&state).copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn state_at(&self, index: usize) -> Option<BuffState> {
        self.states.keys().copied().nth(index)
    }

    pub fn states(&self) -> impl Iterator<Item = BuffState> + '_ {
        self.states.keys().copied()
    }

    pub fn register(&mut self, state: BuffState) -> bool {
        self.states.insert(state, 1).is_none()
    }

    pub fn register_many<I>(&mut self, states: I)
    where
        I: IntoIterator<Item = BuffState>,
    {
        for state in states {
            self.register(state);
        }
    }

    pub fn unregister(&mut self, state: BuffState) -> bool {
        self.states.remove(&state).is_some()
    }

    pub fn unregister_many<I>(&mut self, states: I)
    where
        I: IntoIterator<Item = BuffState>,
    {
        for state in states {
            self.unregister(state);
        }
    }

    pub fn first_active_match<I>(&self, states: I) -> Option<BuffState>
    where
        I: IntoIterator<Item = BuffState>,
    {
        states.into_iter().find(|state| self.contains(*state))
    }

    pub fn tokenize(&mut self, state: BuffState) -> bool {
        let mut changed = false;
        for token in token_group(state) {
            changed |= self.unregister(*token);
        }
        let inserted = self.register(state);
        changed || inserted
    }

    pub fn matching_definition<'a>(
        &'a self,
        effect_type: u8,
        catalog: &'a BuffCatalog,
    ) -> Option<&'a BuffDefinition> {
        self.states.keys().find_map(|state| {
            catalog
                .definition(*state)
                .filter(|definition| definition.effect_type == effect_type)
        })
    }

    pub fn matching_buff_name<'a>(
        &'a self,
        effect_type: u8,
        catalog: &'a BuffCatalog,
    ) -> Option<&'a str> {
        self.matching_definition(effect_type, catalog)
            .map(|definition| definition.name.as_str())
    }
}

impl BuffTimerEntry {
    pub fn new(state: BuffState, remaining_millis: u32) -> Self {
        Self {
            state,
            remaining_millis,
        }
    }

    pub fn remaining_seconds(self) -> u32 {
        self.remaining_millis / 1000
    }

    pub fn remaining_parts(self) -> DurationParts {
        split_duration(self.remaining_seconds())
    }
}

impl BuffTimerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn contains_time_type(&self, time_type: BuffTimeType) -> bool {
        self.entries.contains_key(&time_type)
    }

    pub fn time_type_for_state(&self, state: BuffState) -> Option<BuffTimeType> {
        self.entries.iter().find_map(|(time_type, entry)| {
            if entry.state == state {
                Some(*time_type)
            } else {
                None
            }
        })
    }

    pub fn remaining_millis(&self, time_type: BuffTimeType) -> Option<u32> {
        self.entries
            .get(&time_type)
            .map(|entry| entry.remaining_millis)
    }

    pub fn remaining_seconds(&self, time_type: BuffTimeType) -> Option<u32> {
        self.remaining_millis(time_type)
            .map(|remaining| remaining / 1000)
    }

    pub fn remaining_parts(&self, time_type: BuffTimeType) -> Option<DurationParts> {
        self.remaining_seconds(time_type).map(split_duration)
    }

    pub fn register_definition(
        &mut self,
        definition: &BuffDefinition,
        current_seconds: u32,
        item_duration_seconds: Option<u32>,
    ) -> Option<BuffTimeType> {
        let duration_seconds = if current_seconds == 0 {
            item_duration_seconds?
        } else {
            current_seconds
        };

        if duration_seconds == 0 {
            return None;
        }

        let time_type = definition.time_type();
        if !time_type.is_timed() || self.entries.contains_key(&time_type) {
            return None;
        }

        self.entries.insert(
            time_type,
            BuffTimerEntry::new(definition.state, duration_seconds.saturating_mul(1000)),
        );
        Some(time_type)
    }

    pub fn unregister_time_type(&mut self, time_type: BuffTimeType) -> bool {
        self.entries.remove(&time_type).is_some()
    }

    pub fn unregister_state(&mut self, state: BuffState) -> bool {
        let Some(time_type) = self.time_type_for_state(state) else {
            return false;
        };

        self.unregister_time_type(time_type)
    }

    pub fn advance(&mut self, elapsed_millis: u32) -> Vec<BuffTimeType> {
        let mut expired = Vec::new();

        for (time_type, entry) in &mut self.entries {
            entry.remaining_millis = entry.remaining_millis.saturating_sub(elapsed_millis);
            if entry.remaining_millis == 0 {
                expired.push(*time_type);
            }
        }

        for time_type in &expired {
            self.entries.remove(time_type);
        }

        expired
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BuffTimeType, &BuffTimerEntry)> {
        self.entries.iter()
    }
}

pub fn split_duration(total_seconds: u32) -> DurationParts {
    let days = total_seconds / 86_400;
    let remaining_after_days = total_seconds % 86_400;
    let hours = remaining_after_days / 3600;
    let remaining_after_hours = remaining_after_days % 3600;
    let minutes = remaining_after_hours / 60;
    let seconds = remaining_after_hours % 60;

    DurationParts {
        days,
        hours,
        minutes,
        seconds,
    }
}

pub fn split_minutes_duration(total_minutes: u32) -> DurationParts {
    let days = total_minutes / 1_440;
    let remaining_after_days = total_minutes % 1_440;
    let hours = remaining_after_days / 60;
    let minutes = remaining_after_days % 60;

    DurationParts {
        days,
        hours,
        minutes,
        seconds: 0,
    }
}

pub fn token_group(state: BuffState) -> &'static [BuffState] {
    match state.raw() {
        14..=17 => &CASTLE_REGIMENT_TOKEN_GROUP,
        21..=27 => &CRYWOLF_TOKEN_GROUP,
        29..=31 | 119 => &PC_ROOM_SEAL_TOKEN_GROUP,
        40..=43 | 101 | 102 => &SEAL_TOKEN_GROUP,
        44..=49 => &ELITE_SCROLL_TOKEN_GROUP,
        50..=54 => &SECRET_POTION_TOKEN_GROUP,
        _ => &EMPTY_TOKEN_GROUP,
    }
}

pub fn value_load_type_for_state(state: BuffState) -> BuffValueLoadType {
    if state.raw() >= BUFF_STATE_ATTACK.raw() && state.raw() <= BUFF_STATE_GM_EFFECT.raw() {
        BuffValueLoadType::None
    } else {
        BuffValueLoadType::ItemAddOption
    }
}

fn split_description_tokens(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    text.split('/').map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        split_duration, split_minutes_duration, token_group, value_load_type_for_state,
        BuffCatalog, BuffClass, BuffDefinition, BuffPlugin, BuffRegistry, BuffState, BuffTimeType,
        BuffTimerRegistry, BuffValueLoadType, BUFF_STATE_CASTLE_REGIMENT_ATTACK1,
        BUFF_STATE_NEW_WEALTH_SEAL, BUFF_STATE_PC_ROOM_SEAL1, BUFF_STATE_PC_ROOM_SEAL2,
    };
    use bevy::prelude::App;

    #[test]
    fn token_groups_match_the_legacy_buffs() {
        assert_eq!(
            token_group(BuffState::new(14)),
            &[
                super::BUFF_STATE_CASTLE_REGIMENT_DEFENSE,
                BUFF_STATE_CASTLE_REGIMENT_ATTACK1,
                super::BUFF_STATE_CASTLE_REGIMENT_ATTACK2,
                super::BUFF_STATE_CASTLE_REGIMENT_ATTACK3
            ]
        );
        assert_eq!(
            token_group(BuffState::new(119)),
            &[
                BUFF_STATE_NEW_WEALTH_SEAL,
                BUFF_STATE_PC_ROOM_SEAL1,
                BUFF_STATE_PC_ROOM_SEAL2,
                super::BUFF_STATE_PC_ROOM_SEAL3,
            ]
        );
        assert!(token_group(BuffState::new(200)).is_empty());
    }

    #[test]
    fn registry_tokenizes_and_preserves_unrelated_states() {
        let mut registry = BuffRegistry::new();
        registry.register(BuffState::new(29));
        registry.register(BuffState::new(30));
        registry.register(BuffState::new(44));

        assert!(registry.tokenize(BuffState::new(119)));
        assert!(registry.contains(BuffState::new(119)));
        assert!(!registry.contains(BuffState::new(29)));
        assert!(!registry.contains(BuffState::new(30)));
        assert!(registry.contains(BuffState::new(44)));
        assert_eq!(registry.state_at(0), Some(BuffState::new(44)));
        assert_eq!(registry.state_at(1), Some(BuffState::new(119)));
    }

    #[test]
    fn catalog_tracks_effect_types_and_value_loading() {
        let definition = BuffDefinition::new(BuffState::new(119), 7, BuffClass::Buff)
            .with_name("New Wealth Seal")
            .with_description_text("first/second//fourth/");
        assert_eq!(
            definition.description_lines,
            vec!["first", "second", "", "fourth", ""]
        );
        assert_eq!(definition.time_type(), BuffTimeType::from_effect_type(7));
        assert_eq!(
            definition.value_load_type(),
            BuffValueLoadType::ItemAddOption
        );

        let mut catalog = BuffCatalog::new();
        catalog.insert(definition.clone());
        assert_eq!(catalog.get_by_effect_type(7), Some(&definition));
        assert_eq!(
            catalog.value_load_type(BuffState::new(119)),
            Some(BuffValueLoadType::ItemAddOption)
        );
    }

    #[test]
    fn registry_can_match_active_effect_names() {
        let definition = BuffDefinition::new(BuffState::new(14), 7, BuffClass::Buff)
            .with_name("Castle Regiment Defense");
        let mut catalog = BuffCatalog::new();
        catalog.insert(definition);

        let mut registry = BuffRegistry::new();
        registry.register(BuffState::new(14));

        assert_eq!(
            registry.matching_buff_name(7, &catalog),
            Some("Castle Regiment Defense")
        );
    }

    #[test]
    fn timer_registry_uses_item_duration_and_advances() {
        let definition = BuffDefinition::new(BuffState::new(14), 7, BuffClass::Buff);
        let mut timers = BuffTimerRegistry::new();

        let time_type = timers
            .register_definition(&definition, 0, Some(90))
            .expect("timer should register");

        assert_eq!(time_type, BuffTimeType::from_effect_type(7));
        assert_eq!(
            timers.remaining_parts(time_type),
            Some(super::DurationParts {
                days: 0,
                hours: 0,
                minutes: 1,
                seconds: 30,
            })
        );

        assert_eq!(timers.advance(30_000), Vec::new());
        assert_eq!(timers.remaining_seconds(time_type), Some(60));
        assert_eq!(timers.advance(60_000), vec![time_type]);
        assert!(!timers.contains_time_type(time_type));
    }

    #[test]
    fn duration_parts_split_seconds_and_minutes_like_the_legacy_ui_helpers() {
        assert_eq!(
            split_duration(90_061),
            super::DurationParts {
                days: 1,
                hours: 1,
                minutes: 1,
                seconds: 1,
            }
        );
        assert_eq!(
            split_minutes_duration(1_500),
            super::DurationParts {
                days: 1,
                hours: 1,
                minutes: 0,
                seconds: 0,
            }
        );
    }

    #[test]
    fn buff_plugin_initializes_the_resources() {
        let mut app = App::new();
        app.add_plugins(BuffPlugin);

        assert!(app.world().get_resource::<BuffCatalog>().is_some());
        assert!(app.world().get_resource::<BuffRegistry>().is_some());
        assert!(app.world().get_resource::<BuffTimerRegistry>().is_some());
    }

    #[test]
    fn item_based_value_loading_uses_the_legacy_state_range() {
        assert_eq!(
            value_load_type_for_state(BuffState::new(1)),
            BuffValueLoadType::None
        );
        assert_eq!(
            value_load_type_for_state(BuffState::new(28)),
            BuffValueLoadType::None
        );
        assert_eq!(
            value_load_type_for_state(BuffState::new(119)),
            BuffValueLoadType::ItemAddOption
        );
    }
}
