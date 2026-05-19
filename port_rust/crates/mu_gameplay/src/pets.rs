use bevy::prelude::{App, Plugin, Resource};

use crate::items::EquipmentSlot;

const DARK_SPIRIT_BASE_DAMAGE_MIN: u16 = 180;
const DARK_SPIRIT_BASE_DAMAGE_MAX: u16 = 200;
const DARK_SPIRIT_BASE_ATTACK_SPEED: u16 = 20;
const DARK_SPIRIT_BASE_ATTACK_SUCCESS: u16 = 1000;

const DARK_HORSE_BASE_DAMAGE_BONUS: u16 = 5;
const DARK_HORSE_BASE_ATTACK_SPEED: u16 = 20;
const DARK_HORSE_BASE_ATTACK_SUCCESS: u16 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PetKind {
    DarkSpirit = 0,
    DarkHorse = 1,
}

impl PetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DarkSpirit => "dark-spirit",
            Self::DarkHorse => "dark-horse",
        }
    }

    pub fn from_packet_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::DarkSpirit),
            1 => Some(Self::DarkHorse),
            _ => None,
        }
    }

    pub fn as_packet_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PetCommandMode {
    #[default]
    Normal = 0,
    AttackRandom = 1,
    AttackWithOwner = 2,
    AttackTarget = 3,
}

impl PetCommandMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::AttackRandom => "attack-random",
            Self::AttackWithOwner => "attack-owner",
            Self::AttackTarget => "attack-target",
        }
    }

    pub fn from_packet_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Normal),
            1 => Some(Self::AttackRandom),
            2 => Some(Self::AttackWithOwner),
            3 => Some(Self::AttackTarget),
            _ => None,
        }
    }

    pub fn as_packet_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PetInfo {
    pub kind: Option<PetKind>,
    pub experience_current: u32,
    pub experience_next: u32,
    pub level: u16,
    pub life: u16,
    pub damage_min: u16,
    pub damage_max: u16,
    pub attack_speed: u16,
    pub attack_success: u16,
}

impl PetInfo {
    pub fn new(kind: PetKind) -> Self {
        Self {
            kind: Some(kind),
            ..Self::default()
        }
    }

