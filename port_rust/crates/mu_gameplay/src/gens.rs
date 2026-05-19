use bevy::prelude::{App, Plugin, Resource};

pub const MAX_GENS_TEAM_NAME_LENGTH: usize = 10;
pub const MAX_GENS_TITLE_LENGTH: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GensMode {
    #[default]
    Idle,
    Joining,
    Leaving,
    Rewarding,
    Ranking,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GensType {
    #[default]
    None,
    Duprian,
    Vanert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GensManager {
    mode: GensMode,
    gens_type: GensType,
    contribution: u32,
    next_contribution: u32,
    ranking: Option<u16>,
    team_name: String,
    title_name: String,
    reward_available: bool,
}

impl Default for GensManager {
    fn default() -> Self {
        Self {
            mode: GensMode::Idle,
            gens_type: GensType::None,
            contribution: 0,
            next_contribution: 0,
            ranking: None,
            team_name: String::new(),
            title_name: String::new(),
            reward_available: false,
        }
    }
}

impl Resource for GensManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct GensPlugin;

impl Plugin for GensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GensManager>();
    }
}

impl GensManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> GensMode {
        self.mode
    }

    pub fn gens_type(&self) -> GensType {
        self.gens_type
    }

    pub fn contribution(&self) -> u32 {
        self.contribution
    }

    pub fn next_contribution(&self) -> u32 {
        self.next_contribution
    }

    pub fn ranking(&self) -> Option<u16> {
        self.ranking
    }

    pub fn team_name(&self) -> &str {
        self.team_name.as_str()
    }

    pub fn title_name(&self) -> &str {
        self.title_name.as_str()
    }

    pub fn reward_available(&self) -> bool {
        self.reward_available
    }

    pub fn set_gens_type(&mut self, gens_type: GensType) {
        self.gens_type = gens_type;
    }

    pub fn set_contribution(&mut self, contribution: u32) {
        self.contribution = contribution;
    }

    pub fn set_next_contribution(&mut self, next_contribution: u32) {
        self.next_contribution = next_contribution;
    }

    pub fn set_ranking(&mut self, ranking: Option<u16>) {
        self.ranking = ranking;
    }

    pub fn set_team_name(&mut self, team_name: impl Into<String>) {
        self.team_name = truncate_text(team_name.into(), MAX_GENS_TEAM_NAME_LENGTH);
    }

    pub fn set_title_name(&mut self, title_name: impl Into<String>) {
        self.title_name = truncate_text(title_name.into(), MAX_GENS_TITLE_LENGTH);
    }

    pub fn set_reward_available(&mut self, reward_available: bool) {
        self.reward_available = reward_available;
    }

    pub fn mark_joining(&mut self) {
        self.mode = GensMode::Joining;
    }

    pub fn mark_leaving(&mut self) {
        self.mode = GensMode::Leaving;
    }

    pub fn mark_rewarding(&mut self) {
        self.mode = GensMode::Rewarding;
    }

    pub fn mark_ranking(&mut self) {
        self.mode = GensMode::Ranking;
    }

    pub fn mark_error(&mut self) {
        self.mode = GensMode::Error;
    }

    pub fn clear_error(&mut self) {
        self.mode = GensMode::Idle;
    }
}

fn truncate_text(value: String, max_len: usize) -> String {
    value.chars().take(max_len).collect()
}

#[cfg(test)]
mod tests {
    use super::{GensManager, GensMode, GensPlugin, GensType, MAX_GENS_TEAM_NAME_LENGTH};
    use bevy::prelude::App;

    #[test]
    fn gens_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(GensPlugin);

        let manager = app.world().resource::<GensManager>();
        assert_eq!(manager.mode(), GensMode::Idle);
        assert_eq!(manager.gens_type(), GensType::None);
    }

    #[test]
    fn gens_manager_tracks_rank_and_truncates_names() {
        let mut manager = GensManager::new();
        manager.set_gens_type(GensType::Duprian);
        manager.set_team_name("Duprian Legionary");
        manager.set_title_name("Sentinel of the Ancient Gens Covenant");
        manager.set_contribution(4_300);
        manager.set_next_contribution(5_000);
        manager.set_ranking(Some(12));
        manager.set_reward_available(true);
        manager.mark_ranking();

        assert_eq!(manager.mode(), GensMode::Ranking);
        assert_eq!(manager.gens_type(), GensType::Duprian);
        assert_eq!(
            manager.team_name().chars().count(),
            MAX_GENS_TEAM_NAME_LENGTH
        );
        assert_eq!(manager.title_name().chars().count(), 32);
        assert_eq!(manager.contribution(), 4_300);
        assert_eq!(manager.next_contribution(), 5_000);
        assert_eq!(manager.ranking(), Some(12));
        assert!(manager.reward_available());
    }
}
