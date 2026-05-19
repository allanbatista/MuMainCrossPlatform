use bevy::prelude::{App, Plugin, Resource};

use crate::MAX_USERNAME_SIZE;

pub const MAX_PARTY_MEMBERS: usize = 5;
// Legacy IsPartyActive() skips the final slot.
pub const MAX_PARTY_ACTIVE_MEMBERS: usize = MAX_PARTY_MEMBERS - 1;
pub const PARTY_INDEX_UNSEARCHED: i16 = -2;
pub const PARTY_INDEX_HERO: i16 = -3;
pub const PARTY_INDEX_NOT_FOUND: i16 = -1;

#[derive(Debug, Clone, PartialEq)]
pub struct PartyCharacterSnapshot {
    pub index: i16,
    pub id: String,
    pub is_player: bool,
    pub live: bool,
    pub visible: bool,
    pub alpha: f32,
    pub dead: bool,
}

impl PartyCharacterSnapshot {
    pub fn is_search_candidate(&self) -> bool {
        self.is_player && self.live && self.visible && self.alpha > 0.0 && !self.dead
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMemberInfo {
    pub name: String,
    pub number: u8,
    pub map: u8,
    pub x: u8,
    pub y: u8,
    pub curr_hp: i32,
    pub max_hp: i32,
    pub step_hp: u8,
    pub index: i16,
}

impl Default for PartyMemberInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            number: 0,
            map: 0,
            x: 0,
            y: 0,
            curr_hp: 0,
            max_hp: 0,
            step_hp: 0,
            index: PARTY_INDEX_UNSEARCHED,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyManager {
    members: [PartyMemberInfo; MAX_PARTY_MEMBERS],
    party_number: usize,
}

impl Default for PartyManager {
    fn default() -> Self {
        Self {
            members: std::array::from_fn(|_| PartyMemberInfo::default()),
            party_number: 0,
        }
    }
}

impl Resource for PartyManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct PartyPlugin;

impl Plugin for PartyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PartyManager>();
    }
}

impl PartyManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn party_number(&self) -> usize {
        self.party_number
    }

    pub fn set_party_number(&mut self, party_number: usize) {
        self.party_number = party_number.min(MAX_PARTY_MEMBERS);
    }

    pub fn member(&self, member_num: usize) -> &PartyMemberInfo {
        self.members
            .get(member_num)
            .expect("party member index out of range")
    }

    pub fn member_mut(&mut self, member_num: usize) -> &mut PartyMemberInfo {
        self.members
            .get_mut(member_num)
            .expect("party member index out of range")
    }

    pub fn members(&self) -> &[PartyMemberInfo] {
        &self.members
    }

    pub fn set_member(&mut self, member_num: usize, member: PartyMemberInfo) {
        let slot = self.member_mut(member_num);
        *slot = member;
        slot.name = normalize_username(slot.name.as_str());
        self.party_number = self.party_number.max(member_num + 1).min(MAX_PARTY_MEMBERS);
    }

    pub fn search_party_members(&mut self, characters: &[PartyCharacterSnapshot], hero_id: &str) {
        let party_number = self.party_number.min(MAX_PARTY_MEMBERS);

        for member in self.members.iter_mut().take(party_number) {
            if member.index != PARTY_INDEX_UNSEARCHED {
                continue;
            }

            if let Some(character) = characters
                .iter()
                .find(|character| character.is_search_candidate() && character.id == member.name)
            {
                member.index = character.index;
            }
        }

        for member in self.members.iter_mut().take(party_number) {
            if member.index >= 0 {
                continue;
            }

            member.index = if member.name == hero_id {
                PARTY_INDEX_HERO
            } else {
                PARTY_INDEX_NOT_FOUND
            };
        }
    }

    pub fn is_party_active(&self, characters: &[PartyCharacterSnapshot]) -> bool {
        let mut member_count = 0;

        for member in self
            .members
            .iter()
            .take(self.party_number.min(MAX_PARTY_ACTIVE_MEMBERS))
        {
            if member.name.is_empty() {
                continue;
            }

            if characters
                .iter()
                .any(|character| character.live && character.id == member.name)
            {
                member_count += 1;
            }
        }

        member_count > 1
    }

    pub fn is_party_member_character(&self, character: &PartyCharacterSnapshot) -> bool {
        self.members
            .iter()
            .take(self.party_number)
            .any(|member| member.name == character.id)
    }

    pub fn get_party_member_character<'a>(
        &self,
        member: &PartyMemberInfo,
        characters: &'a [PartyCharacterSnapshot],
    ) -> Option<&'a PartyCharacterSnapshot> {
        if member.name.is_empty() {
            return None;
        }

        characters
            .iter()
            .find(|character| character.live && character.id == member.name)
    }
}