    pub fn with_kind(mut self, kind: PetKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn with_level(mut self, level: u16) -> Self {
        self.level = level;
        self
    }

    pub fn with_experience_current(mut self, experience_current: u32) -> Self {
        self.experience_current = experience_current;
        self
    }

    pub fn with_experience_next(mut self, experience_next: u32) -> Self {
        self.experience_next = experience_next;
        self
    }

    pub fn with_life(mut self, life: u16) -> Self {
        self.life = life;
        self
    }

    pub fn summary(self) -> String {
        let kind = self.kind.map_or("none", PetKind::as_str);

        format!(
            "kind={}|level={}|exp={}/{}|life={}|damage={}-{}|speed={}|success={}",
            kind,
            self.level,
            self.experience_current,
            self.experience_next,
            self.life,
            self.damage_min,
            self.damage_max,
            self.attack_speed,
            self.attack_success,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetCommandState {
    pub kind: PetKind,
    pub mode: PetCommandMode,
    pub target_key: u16,
}

impl PetCommandState {
    pub fn summary(self) -> String {
        format!(
            "kind={}|mode={}|target={}",
            self.kind.as_str(),
            self.mode.as_str(),
            self.target_key,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetAttackState {
    pub kind: PetKind,
    pub skill_type: u8,
    pub source_key: u16,
    pub target_key: u16,
}

impl PetAttackState {
    pub fn summary(self) -> String {
        format!(
            "kind={}|skill={}|source={}|target={}",
            self.kind.as_str(),
            self.skill_type,
            self.source_key,
            self.target_key,
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PetManager {
    hovered: PetInfo,
    dark_spirit: PetInfo,
    dark_horse: PetInfo,
    command: Option<PetCommandState>,
    attack: Option<PetAttackState>,
}

impl Resource for PetManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct PetPlugin;

impl Plugin for PetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PetManager>();
    }
}

impl PetManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn hovered_info(&self) -> &PetInfo {
        &self.hovered
    }

    pub fn dark_spirit_info(&self) -> &PetInfo {
        &self.dark_spirit
    }

    pub fn dark_horse_info(&self) -> &PetInfo {
        &self.dark_horse
    }

    pub fn command(&self) -> Option<&PetCommandState> {
        self.command.as_ref()
    }

    pub fn attack(&self) -> Option<&PetAttackState> {
        self.attack.as_ref()
    }

    pub fn pet_info(&self, kind: PetKind) -> &PetInfo {
        match kind {
            PetKind::DarkSpirit => &self.dark_spirit,
            PetKind::DarkHorse => &self.dark_horse,
        }
    }

    pub fn set_hovered_info(&mut self, info: PetInfo) {
        self.hovered = info;
    }

    pub fn set_equipped_info(&mut self, kind: PetKind, mut info: PetInfo) {
        info.kind = Some(kind);
        match kind {
            PetKind::DarkSpirit => self.dark_spirit = info,
            PetKind::DarkHorse => self.dark_horse = info,
        }
    }

    pub fn apply_pet_info_response(&mut self, slot: Option<EquipmentSlot>, info: PetInfo) {
        self.hovered = info;

        match slot {
            Some(EquipmentSlot::Helper) => self.set_equipped_info(PetKind::DarkHorse, info),
            Some(EquipmentSlot::WeaponLeft) => self.set_equipped_info(PetKind::DarkSpirit, info),
            _ => {}
        }
    }

    pub fn set_command(&mut self, kind: PetKind, mode: PetCommandMode, target_key: u16) {
        self.command = Some(PetCommandState {
            kind,
            mode,
            target_key,
        });
    }

    pub fn clear_command(&mut self) {
        self.command = None;
    }

    pub fn set_attack(&mut self, kind: PetKind, skill_type: u8, source_key: u16, target_key: u16) {
        self.attack = Some(PetAttackState {
            kind,
            skill_type,
            source_key,
            target_key,
        });
    }

    pub fn clear_attack(&mut self) {
        self.attack = None;
    }

    pub fn snapshot(&self) -> String {
        let hovered = self.hovered.summary();
        let dark_spirit = self.dark_spirit.summary();
        let dark_horse = self.dark_horse.summary();
        let command = self
            .command
            .map_or_else(|| "none".to_string(), PetCommandState::summary);
        let attack = self
            .attack
            .map_or_else(|| "none".to_string(), PetAttackState::summary);

        format!(
            "hovered={}|dark-spirit={}|dark-horse={}|command={}|attack={}",
            hovered, dark_spirit, dark_horse, command, attack
        )
    }
}

pub fn calculate_pet_info(mut info: PetInfo, charisma: u16, strength: u16) -> PetInfo {
    let Some(kind) = info.kind else {
        return info;
    };

    let level = u32::from(info.level) + 1;
    info.experience_next = (10 + level) * level * level * level * 100;

    match kind {
        PetKind::DarkSpirit => {
            info.damage_min = DARK_SPIRIT_BASE_DAMAGE_MIN + info.level * 15 + charisma / 8;
            info.damage_max = DARK_SPIRIT_BASE_DAMAGE_MAX + info.level * 15 + charisma / 4;
            info.attack_speed =
                DARK_SPIRIT_BASE_ATTACK_SPEED + (info.level * 4 / 5) + charisma / 50;
            info.attack_success = DARK_SPIRIT_BASE_ATTACK_SUCCESS + info.level + (info.level * 15);
        }
        PetKind::DarkHorse => {
            info.damage_min =
                (strength / 10) + (charisma / 10) + (info.level * DARK_HORSE_BASE_DAMAGE_BONUS);
            info.damage_max = info.damage_min + (info.damage_min / 2);
            info.attack_speed = DARK_HORSE_BASE_ATTACK_SPEED + (info.level * 4 / 5) + charisma / 50;
            info.attack_success = DARK_HORSE_BASE_ATTACK_SUCCESS + info.level + (info.level * 15);
        }
    }

    info
}

pub fn pet_item_value(info: &PetInfo) -> u32 {
    match info.kind {
        Some(PetKind::DarkHorse) => u32::from(info.level) * 2_000_000,
        Some(PetKind::DarkSpirit) => u32::from(info.level) * 1_000_000,
        None => 0,
    }
}

pub fn dark_horse_level_requirement(level: u16) -> u16 {
    218 + (level * 2)
}

pub fn dark_spirit_charisma_requirement(level: u16) -> u16 {
    185 + (level * 15)
}

pub fn dark_horse_defence_bonus(level: u16, dexterity: u16) -> u8 {
    (5 + (dexterity / 20) + (level * 2)) as u8
}

#[cfg(test)]
mod tests {
    use super::{
        calculate_pet_info, dark_horse_defence_bonus, dark_horse_level_requirement,
        dark_spirit_charisma_requirement, pet_item_value, PetCommandMode, PetInfo, PetKind,
        PetManager, PetPlugin,
    };
    use crate::items::EquipmentSlot;
    use bevy::prelude::App;

    #[test]
    fn pet_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(PetPlugin);

        let manager = app.world().resource::<PetManager>();
        let empty = PetInfo::default().summary();
        assert_eq!(
            manager.snapshot(),
            format!(
                "hovered={}|dark-spirit={}|dark-horse={}|command=none|attack=none",
                empty, empty, empty
            )
        );
    }

    #[test]
    fn pet_calculations_match_the_legacy_dark_spirit_and_dark_horse_rules() {
        let spirit = calculate_pet_info(
            PetInfo::new(PetKind::DarkSpirit)
                .with_level(12)
                .with_experience_current(98_765)
                .with_life(255),
            450,
            280,
        );
        assert_eq!(spirit.experience_next, 5_053_100);
        assert_eq!(spirit.damage_min, 416);
        assert_eq!(spirit.damage_max, 492);
        assert_eq!(spirit.attack_speed, 38);
        assert_eq!(spirit.attack_success, 1_192);
        assert_eq!(dark_spirit_charisma_requirement(12), 365);

        let horse = calculate_pet_info(
            PetInfo::new(PetKind::DarkHorse)
                .with_level(12)
                .with_experience_current(98_765)
                .with_life(255),
            450,
            280,
        );
        assert_eq!(horse.experience_next, 5_053_100);
        assert_eq!(horse.damage_min, 133);
        assert_eq!(horse.damage_max, 199);
        assert_eq!(horse.attack_speed, 38);
        assert_eq!(horse.attack_success, 1_192);
        assert_eq!(dark_horse_level_requirement(12), 242);
        assert_eq!(dark_horse_defence_bonus(12, 280), 43);
        assert_eq!(pet_item_value(&horse), 24_000_000);
        assert_eq!(pet_item_value(&spirit), 12_000_000);
    }

    #[test]
    fn pet_manager_tracks_hovered_equipped_command_and_attack_state() {
        let mut manager = PetManager::new();
        let info = calculate_pet_info(
            PetInfo::new(PetKind::DarkHorse)
                .with_level(12)
                .with_experience_current(98_765)
                .with_life(255),
            450,
            280,
        );

        manager.apply_pet_info_response(Some(EquipmentSlot::Helper), info);
        manager.set_command(PetKind::DarkHorse, PetCommandMode::AttackTarget, 4_660);
        manager.set_attack(PetKind::DarkSpirit, 2, 258, 772);

        assert_eq!(manager.hovered_info(), &info);
        assert_eq!(
            manager.dark_horse_info(),
            &info.with_kind(PetKind::DarkHorse)
        );
        assert_eq!(manager.dark_spirit_info().kind, None);
        let empty = PetInfo::default().summary();
        assert_eq!(
            manager.command(),
            Some(&super::PetCommandState {
                kind: PetKind::DarkHorse,
                mode: PetCommandMode::AttackTarget,
                target_key: 4_660,
            })
        );
        assert_eq!(
            manager.attack(),
            Some(&super::PetAttackState {
                kind: PetKind::DarkSpirit,
                skill_type: 2,
                source_key: 258,
                target_key: 772,
            })
        );
        assert_eq!(
            manager.snapshot(),
            format!(
                "hovered=kind=dark-horse|level=12|exp=98765/5053100|life=255|damage=133-199|speed=38|success=1192|dark-spirit={}|dark-horse=kind=dark-horse|level=12|exp=98765/5053100|life=255|damage=133-199|speed=38|success=1192|command=kind=dark-horse|mode=attack-target|target=4660|attack=kind=dark-spirit|skill=2|source=258|target=772",
                empty
            )
        );
    }
}
