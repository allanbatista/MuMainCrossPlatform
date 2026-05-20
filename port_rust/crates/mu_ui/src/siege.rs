use std::convert::TryInto;

use bevy::app::{App, Plugin};

use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{GuildCache, INVALID_GUILD_MARK_INDEX};

const TITLE: &str = "Siege";
const INACTIVE_NOTICE: &str = "Castle Siege is not active.";
const OBSERVER_NOTICE: &str = "Observe the Castle Siege battlefield.";
const SOLDIER_NOTICE: &str = "Follow the siege map and battle skills.";
const COMMANDER_NOTICE: &str = "Command guild groups on the siege map.";

const INACTIVE_ACTIONS: &[SiegeAction] = &[SiegeAction::Close];
const OBSERVER_ACTIONS: &[SiegeAction] = &[SiegeAction::ToggleAlpha, SiegeAction::Close];
const SOLDIER_ACTIONS: &[SiegeAction] = &[
    SiegeAction::ToggleAlpha,
    SiegeAction::ScrollSkillUp,
    SiegeAction::ScrollSkillDown,
    SiegeAction::Close,
];
const COMMANDER_ACTIONS: &[SiegeAction] = &[
    SiegeAction::ToggleAlpha,
    SiegeAction::SelectGroup,
    SiegeAction::SelectCommand,
    SiegeAction::ScrollSkillUp,
    SiegeAction::ScrollSkillDown,
    SiegeAction::ClearLocations,
    SiegeAction::Close,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiegeScreenState {
    Inactive,
    Observer,
    Soldier,
    Commander,
}

impl SiegeScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Observer => "observer",
            Self::Soldier => "soldier",
            Self::Commander => "commander",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Inactive => Some(INACTIVE_NOTICE),
            Self::Observer => Some(OBSERVER_NOTICE),
            Self::Soldier => Some(SOLDIER_NOTICE),
            Self::Commander => Some(COMMANDER_NOTICE),
        }
    }

    pub const fn actions(self) -> &'static [SiegeAction] {
        match self {
            Self::Inactive => INACTIVE_ACTIONS,
            Self::Observer => OBSERVER_ACTIONS,
            Self::Soldier => SOLDIER_ACTIONS,
            Self::Commander => COMMANDER_ACTIONS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiegeAction {
    ToggleAlpha,
    SelectGroup,
    SelectCommand,
    ScrollSkillUp,
    ScrollSkillDown,
    ClearLocations,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiegeCommand {
    Attack,
    Defence,
    Wait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiegeBattleSkill {
    Invisible,
    RemoveInvisible,
    RemoveBuff,
    Stun,
    RemoveStun,
    Mana,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiegeCommanderEntry {
    pub team: u8,
    pub command: SiegeCommand,
    pub x: u8,
    pub y: u8,
    pub life_time: u8,
}

impl SiegeCommanderEntry {
    pub const fn new(team: u8, command: SiegeCommand, x: u8, y: u8, life_time: u8) -> Self {
        Self {
            team,
            command,
            x,
            y,
            life_time,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiegeMemberLocation {
    pub marker_type: u8,
    pub x: u8,
    pub y: u8,
}

impl SiegeMemberLocation {
    pub const fn new(marker_type: u8, x: u8, y: u8) -> Self {
        Self { marker_type, x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiegeTime {
    pub hour: u8,
    pub minute: u8,
    pub show_colon: bool,
}

impl SiegeTime {
    pub const fn new(hour: u8, minute: u8, show_colon: bool) -> Self {
        Self {
            hour,
            minute,
            show_colon,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SiegeScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: SiegeScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub battle_castle_active: bool,
    pub created: bool,
    pub guild_status: Option<&'static str>,
    pub guild_mark_index: Option<i16>,
    pub time: Option<SiegeTime>,
    pub mini_map_alpha: f32,
    pub skill_ui_visible: bool,
    pub skill_tooltip_visible: bool,
    pub command_controls_visible: bool,
    pub mouse_in_minimap: bool,
    pub selected_group: Option<u8>,
    pub selected_command: Option<SiegeCommand>,
    pub command_buffer: Vec<SiegeCommanderEntry>,
    pub guild_member_locations: Vec<SiegeMemberLocation>,
    pub battle_skills: Vec<SiegeBattleSkill>,
    pub current_battle_skill: Option<SiegeBattleSkill>,
    pub actions: &'static [SiegeAction],
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SiegeUiPlugin;

impl Plugin for SiegeUiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn siege_screen(state: SiegeScreenState, guild_cache: &GuildCache) -> SiegeScreen {
    let preset = match state {
        SiegeScreenState::Inactive => SiegeScreenPreset {
            battle_castle_active: false,
            created: false,
            guild_status: None,
            guild_key: None,
            time: None,
            mini_map_alpha: 1.0,
            skill_ui_visible: false,
            skill_tooltip_visible: false,
            command_controls_visible: false,
            mouse_in_minimap: false,
            selected_group: None,
            selected_command: None,
            command_buffer: Vec::new(),
            guild_member_locations: Vec::new(),
            battle_skills: Vec::new(),
            current_battle_skill: None,
            actions: INACTIVE_ACTIONS,
        },
        SiegeScreenState::Observer => SiegeScreenPreset {
            battle_castle_active: true,
            created: false,
            guild_status: None,
            guild_key: None,
            time: Some(SiegeTime::new(0, 18, true)),
            mini_map_alpha: 0.9,
            skill_ui_visible: false,
            skill_tooltip_visible: false,
            command_controls_visible: false,
            mouse_in_minimap: false,
            selected_group: None,
            selected_command: None,
            command_buffer: Vec::new(),
            guild_member_locations: Vec::new(),
            battle_skills: Vec::new(),
            current_battle_skill: None,
            actions: OBSERVER_ACTIONS,
        },
        SiegeScreenState::Soldier => SiegeScreenPreset {
            battle_castle_active: true,
            created: true,
            guild_status: Some("battle-master"),
            guild_key: Some(8),
            time: Some(SiegeTime::new(18, 45, true)),
            mini_map_alpha: 1.0,
            skill_ui_visible: true,
            skill_tooltip_visible: false,
            command_controls_visible: false,
            mouse_in_minimap: false,
            selected_group: None,
            selected_command: None,
            command_buffer: sample_command_buffer(),
            guild_member_locations: Vec::new(),
            battle_skills: vec![
                SiegeBattleSkill::Stun,
                SiegeBattleSkill::RemoveStun,
                SiegeBattleSkill::Mana,
            ],
            current_battle_skill: Some(SiegeBattleSkill::Stun),
            actions: SOLDIER_ACTIONS,
        },
        SiegeScreenState::Commander => SiegeScreenPreset {
            battle_castle_active: true,
            created: true,
            guild_status: Some("master"),
            guild_key: Some(12),
            time: Some(SiegeTime::new(18, 45, true)),
            mini_map_alpha: 0.8,
            skill_ui_visible: true,
            skill_tooltip_visible: true,
            command_controls_visible: true,
            mouse_in_minimap: true,
            selected_group: Some(3),
            selected_command: Some(SiegeCommand::Attack),
            command_buffer: sample_command_buffer(),
            guild_member_locations: sample_member_locations(),
            battle_skills: vec![
                SiegeBattleSkill::Invisible,
                SiegeBattleSkill::RemoveInvisible,
                SiegeBattleSkill::RemoveBuff,
            ],
            current_battle_skill: Some(SiegeBattleSkill::Invisible),
            actions: COMMANDER_ACTIONS,
        },
    };

    SiegeScreen {
        route: UiRoute::Siege,
        layout: UiShellLayout::for_route(UiRoute::Siege),
        widgets: UiShellWidgetSet::for_route(UiRoute::Siege),
        state,
        title: TITLE,
        notice: state.notice(),
        battle_castle_active: preset.battle_castle_active,
        created: preset.created,
        guild_status: preset.guild_status,
        guild_mark_index: project_guild_mark_index(guild_cache, preset.guild_key),
        time: preset.time,
        mini_map_alpha: preset.mini_map_alpha,
        skill_ui_visible: preset.skill_ui_visible,
        skill_tooltip_visible: preset.skill_tooltip_visible,
        command_controls_visible: preset.command_controls_visible,
        mouse_in_minimap: preset.mouse_in_minimap,
        selected_group: preset.selected_group,
        selected_command: preset.selected_command,
        command_buffer: preset.command_buffer,
        guild_member_locations: preset.guild_member_locations,
        battle_skills: preset.battle_skills,
        current_battle_skill: preset.current_battle_skill,
        actions: preset.actions,
    }
}

fn project_guild_mark_index(guild_cache: &GuildCache, guild_key: Option<i32>) -> Option<i16> {
    let guild_key = guild_key?;
    let guild_mark_index = guild_cache.get_guild_mark_index(guild_key);

    if guild_mark_index == INVALID_GUILD_MARK_INDEX {
        return None;
    }

    guild_mark_index.try_into().ok()
}

impl SiegeScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|battle_castle_active={}|created={}|guild_status={:?}|guild_mark_index={:?}|time={:?}|mini_map_alpha={:.1}|skill_ui_visible={}|skill_tooltip_visible={}|command_controls_visible={}|mouse_in_minimap={}|selected_group={:?}|selected_command={:?}|command_buffer={:?}|guild_member_locations={:?}|battle_skills={:?}|current_battle_skill={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.battle_castle_active,
            self.created,
            self.guild_status,
            self.guild_mark_index,
            self.time,
            self.mini_map_alpha,
            self.skill_ui_visible,
            self.skill_tooltip_visible,
            self.command_controls_visible,
            self.mouse_in_minimap,
            self.selected_group,
            self.selected_command,
            self.command_buffer,
            self.guild_member_locations,
            self.battle_skills,
            self.current_battle_skill,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[derive(Debug, Clone)]
struct SiegeScreenPreset {
    battle_castle_active: bool,
    created: bool,
    guild_status: Option<&'static str>,
    guild_key: Option<i32>,
    time: Option<SiegeTime>,
    mini_map_alpha: f32,
    skill_ui_visible: bool,
    skill_tooltip_visible: bool,
    command_controls_visible: bool,
    mouse_in_minimap: bool,
    selected_group: Option<u8>,
    selected_command: Option<SiegeCommand>,
    command_buffer: Vec<SiegeCommanderEntry>,
    guild_member_locations: Vec<SiegeMemberLocation>,
    battle_skills: Vec<SiegeBattleSkill>,
    current_battle_skill: Option<SiegeBattleSkill>,
    actions: &'static [SiegeAction],
}

fn sample_command_buffer() -> Vec<SiegeCommanderEntry> {
    vec![
        SiegeCommanderEntry::new(0, SiegeCommand::Attack, 126, 81, 100),
        SiegeCommanderEntry::new(1, SiegeCommand::Defence, 64, 144, 100),
        SiegeCommanderEntry::new(2, SiegeCommand::Wait, 34, 208, 100),
    ]
}

fn sample_member_locations() -> Vec<SiegeMemberLocation> {
    vec![
        SiegeMemberLocation::new(0, 22, 44),
        SiegeMemberLocation::new(1, 98, 110),
        SiegeMemberLocation::new(2, 150, 76),
    ]
}

#[cfg(test)]
mod tests {
    use super::{siege_screen, GuildCache, SiegeAction, SiegeBattleSkill, SiegeScreenState};

    fn populated_guild_cache() -> GuildCache {
        let mut cache = GuildCache::new();

        for guild_key in 0_i32..=12 {
            assert_eq!(cache.make_guild_mark_index(guild_key), guild_key);
        }

        cache
    }

    #[test]
    fn inactive_and_observer_snapshots_cover_the_base_shell() {
        let cache = populated_guild_cache();

        assert_eq!(
            siege_screen(SiegeScreenState::Inactive, &cache).snapshot(),
            "route=siege|group=gameplay|state=inactive|title=Siege|notice=Some(\"Castle Siege is not active.\")|battle_castle_active=false|created=false|guild_status=None|guild_mark_index=None|time=None|mini_map_alpha=1.0|skill_ui_visible=false|skill_tooltip_visible=false|command_controls_visible=false|mouse_in_minimap=false|selected_group=None|selected_command=None|command_buffer=[]|guild_member_locations=[]|battle_skills=[]|current_battle_skill=None|actions=[Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(
            siege_screen(SiegeScreenState::Observer, &cache).snapshot(),
            "route=siege|group=gameplay|state=observer|title=Siege|notice=Some(\"Observe the Castle Siege battlefield.\")|battle_castle_active=true|created=false|guild_status=None|guild_mark_index=None|time=Some(SiegeTime { hour: 0, minute: 18, show_colon: true })|mini_map_alpha=0.9|skill_ui_visible=false|skill_tooltip_visible=false|command_controls_visible=false|mouse_in_minimap=false|selected_group=None|selected_command=None|command_buffer=[]|guild_member_locations=[]|battle_skills=[]|current_battle_skill=None|actions=[ToggleAlpha, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn soldier_and_commander_snapshots_surface_the_active_controls() {
        let cache = populated_guild_cache();
        let soldier = siege_screen(SiegeScreenState::Soldier, &cache);
        assert_eq!(soldier.state, SiegeScreenState::Soldier);
        assert!(soldier.skill_ui_visible);
        assert_eq!(
            soldier.battle_skills,
            vec![
                SiegeBattleSkill::Stun,
                SiegeBattleSkill::RemoveStun,
                SiegeBattleSkill::Mana,
            ]
        );
        assert_eq!(
            soldier.actions,
            &[
                SiegeAction::ToggleAlpha,
                SiegeAction::ScrollSkillUp,
                SiegeAction::ScrollSkillDown,
                SiegeAction::Close,
            ]
        );
        assert_eq!(
            soldier.snapshot(),
            "route=siege|group=gameplay|state=soldier|title=Siege|notice=Some(\"Follow the siege map and battle skills.\")|battle_castle_active=true|created=true|guild_status=Some(\"battle-master\")|guild_mark_index=Some(8)|time=Some(SiegeTime { hour: 18, minute: 45, show_colon: true })|mini_map_alpha=1.0|skill_ui_visible=true|skill_tooltip_visible=false|command_controls_visible=false|mouse_in_minimap=false|selected_group=None|selected_command=None|command_buffer=[SiegeCommanderEntry { team: 0, command: Attack, x: 126, y: 81, life_time: 100 }, SiegeCommanderEntry { team: 1, command: Defence, x: 64, y: 144, life_time: 100 }, SiegeCommanderEntry { team: 2, command: Wait, x: 34, y: 208, life_time: 100 }]|guild_member_locations=[]|battle_skills=[Stun, RemoveStun, Mana]|current_battle_skill=Some(Stun)|actions=[ToggleAlpha, ScrollSkillUp, ScrollSkillDown, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        let commander = siege_screen(SiegeScreenState::Commander, &cache);
        assert!(commander.command_controls_visible);
        assert!(commander.mouse_in_minimap);
        assert_eq!(
            commander.guild_member_locations,
            vec![
                super::SiegeMemberLocation::new(0, 22, 44),
                super::SiegeMemberLocation::new(1, 98, 110),
                super::SiegeMemberLocation::new(2, 150, 76),
            ]
        );
        assert_eq!(
            commander.battle_skills,
            vec![
                SiegeBattleSkill::Invisible,
                SiegeBattleSkill::RemoveInvisible,
                SiegeBattleSkill::RemoveBuff,
            ]
        );
        assert_eq!(
            commander.snapshot(),
            "route=siege|group=gameplay|state=commander|title=Siege|notice=Some(\"Command guild groups on the siege map.\")|battle_castle_active=true|created=true|guild_status=Some(\"master\")|guild_mark_index=Some(12)|time=Some(SiegeTime { hour: 18, minute: 45, show_colon: true })|mini_map_alpha=0.8|skill_ui_visible=true|skill_tooltip_visible=true|command_controls_visible=true|mouse_in_minimap=true|selected_group=Some(3)|selected_command=Some(Attack)|command_buffer=[SiegeCommanderEntry { team: 0, command: Attack, x: 126, y: 81, life_time: 100 }, SiegeCommanderEntry { team: 1, command: Defence, x: 64, y: 144, life_time: 100 }, SiegeCommanderEntry { team: 2, command: Wait, x: 34, y: 208, life_time: 100 }]|guild_member_locations=[SiegeMemberLocation { marker_type: 0, x: 22, y: 44 }, SiegeMemberLocation { marker_type: 1, x: 98, y: 110 }, SiegeMemberLocation { marker_type: 2, x: 150, y: 76 }]|battle_skills=[Invisible, RemoveInvisible, RemoveBuff]|current_battle_skill=Some(Invisible)|actions=[ToggleAlpha, SelectGroup, SelectCommand, ScrollSkillUp, ScrollSkillDown, ClearLocations, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }

    #[test]
    fn guild_mark_index_is_projected_from_the_cache() {
        let mut cache = GuildCache::new();
        assert_eq!(cache.make_guild_mark_index(12), 0);

        let commander = siege_screen(SiegeScreenState::Commander, &cache);

        assert_eq!(commander.guild_mark_index, Some(0));
    }
}
