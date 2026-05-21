use std::collections::BTreeSet;

use thiserror::Error;

pub const MU_HELPER_PACKET_PAYLOAD_SIZE: usize = 257;
pub const MU_HELPER_MAX_HUNTING_RANGE: u8 = 6;
pub const MU_HELPER_MAX_OBTAINING_RANGE: u8 = 8;
pub const MU_HELPER_MAX_SECONDS_AWAY: u8 = 15;
pub const MU_HELPER_MAX_THRESHOLD: u8 = 100;
pub const MU_HELPER_THRESHOLD_STEP: u8 = 10;
pub const MU_HELPER_MAX_EXTRA_ITEMS: usize = 12;
pub const MU_HELPER_MAX_EXTRA_ITEM_LENGTH: usize = 15;

const MU_HELPER_EXTRA_ITEM_BYTES: usize = 15;
const MU_HELPER_EXTRA_ITEM_COUNT: usize = 12;
const MU_HELPER_EXTRA_ITEMS_OFFSET: usize = 65;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum MuHelperDarkRavenMode {
    #[default]
    Cease = 0,
    Auto = 1,
    Together = 2,
}

impl MuHelperDarkRavenMode {
    pub fn from_packet_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Cease),
            1 => Some(Self::Auto),
            2 => Some(Self::Together),
            _ => None,
        }
    }

    pub const fn as_packet_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MuHelperMobPresence {
    #[default]
    Nearby,
    Attacking,
}

impl MuHelperMobPresence {
    pub const fn as_packet_value(self) -> u8 {
        match self {
            Self::Nearby => 0,
            Self::Attacking => 1,
        }
    }

    pub fn from_packet_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Nearby),
            1 => Some(Self::Attacking),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MuHelperMobCount {
    #[default]
    Two,
    Three,
    Four,
    Five,
}

impl MuHelperMobCount {
    pub const fn as_packet_value(self) -> u8 {
        match self {
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
        }
    }

    pub fn from_packet_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Two),
            1 => Some(Self::Three),
            2 => Some(Self::Four),
            3 => Some(Self::Five),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MuHelperSkillCondition {
    pub on_timer: bool,
    pub on_condition: bool,
    pub mob_presence: MuHelperMobPresence,
    pub mob_count: MuHelperMobCount,
}

impl MuHelperSkillCondition {
    fn encode(self) -> (bool, bool, u8, u8) {
        (
            self.on_timer,
            self.on_condition,
            self.mob_presence.as_packet_value(),
            self.mob_count.as_packet_value(),
        )
    }

