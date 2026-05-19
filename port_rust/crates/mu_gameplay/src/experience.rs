pub const NORMAL_EXPERIENCE_LEVEL_BONUS: u64 = 9;
pub const NORMAL_EXPERIENCE_SCALE: u64 = 10;
pub const OVERLEVEL_EXPERIENCE_THRESHOLD: u16 = 255;
pub const OVERLEVEL_EXPERIENCE_SCALE: u64 = 1000;

pub const MASTER_LEVEL_EXPERIENCE_OFFSET: i64 = 3_892_250_000;
pub const MASTER_LEVEL_EXPONENT_BONUS: i64 = 9;
pub const MASTER_LEVEL_EXPONENT_SCALE: i64 = 10;
pub const MASTER_LEVEL_OVERFLOW_SCALE: i64 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExperienceBand {
    pub lower_bound: u64,
    pub upper_bound: u64,
}

impl ExperienceBand {
    pub const fn new(lower_bound: u64, upper_bound: u64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }

    pub fn for_level(level: u16) -> Self {
        Self::new(
            previous_experience_for_level(level),
            next_experience_for_level(level),
        )
    }

    pub fn for_master_level(character_level: u16, master_level: i16) -> Self {
        let lower_bound = previous_master_level_experience(character_level, master_level).max(0);
        let upper_bound = next_master_level_experience(character_level, master_level).max(0);

        Self::new(lower_bound as u64, upper_bound.max(lower_bound) as u64)
    }

    pub fn clamped_experience(self, experience: u64) -> u64 {
        experience.clamp(self.lower_bound, self.upper_bound)
    }

    pub fn ratio(self, experience: u64) -> f64 {
        let span = self.upper_bound.saturating_sub(self.lower_bound);
        if span == 0 {
            return 0.0;
        }

        let clamped = self.clamped_experience(experience);
        (clamped - self.lower_bound) as f64 / span as f64
    }

    pub fn contains(self, experience: u64) -> bool {
        experience >= self.lower_bound && experience <= self.upper_bound
    }
}

pub fn next_experience_for_level(level: u16) -> u64 {
    let level = u64::from(level);
    let mut next_experience =
        (NORMAL_EXPERIENCE_LEVEL_BONUS + level) * level * level * NORMAL_EXPERIENCE_SCALE;

    if level > u64::from(OVERLEVEL_EXPERIENCE_THRESHOLD) {
        let over_level = level - u64::from(OVERLEVEL_EXPERIENCE_THRESHOLD);
        next_experience += (NORMAL_EXPERIENCE_LEVEL_BONUS + over_level)
            * over_level
            * over_level
            * OVERLEVEL_EXPERIENCE_SCALE;
    }

    next_experience
}

pub fn previous_experience_for_level(level: u16) -> u64 {
    level.checked_sub(1).map_or(0, next_experience_for_level)
}

pub fn next_master_level_experience(character_level: u16, master_level: i16) -> i64 {
    let total_level = i64::from(character_level) + i64::from(master_level) + 1;
    let over_level = total_level - 255;
    let master_experience = ((MASTER_LEVEL_EXPONENT_BONUS + total_level)
        * total_level
        * total_level
        * MASTER_LEVEL_EXPONENT_SCALE)
        + ((MASTER_LEVEL_EXPONENT_BONUS + over_level)
            * over_level
            * over_level
            * MASTER_LEVEL_OVERFLOW_SCALE);

    (master_experience - MASTER_LEVEL_EXPERIENCE_OFFSET) / 2
}

pub fn previous_master_level_experience(character_level: u16, master_level: i16) -> i64 {
    if master_level <= 0 {
        return 0;
    }

    next_master_level_experience(character_level, master_level - 1)
}

#[cfg(test)]
mod tests {
    use super::{
        next_experience_for_level, next_master_level_experience, previous_experience_for_level,
        previous_master_level_experience, ExperienceBand,
    };

    #[test]
    fn normal_experience_curve_matches_the_legacy_formula() {
        assert_eq!(next_experience_for_level(1), 100);
        assert_eq!(next_experience_for_level(255), 171_666_000);
        assert_eq!(next_experience_for_level(256), 173_680_400);
    }

    #[test]
    fn experience_band_uses_the_previous_level_threshold_as_lower_bound() {
        let band = ExperienceBand::for_level(256);

        assert_eq!(band.lower_bound, 171_666_000);
        assert_eq!(band.upper_bound, 173_680_400);
        assert!(band.contains(171_666_000));
        assert!(band.contains(173_680_400));
        assert_eq!(band.ratio(172_673_200), 0.5);
    }

    #[test]
    fn master_level_curve_matches_the_legacy_formula() {
        assert_eq!(next_master_level_experience(400, 0), 35_507_050);
    }

    #[test]
    fn master_experience_band_reuses_the_previous_threshold() {
        let band = ExperienceBand::for_master_level(400, 12);

        assert_eq!(
            band.lower_bound,
            previous_master_level_experience(400, 12) as u64
        );
        assert_eq!(
            band.upper_bound,
            next_master_level_experience(400, 12) as u64
        );
    }

    #[test]
    fn previous_threshold_helpers_return_zero_for_the_first_level() {
        assert_eq!(previous_experience_for_level(1), 0);
        assert_eq!(previous_master_level_experience(400, 0), 0);
    }
}
