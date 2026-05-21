use bevy::prelude::{App, Plugin, Resource};

pub const MAX_USERNAME_SIZE: usize = 10;
pub const MAX_DUEL_PLAYERS: usize = 2;
pub const MAX_DUEL_CHANNELS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuelCharacterSnapshot {
    pub key: i16,
    pub id: String,
    pub owner_id: String,
    pub base_class: u8,
}

impl DuelCharacterSnapshot {
    pub fn is_summon(&self) -> bool {
        self.base_class == 0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DuelPlayerInfo {
    pub index: i16,
    pub id: String,
    pub score: i32,
    pub hp_rate: f32,
    pub sd_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DuelChannelInfo {
    pub enabled: bool,
    pub joinable: bool,
    pub player_one_id: String,
    pub player_two_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DuelManager {
    duel_enabled: bool,
    pet_duel_enabled: bool,
    players: [DuelPlayerInfo; MAX_DUEL_PLAYERS],
    channels: [DuelChannelInfo; MAX_DUEL_CHANNELS],
    current_channel: i32,
    fighter_regenerated: bool,
    watch_users: Vec<String>,
}

impl Default for DuelManager {
    fn default() -> Self {
        Self {
            duel_enabled: false,
            pet_duel_enabled: false,
            players: std::array::from_fn(|_| DuelPlayerInfo::default()),
            channels: std::array::from_fn(|_| DuelChannelInfo::default()),
            current_channel: -1,
            fighter_regenerated: false,
            watch_users: Vec::new(),
        }
    }
}

impl Resource for DuelManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DuelManager>();
    }
}

impl DuelManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn duel_enabled(&self) -> bool {
        self.duel_enabled
    }

    pub fn pet_duel_enabled(&self) -> bool {
        self.pet_duel_enabled
    }

    pub fn enable_duel(&mut self, enabled: bool) {
        if !enabled {
            self.reset();
            return;
        }

        self.duel_enabled = true;
    }

    pub fn enable_pet_duel(&mut self, enabled: bool) {
        self.pet_duel_enabled = enabled;
    }

    pub fn player(&self, player_num: usize) -> &DuelPlayerInfo {
        self.players
            .get(player_num)
            .expect("duel player index out of range")
    }

    pub fn players(&self) -> &[DuelPlayerInfo] {
        &self.players
    }

    pub fn channel(&self, channel_index: usize) -> &DuelChannelInfo {
        self.channels
            .get(channel_index)
            .expect("duel channel index out of range")
    }

    pub fn channels(&self) -> &[DuelChannelInfo] {
        &self.channels
    }

    pub fn set_duel_player(&mut self, player_num: usize, index: i16, id: Option<&str>) {
        let player = self.player_mut(player_num);
        player.index = index;
        player.id = normalize_username(id);
    }

    pub fn set_hero_as_duel_player(&mut self, player_num: usize, hero_key: i16, hero_id: &str) {
        self.set_duel_player(player_num, hero_key, Some(hero_id));
    }

    pub fn set_score(&mut self, player_num: usize, score: i32) {
        self.player_mut(player_num).score = score;
    }

    pub fn set_hp(&mut self, player_num: usize, rate_percent: i32) {
        self.player_mut(player_num).hp_rate = rate_percent as f32 * 0.01;
    }

    pub fn set_sd(&mut self, player_num: usize, rate_percent: i32) {
        self.player_mut(player_num).sd_rate = rate_percent as f32 * 0.01;
    }

    pub fn is_duel_player(
        &self,
        character: &DuelCharacterSnapshot,
        player_num: usize,
        include_summon: bool,
    ) -> bool {
        let player = self.player(player_num);
        if character.key == player.index && character.id == player.id {
            return true;
        }

        include_summon && character.is_summon() && character.owner_id == player.id
    }

    pub fn is_duel_player_index(&self, player_num: usize, index: i16) -> bool {
        self.player(player_num).index == index
    }

    pub fn set_duel_channel(
        &mut self,
        channel_index: usize,
        enabled: bool,
        joinable: bool,
        player_one_id: Option<&str>,
        player_two_id: Option<&str>,
    ) {
        let channel = self.channel_mut(channel_index);
        channel.enabled = enabled;
        channel.joinable = joinable;
        channel.player_one_id = normalize_username(player_one_id);
        channel.player_two_id = normalize_username(player_two_id);
    }

    pub fn is_duel_channel_enabled(&self, channel_index: usize) -> bool {
        self.channel(channel_index).enabled
    }

    pub fn is_duel_channel_joinable(&self, channel_index: usize) -> bool {
        self.channel(channel_index).joinable
    }

    pub fn duel_channel_player_one_id(&self, channel_index: usize) -> &str {
        self.channel(channel_index).player_one_id.as_str()
    }

    pub fn duel_channel_player_two_id(&self, channel_index: usize) -> &str {
        self.channel(channel_index).player_two_id.as_str()
    }

    pub fn set_current_channel(&mut self, channel: i32) {
        self.current_channel = channel;
    }

    pub fn current_channel(&self) -> i32 {
        self.current_channel
    }

    pub fn remove_all_duel_watch_user(&mut self) {
        self.watch_users.clear();
    }

    pub fn add_duel_watch_user(&mut self, user_id: Option<&str>) {
        let Some(user_id) = user_id else {
            return;
        };

        self.watch_users.push(normalize_username(Some(user_id)));
    }

    pub fn remove_duel_watch_user(&mut self, user_id: &str) -> bool {
        let before = self.watch_users.len();
        self.watch_users
            .retain(|watch_user| watch_user.as_str() != user_id);
        before != self.watch_users.len()
    }

    pub fn duel_watch_user(&self, index: usize) -> Option<&str> {
        self.watch_users.get(index).map(String::as_str)
    }

    pub fn duel_watch_users(&self) -> &[String] {
        &self.watch_users
    }

    pub fn fighter_regenerated(&self) -> bool {
        self.fighter_regenerated
    }

    pub fn set_fighter_regenerated(&mut self, flag: bool) {
        self.fighter_regenerated = flag;
    }

    fn player_mut(&mut self, player_num: usize) -> &mut DuelPlayerInfo {
        self.players
            .get_mut(player_num)
            .expect("duel player index out of range")
    }

    fn channel_mut(&mut self, channel_index: usize) -> &mut DuelChannelInfo {
        self.channels
            .get_mut(channel_index)
            .expect("duel channel index out of range")
    }
}

fn normalize_username(value: Option<&str>) -> String {
    let Some(value) = value else {
        return String::new();
    };

    value.chars().take(MAX_USERNAME_SIZE).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        DuelCharacterSnapshot, DuelManager, DuelPlugin, MAX_DUEL_CHANNELS, MAX_DUEL_PLAYERS,
        MAX_USERNAME_SIZE,
    };
    use bevy::prelude::App;