    fn decode(on_timer: bool, on_condition: bool, mob_presence: u8, mob_count: u8) -> Self {
        Self {
            on_timer,
            on_condition,
            mob_presence: MuHelperMobPresence::from_packet_value(mob_presence).unwrap_or_default(),
            mob_count: MuHelperMobCount::from_packet_value(mob_count).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MuHelperSkillSlot {
    pub skill_id: u16,
    pub interval_seconds: u16,
    pub condition: MuHelperSkillCondition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuHelperConfig {
    pub hunting_range: u8,
    pub max_seconds_away: u8,
    pub long_range_counter_attack: bool,
    pub return_to_original_position: bool,
    pub basic_skill_id: u16,
    pub activation_skills: [MuHelperSkillSlot; 2],
    pub use_combo: bool,
    pub buff_skill_ids: [u16; 3],
    pub buff_duration: bool,
    pub buff_duration_party: bool,
    pub buff_cast_interval: u16,
    pub auto_heal: bool,
    pub heal_threshold: u8,
    pub support_party: bool,
    pub auto_heal_party: bool,
    pub heal_party_threshold: u8,
    pub use_heal_potion: bool,
    pub potion_threshold: u8,
    pub use_drain_life: bool,
    pub use_dark_raven: bool,
    pub dark_raven_mode: MuHelperDarkRavenMode,
    pub repair_item: bool,
    pub obtaining_range: u8,
    pub pick_all_items: bool,
    pub pick_select_items: bool,
    pub pick_jewel: bool,
    pub pick_zen: bool,
    pub pick_ancient: bool,
    pub pick_excellent: bool,
    pub pick_extra_items: bool,
    pub extra_items: BTreeSet<String>,
    pub use_self_defense: bool,
    pub auto_accept_friend: bool,
    pub auto_accept_guild: bool,
}

impl Default for MuHelperConfig {
    fn default() -> Self {
        Self {
            hunting_range: MU_HELPER_MAX_HUNTING_RANGE,
            max_seconds_away: 10,
            long_range_counter_attack: false,
            return_to_original_position: true,
            basic_skill_id: 0,
            activation_skills: [MuHelperSkillSlot::default(), MuHelperSkillSlot::default()],
            use_combo: false,
            buff_skill_ids: [0; 3],
            buff_duration: true,
            buff_duration_party: true,
            buff_cast_interval: 0,
            auto_heal: false,
            heal_threshold: 60,
            support_party: false,
            auto_heal_party: false,
            heal_party_threshold: 60,
            use_heal_potion: false,
            potion_threshold: 40,
            use_drain_life: false,
            use_dark_raven: false,
            dark_raven_mode: MuHelperDarkRavenMode::Cease,
            repair_item: false,
            obtaining_range: MU_HELPER_MAX_OBTAINING_RANGE,
            pick_all_items: false,
            pick_select_items: false,
            pick_jewel: false,
            pick_zen: false,
            pick_ancient: false,
            pick_excellent: false,
            pick_extra_items: false,
            extra_items: BTreeSet::new(),
            use_self_defense: false,
            auto_accept_friend: false,
            auto_accept_guild: false,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum MuHelperConfigError {
    #[error("hunting range {value} exceeds the legacy limit of {max}")]
    HuntingRangeOutOfRange { value: u8, max: u8 },
    #[error("obtaining range {value} exceeds the legacy limit of {max}")]
    ObtainingRangeOutOfRange { value: u8, max: u8 },
    #[error("seconds away {value} exceeds the legacy limit of {max}")]
    SecondsAwayOutOfRange { value: u8, max: u8 },
    #[error("{field} threshold {value} must be a multiple of {step} and at most {max}")]
    ThresholdOutOfRange {
        field: &'static str,
        value: u8,
        step: u8,
        max: u8,
    },
    #[error("extra item list has {count} entries but the legacy limit is {max}")]
    TooManyExtraItems { count: usize, max: usize },
    #[error("extra item #{index} is longer than {max} characters")]
    ExtraItemTooLong {
        index: usize,
        max: usize,
        length: usize,
    },
    #[error("extra item #{index} is not ascii")]
    ExtraItemNotAscii { index: usize },
}

impl MuHelperConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), MuHelperConfigError> {
        validate_range("hunting", self.hunting_range, MU_HELPER_MAX_HUNTING_RANGE)?;
        validate_range(
            "obtaining",
            self.obtaining_range,
            MU_HELPER_MAX_OBTAINING_RANGE,
        )?;
        validate_range(
            "seconds away",
            self.max_seconds_away,
            MU_HELPER_MAX_SECONDS_AWAY,
        )?;

        validate_threshold("heal", self.heal_threshold)?;
        validate_threshold("heal party", self.heal_party_threshold)?;
        validate_threshold("potion", self.potion_threshold)?;

        if self.extra_items.len() > MU_HELPER_MAX_EXTRA_ITEMS {
            return Err(MuHelperConfigError::TooManyExtraItems {
                count: self.extra_items.len(),
                max: MU_HELPER_MAX_EXTRA_ITEMS,
            });
        }

        for (index, item) in self.extra_items.iter().enumerate() {
            if !item.is_ascii() {
                return Err(MuHelperConfigError::ExtraItemNotAscii { index });
            }

            let length = item.chars().count();
            if length > MU_HELPER_MAX_EXTRA_ITEM_LENGTH {
                return Err(MuHelperConfigError::ExtraItemTooLong {
                    index,
                    max: MU_HELPER_MAX_EXTRA_ITEM_LENGTH,
                    length,
                });
            }
        }

        Ok(())
    }

    pub fn to_packet_payload(
        &self,
    ) -> Result<[u8; MU_HELPER_PACKET_PAYLOAD_SIZE], MuHelperConfigError> {
        self.validate()?;

        let mut payload = [0u8; MU_HELPER_PACKET_PAYLOAD_SIZE];
        payload[1] = encode_packet_flags(
            self.pick_jewel,
            self.pick_ancient,
            self.pick_excellent,
            self.pick_zen,
            self.pick_extra_items,
        );
        payload[2] = (self.hunting_range & 0x0F) | ((self.obtaining_range & 0x0F) << 4);
        write_u16(&mut payload, 3, u16::from(self.max_seconds_away & 0x0F));
        write_u16(&mut payload, 5, self.basic_skill_id);
        write_u16(&mut payload, 7, self.activation_skills[0].skill_id);
        write_u16(&mut payload, 9, self.activation_skills[0].interval_seconds);
        write_u16(&mut payload, 11, self.activation_skills[1].skill_id);
        write_u16(&mut payload, 13, self.activation_skills[1].interval_seconds);
        write_u16(&mut payload, 15, self.buff_cast_interval);
        write_u16(&mut payload, 17, self.buff_skill_ids[0]);
        write_u16(&mut payload, 19, self.buff_skill_ids[1]);
        write_u16(&mut payload, 21, self.buff_skill_ids[2]);
        payload[23] = ((self.potion_threshold / MU_HELPER_THRESHOLD_STEP) & 0x0F)
            | (((self.heal_threshold / MU_HELPER_THRESHOLD_STEP) & 0x0F) << 4);
        payload[24] = ((self.heal_party_threshold / MU_HELPER_THRESHOLD_STEP) & 0x0F)
            | (((self.heal_threshold / MU_HELPER_THRESHOLD_STEP) & 0x0F) << 4);
        payload[25] = encode_bool_flags([
            self.use_heal_potion,
            self.auto_heal,
            self.use_drain_life,
            self.long_range_counter_attack,
            self.return_to_original_position,
            self.use_combo,
            self.support_party,
            self.auto_heal_party,
        ]);
        let (skill1_delay, skill1_condition, skill1_presence, skill1_count) =
            self.activation_skills[0].condition.encode();
        payload[26] = encode_skill1_byte(
            self.buff_duration_party,
            self.use_dark_raven,
            self.buff_duration,
            skill1_delay,
            skill1_condition,
            skill1_presence,
            skill1_count,
        );
        let (skill2_delay, skill2_condition, skill2_presence, skill2_count) =
            self.activation_skills[1].condition.encode();
        payload[27] = encode_skill2_byte(
            skill2_delay,
            skill2_condition,
            skill2_presence,
            skill2_count,
            self.repair_item,
            self.pick_all_items,
            self.pick_select_items,
        );
        payload[28] = self.dark_raven_mode.as_packet_value();

        let mut offset = MU_HELPER_EXTRA_ITEMS_OFFSET;
        for item in self.extra_items.iter().take(MU_HELPER_EXTRA_ITEM_COUNT) {
            write_fixed_ascii(
                &mut payload[offset..offset + MU_HELPER_EXTRA_ITEM_BYTES],
                item,
            );
            offset += MU_HELPER_EXTRA_ITEM_BYTES;
        }

        Ok(payload)
    }

    pub fn from_packet_payload(payload: &[u8; MU_HELPER_PACKET_PAYLOAD_SIZE]) -> Self {
        let mut extra_items = BTreeSet::new();
        for chunk in payload[MU_HELPER_EXTRA_ITEMS_OFFSET..]
            .chunks_exact(MU_HELPER_EXTRA_ITEM_BYTES)
            .take(MU_HELPER_EXTRA_ITEM_COUNT)
        {
            if let Some(item) = decode_fixed_ascii(chunk) {
                extra_items.insert(item);
            }
        }

        let skill1_condition = MuHelperSkillCondition::decode(
            read_bool_flag(payload[26], 3),
            read_bool_flag(payload[26], 4),
            if read_bool_flag(payload[26], 5) { 1 } else { 0 },
            read_u8_bits(payload[26], 6, 2),
        );
        let skill2_condition = MuHelperSkillCondition::decode(
            read_bool_flag(payload[27], 0),
            read_bool_flag(payload[27], 1),
            read_u8_bits(payload[27], 2, 1),
            read_u8_bits(payload[27], 3, 2),
        );

        Self {
            hunting_range: payload[2] & 0x0F,
            max_seconds_away: read_u16(payload, 3) as u8,
            long_range_counter_attack: read_bool_flag(payload[25], 3),
            return_to_original_position: read_bool_flag(payload[25], 4),
            basic_skill_id: read_u16(payload, 5),
            activation_skills: [
                MuHelperSkillSlot {
                    skill_id: read_u16(payload, 7),
                    interval_seconds: read_u16(payload, 9),
                    condition: skill1_condition,
                },
                MuHelperSkillSlot {
                    skill_id: read_u16(payload, 11),
                    interval_seconds: read_u16(payload, 13),
                    condition: skill2_condition,
                },
            ],
            use_combo: read_bool_flag(payload[25], 5),
            buff_skill_ids: [
                read_u16(payload, 17),
                read_u16(payload, 19),
                read_u16(payload, 21),
            ],
            buff_duration: read_bool_flag(payload[26], 2),
            buff_duration_party: read_bool_flag(payload[26], 0),
            buff_cast_interval: read_u16(payload, 15),
            auto_heal: read_bool_flag(payload[25], 1),
            heal_threshold: ((payload[23] >> 4) & 0x0F) * MU_HELPER_THRESHOLD_STEP,
            support_party: read_bool_flag(payload[25], 6),
            auto_heal_party: read_bool_flag(payload[25], 7),
            heal_party_threshold: (payload[24] & 0x0F) * MU_HELPER_THRESHOLD_STEP,
            use_heal_potion: read_bool_flag(payload[25], 0),
            potion_threshold: (payload[23] & 0x0F) * MU_HELPER_THRESHOLD_STEP,
            use_drain_life: read_bool_flag(payload[25], 2),
            use_dark_raven: read_bool_flag(payload[26], 1),
            dark_raven_mode: MuHelperDarkRavenMode::from_packet_value(payload[28])
                .unwrap_or_default(),
            repair_item: read_bool_flag(payload[27], 5),
            obtaining_range: (payload[2] >> 4) & 0x0F,
            pick_all_items: read_bool_flag(payload[27], 6),
            pick_select_items: read_bool_flag(payload[27], 7),
            pick_jewel: read_bool_flag(payload[1], 3),
            pick_ancient: read_bool_flag(payload[1], 4),
            pick_excellent: read_bool_flag(payload[1], 5),
            pick_zen: read_bool_flag(payload[1], 6),
            pick_extra_items: read_bool_flag(payload[1], 7),
            extra_items,
            use_self_defense: false,
            auto_accept_friend: false,
            auto_accept_guild: false,
        }
    }
}

fn validate_range(field: &'static str, value: u8, max: u8) -> Result<(), MuHelperConfigError> {
    if value > max {
        return Err(match field {
            "hunting" => MuHelperConfigError::HuntingRangeOutOfRange { value, max },
            "obtaining" => MuHelperConfigError::ObtainingRangeOutOfRange { value, max },
            "seconds away" => MuHelperConfigError::SecondsAwayOutOfRange { value, max },
            _ => unreachable!(),
        });
    }

    Ok(())
}

fn validate_threshold(field: &'static str, value: u8) -> Result<(), MuHelperConfigError> {
    if value > MU_HELPER_MAX_THRESHOLD || !value.is_multiple_of(MU_HELPER_THRESHOLD_STEP) {
        return Err(MuHelperConfigError::ThresholdOutOfRange {
            field,
            value,
            step: MU_HELPER_THRESHOLD_STEP,
            max: MU_HELPER_MAX_THRESHOLD,
        });
    }

    Ok(())
}

fn encode_bool_flags(flags: [bool; 8]) -> u8 {
    let mut value = 0u8;
    for (bit_index, flag) in flags.into_iter().enumerate() {
        if flag {
            value |= 1 << bit_index;
        }
    }
    value
}

fn encode_packet_flags(
    jewel_or_gem: bool,
    set_item: bool,
    excellent_item: bool,
    zen: bool,
    add_extra_item: bool,
) -> u8 {
    let mut value = 0u8;
    if jewel_or_gem {
        value |= 1 << 3;
    }
    if set_item {
        value |= 1 << 4;
    }
    if excellent_item {
        value |= 1 << 5;
    }
    if zen {
        value |= 1 << 6;
    }
    if add_extra_item {
        value |= 1 << 7;
    }
    value
}

fn encode_skill1_byte(
    buff_duration_party: bool,
    use_dark_raven: bool,
    buff_duration: bool,
    delay: bool,
    condition: bool,
    presence: u8,
    count: u8,
) -> u8 {
    let mut value = 0u8;
    if buff_duration_party {
        value |= 1 << 0;
    }
    if use_dark_raven {
        value |= 1 << 1;
    }
    if buff_duration {
        value |= 1 << 2;
    }
    if delay {
        value |= 1 << 3;
    }
    if condition {
        value |= 1 << 4;
    }
    if presence != 0 {
        value |= 1 << 5;
    }
    value |= (count & 0x03) << 6;
    value
}

fn encode_skill2_byte(
    delay: bool,
    condition: bool,
    presence: u8,
    count: u8,
    repair_item: bool,
    pick_all_items: bool,
    pick_select_items: bool,
) -> u8 {
    let mut value = 0u8;
    if delay {
        value |= 1 << 0;
    }
    if condition {
        value |= 1 << 1;
    }
    if presence != 0 {
        value |= 1 << 2;
    }
    value |= (count & 0x03) << 3;
    if repair_item {
        value |= 1 << 5;
    }
    if pick_all_items {
        value |= 1 << 6;
    }
    if pick_select_items {
        value |= 1 << 7;
    }
    value
}

fn read_bool_flag(byte: u8, bit_index: u8) -> bool {
    (byte & (1 << bit_index)) != 0
}

fn read_u8_bits(byte: u8, start_bit: u8, bit_count: u8) -> u8 {
    (byte >> start_bit) & ((1 << bit_count) - 1)
}

fn read_u16(payload: &[u8; MU_HELPER_PACKET_PAYLOAD_SIZE], offset: usize) -> u16 {
    u16::from_le_bytes([payload[offset], payload[offset + 1]])
}

fn write_u16(payload: &mut [u8; MU_HELPER_PACKET_PAYLOAD_SIZE], offset: usize, value: u16) {
    let bytes = value.to_le_bytes();
    payload[offset] = bytes[0];
    payload[offset + 1] = bytes[1];
}

fn write_fixed_ascii(output: &mut [u8], value: &str) {
    let bytes = value.as_bytes();
    let copy_len = bytes.len().min(output.len());
    output[..copy_len].copy_from_slice(&bytes[..copy_len]);
    for byte in &mut output[copy_len..] {
        *byte = 0;
    }
}

fn decode_fixed_ascii(bytes: &[u8]) -> Option<String> {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    if end == 0 {
        return None;
    }

    let value = std::str::from_utf8(&bytes[..end]).ok()?;
    Some(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        MuHelperConfig, MuHelperConfigError, MuHelperDarkRavenMode, MuHelperMobCount,
        MuHelperMobPresence, MuHelperSkillCondition, MuHelperSkillSlot, MU_HELPER_MAX_EXTRA_ITEMS,
        MU_HELPER_MAX_EXTRA_ITEM_LENGTH, MU_HELPER_MAX_HUNTING_RANGE,
        MU_HELPER_MAX_OBTAINING_RANGE, MU_HELPER_PACKET_PAYLOAD_SIZE, MU_HELPER_THRESHOLD_STEP,
    };
    use mu_protocol::mu_helper::mu_helper_save_data_request;
    use proptest::prelude::*;
    use std::collections::BTreeSet;

    fn threshold_strategy() -> impl Strategy<Value = u8> {
        prop::sample::select(
            (0..=10)
                .map(|value| value * MU_HELPER_THRESHOLD_STEP)
                .collect::<Vec<_>>(),
        )
    }

    fn extra_item_strategy() -> impl Strategy<Value = String> {
        prop::collection::vec(
            proptest::char::range('a', 'z'),
            1..=MU_HELPER_MAX_EXTRA_ITEM_LENGTH,
        )
        .prop_map(|chars| chars.into_iter().collect())
    }

    prop_compose! {
        fn skill_condition_strategy()
            (on_timer in any::<bool>(),
             on_condition in any::<bool>(),
             mob_presence in prop::sample::select(vec![
                MuHelperMobPresence::Nearby,
                MuHelperMobPresence::Attacking,
             ]),
             mob_count in prop::sample::select(vec![
                MuHelperMobCount::Two,
                MuHelperMobCount::Three,
                MuHelperMobCount::Four,
                MuHelperMobCount::Five,
             ]))
            -> MuHelperSkillCondition {
                MuHelperSkillCondition {
                    on_timer,
                    on_condition,
                    mob_presence,
                    mob_count,
                }
            }
    }

    prop_compose! {
        fn helper_config_strategy()
            (hunting_range in 0u8..=MU_HELPER_MAX_HUNTING_RANGE,
             obtaining_range in 1u8..=MU_HELPER_MAX_OBTAINING_RANGE,
             max_seconds_away in 0u8..=15,
             basic_skill_id in any::<u16>(),
             activation_1_skill_id in any::<u16>(),
             activation_1_interval_seconds in any::<u16>(),
             activation_2_skill_id in any::<u16>(),
             activation_2_interval_seconds in any::<u16>(),
             activation_1_condition in skill_condition_strategy(),
             heal_threshold in threshold_strategy(),
             heal_party_threshold in threshold_strategy(),
             potion_threshold in threshold_strategy(),
             dark_raven_mode in prop::sample::select(vec![
                MuHelperDarkRavenMode::Cease,
                MuHelperDarkRavenMode::Auto,
                MuHelperDarkRavenMode::Together,
             ]),
             extra_items in prop::collection::btree_set(extra_item_strategy(), 0..=MU_HELPER_MAX_EXTRA_ITEMS))
            -> MuHelperConfig {
                let mut config = MuHelperConfig::default();
                config.hunting_range = hunting_range;
                config.obtaining_range = obtaining_range;
                config.max_seconds_away = max_seconds_away;
                config.basic_skill_id = basic_skill_id;
                config.activation_skills = [
                    MuHelperSkillSlot {
                        skill_id: activation_1_skill_id,
                        interval_seconds: activation_1_interval_seconds,
                        condition: activation_1_condition,
                    },
                    MuHelperSkillSlot {
                        skill_id: activation_2_skill_id,
                        interval_seconds: activation_2_interval_seconds,
                        condition: MuHelperSkillCondition::default(),
                    },
                ];
                config.heal_threshold = heal_threshold;
                config.heal_party_threshold = heal_party_threshold;
                config.potion_threshold = potion_threshold;
                config.dark_raven_mode = dark_raven_mode;
                config.extra_items = extra_items;
                config.pick_extra_items = !config.extra_items.is_empty();
                config
            }
    }

    #[test]
    fn default_config_matches_legacy_reset_values() {
        let config = MuHelperConfig::default();

        assert_eq!(config.hunting_range, MU_HELPER_MAX_HUNTING_RANGE);
        assert_eq!(config.obtaining_range, MU_HELPER_MAX_OBTAINING_RANGE);
        assert_eq!(config.max_seconds_away, 10);
        assert!(config.return_to_original_position);
        assert!(config.buff_duration);
        assert!(config.buff_duration_party);
        assert_eq!(config.heal_threshold, 60);
        assert_eq!(config.potion_threshold, 40);
        assert_eq!(config.heal_party_threshold, 60);
        assert_eq!(config.dark_raven_mode, MuHelperDarkRavenMode::Cease);
    }

    #[test]
    fn config_validation_rejects_legacy_limit_violations() {
        let config = MuHelperConfig {
            hunting_range: 7,
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(MuHelperConfigError::HuntingRangeOutOfRange { .. })
        ));

        let config = MuHelperConfig {
            obtaining_range: 9,
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(MuHelperConfigError::ObtainingRangeOutOfRange { .. })
        ));

        let config = MuHelperConfig {
            heal_threshold: 55,
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(MuHelperConfigError::ThresholdOutOfRange { field: "heal", .. })
        ));

        let config = MuHelperConfig {
            extra_items: (0..=MU_HELPER_MAX_EXTRA_ITEMS)
                .map(|index| format!("item-{index}"))
                .collect::<BTreeSet<_>>(),
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(MuHelperConfigError::TooManyExtraItems { .. })
        ));
    }

    #[test]
    fn packet_payload_roundtrips_default_config() {
        let config = MuHelperConfig::default();
        let payload = config.to_packet_payload().unwrap();
        let request = mu_helper_save_data_request(payload).unwrap();

        assert_eq!(payload.len(), MU_HELPER_PACKET_PAYLOAD_SIZE);
        assert_eq!(&request[..4], &[0xC2, 0x01, 0x05, 0xAE]);
        assert_eq!(&request[4..], &payload);
        let decoded = MuHelperConfig::from_packet_payload(&payload);
        assert_eq!(decoded.hunting_range, config.hunting_range);
        assert_eq!(decoded.obtaining_range, config.obtaining_range);
        assert_eq!(decoded.max_seconds_away, config.max_seconds_away);
        assert_eq!(decoded.basic_skill_id, config.basic_skill_id);
        assert_eq!(decoded.buff_skill_ids, config.buff_skill_ids);
        assert_eq!(decoded.extra_items, config.extra_items);
        assert!(!decoded.use_self_defense);
    }

    proptest! {
        #[test]
        fn packet_payload_roundtrips_valid_configs(config in helper_config_strategy()) {
            let payload = config.to_packet_payload().unwrap();
            let decoded = MuHelperConfig::from_packet_payload(&payload);

            prop_assert_eq!(decoded.hunting_range, config.hunting_range);
            prop_assert_eq!(decoded.obtaining_range, config.obtaining_range);
            prop_assert_eq!(decoded.max_seconds_away, config.max_seconds_away);
            prop_assert_eq!(decoded.basic_skill_id, config.basic_skill_id);
            prop_assert_eq!(decoded.activation_skills[0].skill_id, config.activation_skills[0].skill_id);
            prop_assert_eq!(decoded.activation_skills[0].interval_seconds, config.activation_skills[0].interval_seconds);
            prop_assert_eq!(decoded.activation_skills[0].condition, config.activation_skills[0].condition);
            prop_assert_eq!(decoded.activation_skills[1].skill_id, config.activation_skills[1].skill_id);
            prop_assert_eq!(decoded.activation_skills[1].interval_seconds, config.activation_skills[1].interval_seconds);
            prop_assert_eq!(decoded.activation_skills[1].condition, config.activation_skills[1].condition);
            prop_assert_eq!(decoded.heal_threshold, config.heal_threshold);
            prop_assert_eq!(decoded.heal_party_threshold, config.heal_party_threshold);
            prop_assert_eq!(decoded.potion_threshold, config.potion_threshold);
            prop_assert_eq!(decoded.dark_raven_mode, config.dark_raven_mode);
            prop_assert_eq!(decoded.pick_extra_items, config.pick_extra_items);
            prop_assert_eq!(decoded.extra_items, config.extra_items);
            prop_assert!(!decoded.use_self_defense);
        }
    }
}
