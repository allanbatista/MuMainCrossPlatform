use crate::classes::MASTER_EXPERIENCE_UNLOCK_LEVEL;
pub use crate::experience::next_master_level_experience;
const MASTER_LEVEL_UNLOCK_LEVEL: u16 = MASTER_EXPERIENCE_UNLOCK_LEVEL;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MasterLevelState {
    pub level: i16,
    pub current_experience: i64,
    pub next_experience: i64,
    pub add_points: i16,
    pub level_up_points: i16,
    pub total_points: i16,
    pub max_points: i16,
    pub max_life: u32,
    pub max_mana: u32,
    pub max_shield: u32,
    pub max_bp: u32,
}

impl MasterLevelState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(level: i16) -> Self {
        Self {
            level,
            ..Self::default()
        }
    }

    pub fn recalculate_next_experience(&mut self, character_level: u16) {
        self.current_experience = self.next_experience;
        self.next_experience = next_master_level_experience(character_level, self.level);
    }

    pub fn is_enabled_for_level(class_level: u16) -> bool {
        class_level >= MASTER_LEVEL_UNLOCK_LEVEL
    }
}

#[cfg(test)]
mod tests {
    use super::{next_master_level_experience, MasterLevelState};

    #[test]
    fn master_level_curve_matches_the_legacy_formula() {
        assert_eq!(next_master_level_experience(400, 0), 35_507_050);
    }

    #[test]
    fn recalculation_moves_next_experience_into_current_experience() {
        let mut state = MasterLevelState {
            level: 12,
            current_experience: 0,
            next_experience: 123,
            add_points: 0,
            level_up_points: 0,
            total_points: 0,
            max_points: 0,
            max_life: 0,
            max_mana: 0,
            max_shield: 0,
            max_bp: 0,
        };

        state.recalculate_next_experience(400);

        assert_eq!(state.current_experience, 123);
        assert_eq!(state.next_experience, next_master_level_experience(400, 12));
    }

    #[test]
    fn unlock_threshold_matches_the_legacy_master_level_gate() {
        assert!(!MasterLevelState::is_enabled_for_level(399));
        assert!(MasterLevelState::is_enabled_for_level(400));
    }
}
