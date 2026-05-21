use crate::classes::CharacterClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DamageRange {
    pub min: i32,
    pub max: i32,
}

impl DamageRange {
    pub const fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatProfile {
    pub class_: CharacterClass,
    pub magic_damage: DamageRange,
    pub curse_damage: DamageRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CombatBonuses {
    pub magic_power_percent: i32,
    pub mastery_bonus: i32,
    pub skill_attack_bonus: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkillDamageProfile {
    pub damage: i32,
    pub mastery_type: u8,
    pub is_summon_skill: bool,
}

pub fn magic_skill_damage(
    profile: &CombatProfile,
    skill: &SkillDamageProfile,
    bonuses: &CombatBonuses,
) -> DamageRange {
    if skill.is_summon_skill {
        return profile.magic_damage;
    }

    let damage = skill_damage_range(profile.magic_damage, skill.damage);
    let scaled = scale_range_by_percent(damage, bonuses.magic_power_percent);
    add_flat_bonus(scaled, bonuses.mastery_bonus + bonuses.skill_attack_bonus)
}

pub fn curse_skill_damage(
    profile: &CombatProfile,
    skill: &SkillDamageProfile,
) -> Option<DamageRange> {
    if profile.class_.base_class() != CharacterClass::Summoner {
        return None;
    }

    if skill.is_summon_skill {
        return Some(skill_damage_range(profile.curse_damage, skill.damage));
    }

    Some(profile.curse_damage)
}

pub fn skill_damage(skill: &SkillDamageProfile, bonuses: &CombatBonuses) -> DamageRange {
    let damage = DamageRange::new(skill.damage, skill.damage + skill.damage / 2);
    add_flat_bonus(damage, bonuses.mastery_bonus + bonuses.skill_attack_bonus)
}

fn skill_damage_range(base: DamageRange, damage: i32) -> DamageRange {
    DamageRange::new(base.min + damage, base.max + damage + damage / 2)
}

fn scale_range_by_percent(range: DamageRange, percent: i32) -> DamageRange {
    DamageRange::new(
        scale_by_percent(range.min, percent),
        scale_by_percent(range.max, percent),
    )
}

fn scale_by_percent(value: i32, percent: i32) -> i32 {
    let scaled = i64::from(value) * i64::from(100 + percent);
    (scaled / 100) as i32
}

fn add_flat_bonus(range: DamageRange, bonus: i32) -> DamageRange {
    DamageRange::new(range.min + bonus, range.max + bonus)
}

#[cfg(test)]
mod tests {
    use super::{
        add_flat_bonus, magic_skill_damage, skill_damage, CombatBonuses, CombatProfile,
        DamageRange, SkillDamageProfile,
    };
    use crate::classes::CharacterClass;

    #[test]
    fn magic_skill_damage_applies_flat_and_percent_bonuses() {
        let profile = CombatProfile {
            class_: CharacterClass::Wizard,
            magic_damage: DamageRange::new(100, 200),
            curse_damage: DamageRange::new(0, 0),
        };
        let skill = SkillDamageProfile {
            damage: 50,
            mastery_type: 3,
            is_summon_skill: false,
        };
        let bonuses = CombatBonuses {
            magic_power_percent: 20,
            mastery_bonus: 7,
            skill_attack_bonus: 3,
        };

        assert_eq!(
            magic_skill_damage(&profile, &skill, &bonuses),
            DamageRange::new(190, 340)
        );
    }

    #[test]
    fn summon_magic_skills_keep_the_base_character_range() {
        let profile = CombatProfile {
            class_: CharacterClass::Wizard,
            magic_damage: DamageRange::new(100, 200),
            curse_damage: DamageRange::new(0, 0),
        };
        let skill = SkillDamageProfile {
            damage: 50,
            mastery_type: 3,
            is_summon_skill: true,
        };
        let bonuses = CombatBonuses {
            magic_power_percent: 20,
            mastery_bonus: 7,
            skill_attack_bonus: 3,
        };

        assert_eq!(
            magic_skill_damage(&profile, &skill, &bonuses),
            profile.magic_damage
        );
    }

    #[test]
    fn curse_skill_damage_only_applies_to_summoners() {
        let knight = CombatProfile {
            class_: CharacterClass::Knight,
            magic_damage: DamageRange::new(0, 0),
            curse_damage: DamageRange::new(80, 120),
        };
        let summoner = CombatProfile {
            class_: CharacterClass::Summoner,
            magic_damage: DamageRange::new(0, 0),
            curse_damage: DamageRange::new(80, 120),
        };
        let summon_skill = SkillDamageProfile {
            damage: 30,
            mastery_type: 0,
            is_summon_skill: true,
        };

        assert_eq!(super::curse_skill_damage(&knight, &summon_skill), None);
        assert_eq!(
            super::curse_skill_damage(&summoner, &summon_skill),
            Some(DamageRange::new(110, 165))
        );
    }

    #[test]
    fn generic_skill_damage_applies_flat_bonuses() {
        let skill = SkillDamageProfile {
            damage: 40,
            mastery_type: 7,
            is_summon_skill: false,
        };
        let bonuses = CombatBonuses {
            magic_power_percent: 0,
            mastery_bonus: 7,
            skill_attack_bonus: 3,
        };

        assert_eq!(skill_damage(&skill, &bonuses), DamageRange::new(50, 70));
    }

    #[test]
    fn flat_bonus_helper_keeps_the_damage_span_intact() {
        let range = DamageRange::new(5, 9);

        assert_eq!(add_flat_bonus(range, 4), DamageRange::new(9, 13));
    }
}
