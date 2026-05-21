use bevy::prelude::{App, Plugin, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuestMode {
    #[default]
    Journal,
    Dialogue,
    Reward,
    ByEtc,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuestDialogueState {
    pub quest_index: u32,
    pub npc_index: i32,
    pub npc_name: String,
    pub current_page: usize,
    pub max_page: usize,
    pub selected_answer: Option<u8>,
    pub subject: String,
    pub summary: String,
    pub npc_words: Vec<String>,
    pub player_words: Vec<String>,
    pub answers: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuestRewardState {
    pub quest_index: u32,
    pub visible: bool,
    pub request_reward_text: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestManager {
    mode: QuestMode,
    npc_index: Option<i32>,
    npc_name: String,
    current_quests: Vec<u32>,
    etc_quests: Vec<u32>,
    selected_quest_index: Option<u32>,
    dialogue: Option<QuestDialogueState>,
    reward: Option<QuestRewardState>,
}

impl Default for QuestManager {
    fn default() -> Self {
        Self {
            mode: QuestMode::Journal,
            npc_index: None,
            npc_name: String::new(),
            current_quests: Vec::new(),
            etc_quests: Vec::new(),
            selected_quest_index: None,
            dialogue: None,
            reward: None,
        }
    }
}

impl Resource for QuestManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QuestManager>();
    }
}

impl QuestManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn mode(&self) -> QuestMode {
        self.mode
    }

    pub fn npc_index(&self) -> Option<i32> {
        self.npc_index
    }

    pub fn npc_name(&self) -> &str {
        self.npc_name.as_str()
    }

    pub fn current_quests(&self) -> &[u32] {
        &self.current_quests
    }

    pub fn etc_quests(&self) -> &[u32] {
        &self.etc_quests
    }

    pub fn selected_quest_index(&self) -> Option<u32> {
        self.selected_quest_index
    }

    pub fn dialogue(&self) -> Option<&QuestDialogueState> {
        self.dialogue.as_ref()
    }

    pub fn reward(&self) -> Option<&QuestRewardState> {
        self.reward.as_ref()
    }

    pub fn set_current_npc(&mut self, npc_index: i32, npc_name: impl Into<String>) {
        self.npc_index = Some(npc_index);
        self.npc_name = npc_name.into();
    }

    pub fn clear_current_npc(&mut self) {
        self.npc_index = None;
        self.npc_name.clear();
    }

    pub fn set_current_quests<I>(&mut self, quests: I)
    where
        I: IntoIterator<Item = u32>,
    {
        self.current_quests = quests.into_iter().collect();
    }

    pub fn add_current_quest(&mut self, quest_index: u32) {
        if !self.current_quests.contains(&quest_index) {
            self.current_quests.push(quest_index);
        }
    }

    pub fn remove_current_quest(&mut self, quest_index: u32) -> bool {
        let before = self.current_quests.len();
        self.current_quests
            .retain(|current| *current != quest_index);
        before != self.current_quests.len()
    }

    pub fn set_etc_quests<I>(&mut self, quests: I)
    where
        I: IntoIterator<Item = u32>,
    {
        self.etc_quests = quests.into_iter().collect();
    }

    pub fn set_dialogue(&mut self, dialogue: QuestDialogueState) {
        self.selected_quest_index = Some(dialogue.quest_index);
        self.mode = QuestMode::Dialogue;
        self.dialogue = Some(dialogue);
    }

    pub fn clear_dialogue(&mut self) {
        self.dialogue = None;
        if self.mode != QuestMode::Error {
            self.mode = QuestMode::Journal;
        }
    }

    pub fn set_reward(&mut self, reward: QuestRewardState) {
        self.selected_quest_index = Some(reward.quest_index);
        self.mode = QuestMode::Reward;
        self.reward = Some(reward);
    }

    pub fn clear_reward(&mut self) {
        self.reward = None;
        if self.mode != QuestMode::Error {
            self.mode = QuestMode::Journal;
        }
    }

    pub fn mark_by_etc(&mut self) {
        self.mode = QuestMode::ByEtc;
    }

    pub fn mark_error(&mut self) {
        self.mode = QuestMode::Error;
    }

    pub fn clear_error(&mut self) {
        if self.reward.is_some() {
            self.mode = QuestMode::Reward;
        } else if self.dialogue.is_some() {
            self.mode = QuestMode::Dialogue;
        } else {
            self.mode = QuestMode::Journal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{QuestDialogueState, QuestManager, QuestMode, QuestPlugin, QuestRewardState};
    use bevy::prelude::App;

    #[test]
    fn quest_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(QuestPlugin);

        let manager = app.world().resource::<QuestManager>();
        assert_eq!(manager.mode(), QuestMode::Journal);
        assert!(manager.current_quests().is_empty());
        assert!(manager.etc_quests().is_empty());
    }

    #[test]
    fn quest_manager_tracks_dialogue_reward_and_lists() {
        let mut manager = QuestManager::new();
        manager.set_current_npc(18, "Elpis");
        manager.set_current_quests([1001, 1002, 1008]);
        manager.set_etc_quests([2001, 2002]);
        manager.set_dialogue(QuestDialogueState {
            quest_index: 1002,
            npc_index: 18,
            npc_name: "Elpis".to_string(),
            current_page: 1,
            max_page: 4,
            selected_answer: Some(2),
            subject: "Ancient Rite".to_string(),
            summary: "Bring the relic to the altar.".to_string(),
            npc_words: vec!["Welcome back.".into()],
            player_words: vec!["I am ready.".into()],
            answers: vec![1, 2, 3],
        });
        manager.set_reward(QuestRewardState {
            quest_index: 1008,
            visible: true,
            request_reward_text: vec!["Claim reward".into()],
        });

        assert_eq!(manager.npc_index(), Some(18));
        assert_eq!(manager.npc_name(), "Elpis");
        assert_eq!(manager.current_quests(), &[1001, 1002, 1008]);
        assert_eq!(manager.etc_quests(), &[2001, 2002]);
        assert_eq!(manager.selected_quest_index(), Some(1008));
        assert_eq!(manager.mode(), QuestMode::Reward);
        assert_eq!(manager.dialogue().unwrap().subject, "Ancient Rite");
        assert!(manager.reward().unwrap().visible);
    }
}