fn normalize_username(value: &str) -> String {
    value.chars().take(MAX_USERNAME_SIZE).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        PartyCharacterSnapshot, PartyManager, PartyMemberInfo, PartyPlugin,
        MAX_PARTY_ACTIVE_MEMBERS, MAX_PARTY_MEMBERS, PARTY_INDEX_HERO, PARTY_INDEX_NOT_FOUND,
        PARTY_INDEX_UNSEARCHED,
    };
    use bevy::prelude::App;

    fn character(
        index: i16,
        id: &str,
        is_player: bool,
        live: bool,
        visible: bool,
        alpha: f32,
        dead: bool,
    ) -> PartyCharacterSnapshot {
        PartyCharacterSnapshot {
            index,
            id: id.to_string(),
            is_player,
            live,
            visible,
            alpha,
            dead,
        }
    }

    #[test]
    fn plugin_registers_the_party_manager_resource() {
        let mut app = App::new();
        app.add_plugins(PartyPlugin);

        let manager = app.world().resource::<PartyManager>();
        assert_eq!(manager.party_number(), 0);
        assert_eq!(manager.members().len(), MAX_PARTY_MEMBERS);
        assert!(manager
            .members()
            .iter()
            .all(|member| member.index == PARTY_INDEX_UNSEARCHED));
    }

    #[test]
    fn search_party_members_matches_visible_live_characters_and_preserves_sentinels() {
        let mut manager = PartyManager::new();
        manager.set_party_number(5);
        manager.set_member(
            0,
            PartyMemberInfo {
                name: "Hero".into(),
                ..PartyMemberInfo::default()
            },
        );
        manager.set_member(
            1,
            PartyMemberInfo {
                name: "Ally".into(),
                ..PartyMemberInfo::default()
            },
        );
        manager.set_member(
            2,
            PartyMemberInfo {
                name: "Pinned".into(),
                index: 17,
                ..PartyMemberInfo::default()
            },
        );
        manager.set_member(
            3,
            PartyMemberInfo {
                name: "Stranger".into(),
                ..PartyMemberInfo::default()
            },
        );
        manager.set_member(
            4,
            PartyMemberInfo {
                name: "OfflineH".into(),
                ..PartyMemberInfo::default()
            },
        );

        let characters = [
            character(11, "Hero", true, true, true, 1.0, false),
            character(12, "Ally", true, true, true, 1.0, false),
            character(13, "Ghost", true, true, false, 1.0, false),
            character(14, "OfflineH", true, true, false, 1.0, false),
        ];

        manager.search_party_members(&characters, "OfflineH");

        assert_eq!(manager.member(0).index, 11);
        assert_eq!(manager.member(1).index, 12);
        assert_eq!(manager.member(2).index, 17);
        assert_eq!(manager.member(3).index, PARTY_INDEX_NOT_FOUND);
        assert_eq!(manager.member(4).index, PARTY_INDEX_HERO);
    }

    #[test]
    fn is_party_active_skips_the_last_party_slot_like_legacy_manager() {
        let mut manager = PartyManager::new();
        manager.set_party_number(MAX_PARTY_ACTIVE_MEMBERS + 1);
        manager.set_member(
            0,
            PartyMemberInfo {
                name: "Hero".into(),
                ..PartyMemberInfo::default()
            },
        );
        manager.set_member(
            MAX_PARTY_ACTIVE_MEMBERS,
            PartyMemberInfo {
                name: "Ignored".into(),
                ..PartyMemberInfo::default()
            },
        );

        let characters = [
            character(11, "Hero", true, true, true, 1.0, false),
            character(12, "Ignored", true, true, true, 1.0, false),
        ];

        assert!(!manager.is_party_active(&characters));
        assert!(manager.is_party_member_character(&characters[0]));
        assert_eq!(
            manager.get_party_member_character(manager.member(0), &characters),
            Some(&characters[0])
        );
        assert_eq!(
            manager
                .get_party_member_character(manager.member(MAX_PARTY_ACTIVE_MEMBERS), &characters),
            Some(&characters[1])
        );
    }
}