    #[test]
    fn plugin_registers_the_duel_manager_resource() {
        let mut app = App::new();
        app.add_plugins(DuelPlugin);

        let manager = app.world().resource::<DuelManager>();
        assert!(!manager.duel_enabled());
        assert_eq!(manager.current_channel(), -1);
        assert_eq!(manager.players().len(), MAX_DUEL_PLAYERS);
        assert_eq!(manager.channels().len(), MAX_DUEL_CHANNELS);
    }

    #[test]
    fn reset_restores_the_legacy_defaults() {
        let mut manager = DuelManager::new();
        manager.enable_duel(true);
        manager.enable_pet_duel(true);
        manager.set_duel_player(0, 7, Some("abcdefghijklmnop"));
        manager.set_duel_channel(0, true, true, Some("Alice"), Some("Bob"));
        manager.add_duel_watch_user(Some("Viewer"));
        manager.set_current_channel(3);
        manager.set_fighter_regenerated(true);

        manager.reset();

        assert!(!manager.duel_enabled());
        assert!(!manager.pet_duel_enabled());
        assert_eq!(manager.current_channel(), -1);
        assert!(!manager.fighter_regenerated());
        assert!(manager.duel_watch_users().is_empty());
        assert_eq!(manager.player(0).id, "");
        assert_eq!(manager.channel(0).player_one_id, "");
    }

    #[test]
    fn enables_and_disables_duels_like_the_legacy_manager() {
        let mut manager = DuelManager::new();
        manager.enable_pet_duel(true);
        manager.set_duel_player(0, 5, Some("Hero"));

        manager.enable_duel(true);
        assert!(manager.duel_enabled());
        assert!(manager.pet_duel_enabled());
        assert_eq!(manager.player(0).index, 5);

        manager.enable_duel(false);
        assert!(!manager.duel_enabled());
        assert!(!manager.pet_duel_enabled());
        assert_eq!(manager.player(0).index, 0);
    }

    #[test]
    fn tracks_duel_players_channels_and_watchers() {
        let mut manager = DuelManager::new();
        manager.set_duel_player(0, 11, Some("abcdefghijklmnop"));
        manager.set_hero_as_duel_player(1, 22, "Champion");
        manager.set_score(0, 42);
        manager.set_hp(0, 75);
        manager.set_sd(0, 25);
        manager.set_duel_channel(2, true, false, Some("Alice"), Some("Bob"));
        manager.add_duel_watch_user(Some("Watcher"));
        manager.add_duel_watch_user(None);

        assert_eq!(manager.player(0).id, "abcdefghij");
        assert_eq!(manager.player(1).id, "Champion");
        assert_eq!(manager.player(0).score, 42);
        assert_eq!(manager.player(0).hp_rate, 0.75);
        assert_eq!(manager.player(0).sd_rate, 0.25);
        assert!(manager.is_duel_channel_enabled(2));
        assert!(!manager.is_duel_channel_joinable(2));
        assert_eq!(manager.duel_channel_player_one_id(2), "Alice");
        assert_eq!(manager.duel_channel_player_two_id(2), "Bob");
        assert_eq!(manager.duel_watch_user(0), Some("Watcher"));
        assert_eq!(manager.duel_watch_user(1), None);
        assert!(manager.remove_duel_watch_user("Watcher"));
        assert!(!manager.remove_duel_watch_user("Watcher"));
    }

    #[test]
    fn matches_duel_players_and_summons_by_legacy_rules() {
        let mut manager = DuelManager::new();
        manager.set_duel_player(0, 7, Some("Hero"));

        let direct = DuelCharacterSnapshot {
            key: 7,
            id: "Hero".to_string(),
            owner_id: String::new(),
            base_class: 1,
        };
        let summon = DuelCharacterSnapshot {
            key: 9,
            id: "Pet".to_string(),
            owner_id: "Hero".to_string(),
            base_class: 0,
        };
        let stranger = DuelCharacterSnapshot {
            key: 9,
            id: "Pet".to_string(),
            owner_id: "SomeoneElse".to_string(),
            base_class: 0,
        };

        assert!(manager.is_duel_player(&direct, 0, true));
        assert!(manager.is_duel_player(&summon, 0, true));
        assert!(!manager.is_duel_player(&summon, 0, false));
        assert!(!manager.is_duel_player(&stranger, 0, true));
        assert!(!direct.is_summon());
        assert!(summon.is_summon());
        assert_eq!(MAX_USERNAME_SIZE, 10);
    }
}
