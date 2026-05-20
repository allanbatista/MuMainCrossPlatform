use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::CharacterClass;

const CHARACTER_SELECT_TITLE: &str = "Character Select";
const LOADING_NOTICE: &str = "Loading character list...";
const EMPTY_NOTICE: &str = "No characters are available.";
const ERROR_NOTICE: &str = "Failed to load the character list.";
const ACTION_DENIED_NOTICE: &str = "The selected character cannot be used.";
const ACCOUNT_BLOCK_NOTICE: &str = "Account block item active.";

const READY_CHARACTERS: &[CharacterSelectCharacter] = &[
    CharacterSelectCharacter::new(
        0,
        "Astra",
        380,
        CharacterClass::Knight.display_name(),
        "Guildless",
        false,
        true,
    ),
    CharacterSelectCharacter::new(
        1,
        "Selene",
        297,
        CharacterClass::Elf.display_name(),
        "Night Watch",
        true,
        false,
    ),
];

const ACTION_DENIED_CHARACTERS: &[CharacterSelectCharacter] = &[
    CharacterSelectCharacter::new(
        0,
        "Astra",
        380,
        CharacterClass::Knight.display_name(),
        "Guildless",
        false,
        false,
    ),
    CharacterSelectCharacter::new(
        1,
        "Selene",
        297,
        CharacterClass::Elf.display_name(),
        "Night Watch",
        true,
        true,
    ),
];

const LOADING_BUTTONS: &[CharacterSelectButton] = &[
    CharacterSelectButton::new(CharacterSelectAction::Create, false),
    CharacterSelectButton::new(CharacterSelectAction::Menu, true),
    CharacterSelectButton::new(CharacterSelectAction::Connect, false),
    CharacterSelectButton::new(CharacterSelectAction::Delete, false),
];

const EMPTY_BUTTONS: &[CharacterSelectButton] = &[
    CharacterSelectButton::new(CharacterSelectAction::Create, true),
    CharacterSelectButton::new(CharacterSelectAction::Menu, true),
    CharacterSelectButton::new(CharacterSelectAction::Connect, false),
    CharacterSelectButton::new(CharacterSelectAction::Delete, false),
];

const READY_BUTTONS: &[CharacterSelectButton] = &[
    CharacterSelectButton::new(CharacterSelectAction::Create, true),
    CharacterSelectButton::new(CharacterSelectAction::Menu, true),
    CharacterSelectButton::new(CharacterSelectAction::Connect, true),
    CharacterSelectButton::new(CharacterSelectAction::Delete, true),
];

const ERROR_BUTTONS: &[CharacterSelectButton] = &[
    CharacterSelectButton::new(CharacterSelectAction::Create, false),
    CharacterSelectButton::new(CharacterSelectAction::Menu, true),
    CharacterSelectButton::new(CharacterSelectAction::Connect, false),
    CharacterSelectButton::new(CharacterSelectAction::Delete, false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSelectScreenState {
    Loading,
    Empty,
    Ready,
    Error,
    ActionDenied,
}

impl CharacterSelectScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::Ready => "ready",
            Self::Error => "error",
            Self::ActionDenied => "action-denied",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Loading => Some(LOADING_NOTICE),
            Self::Empty => Some(EMPTY_NOTICE),
            Self::Ready => None,
            Self::Error => Some(ERROR_NOTICE),
            Self::ActionDenied => Some(ACTION_DENIED_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSelectAction {
    Menu,
    Create,
    Connect,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterSelectButton {
    pub action: CharacterSelectAction,
    pub enabled: bool,
}

impl CharacterSelectButton {
    pub const fn new(action: CharacterSelectAction, enabled: bool) -> Self {
        Self { action, enabled }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterSelectCharacter {
    pub slot_index: u8,
    pub name: &'static str,
    pub level: u16,
    pub class_name: &'static str,
    pub guild_label: &'static str,
    pub item_blocked: bool,
    pub selected: bool,
}

impl CharacterSelectCharacter {
    pub const fn new(
        slot_index: u8,
        name: &'static str,
        level: u16,
        class_name: &'static str,
        guild_label: &'static str,
        item_blocked: bool,
        selected: bool,
    ) -> Self {
        Self {
            slot_index,
            name,
            level,
            class_name,
            guild_label,
            item_blocked,
            selected,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterSelectScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: CharacterSelectScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub account_block_notice: Option<&'static str>,
    pub empty_slots: Option<u8>,
    pub selected_slot: Option<u8>,
    pub characters: &'static [CharacterSelectCharacter],
    pub buttons: &'static [CharacterSelectButton],
}

pub fn character_select_screen(state: CharacterSelectScreenState) -> CharacterSelectScreen {
    let (empty_slots, selected_slot, characters, buttons, account_block_notice) = match state {
        CharacterSelectScreenState::Loading => (None, None, &[][..], LOADING_BUTTONS, None),
        CharacterSelectScreenState::Empty => (Some(5), None, &[][..], EMPTY_BUTTONS, None),
        CharacterSelectScreenState::Ready => (
            Some(3),
            Some(0),
            READY_CHARACTERS,
            READY_BUTTONS,
            Some(ACCOUNT_BLOCK_NOTICE),
        ),
        CharacterSelectScreenState::Error => (None, None, &[][..], ERROR_BUTTONS, None),
        CharacterSelectScreenState::ActionDenied => (
            Some(3),
            Some(1),
            ACTION_DENIED_CHARACTERS,
            READY_BUTTONS,
            Some(ACCOUNT_BLOCK_NOTICE),
        ),
    };

    CharacterSelectScreen {
        route: UiRoute::CharacterSelect,
        layout: UiShellLayout::for_route(UiRoute::CharacterSelect),
        widgets: UiShellWidgetSet::for_route(UiRoute::CharacterSelect),
        state,
        title: CHARACTER_SELECT_TITLE,
        notice: state.notice(),
        account_block_notice,
        empty_slots,
        selected_slot,
        characters,
        buttons,
    }
}

impl CharacterSelectScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|account_block_notice={:?}|empty_slots={:?}|selected_slot={:?}|characters={:?}|buttons={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.account_block_notice,
            self.empty_slots,
            self.selected_slot,
            self.characters,
            self.buttons,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{character_select_screen, CharacterSelectScreenState};

    #[test]
    fn character_select_loading_empty_and_error_snapshots() {
        assert_eq!(
            character_select_screen(CharacterSelectScreenState::Loading).snapshot(),
            "route=character-select|group=character|state=loading|title=Character Select|notice=Some(\"Loading character list...\")|account_block_notice=None|empty_slots=None|selected_slot=None|characters=[]|buttons=[CharacterSelectButton { action: Create, enabled: false }, CharacterSelectButton { action: Menu, enabled: true }, CharacterSelectButton { action: Connect, enabled: false }, CharacterSelectButton { action: Delete, enabled: false }]|layout=UiShellLayout { group: Character, outer_margin: 32.0, panel_gap: 18.0, sidebar_width: 0.0, content_max_width: 1100.0, footer_height: 36.0 }|widgets=[Header, Body, ActionBar, Footer]"
        );
        assert_eq!(
            character_select_screen(CharacterSelectScreenState::Empty).snapshot(),
            "route=character-select|group=character|state=empty|title=Character Select|notice=Some(\"No characters are available.\")|account_block_notice=None|empty_slots=Some(5)|selected_slot=None|characters=[]|buttons=[CharacterSelectButton { action: Create, enabled: true }, CharacterSelectButton { action: Menu, enabled: true }, CharacterSelectButton { action: Connect, enabled: false }, CharacterSelectButton { action: Delete, enabled: false }]|layout=UiShellLayout { group: Character, outer_margin: 32.0, panel_gap: 18.0, sidebar_width: 0.0, content_max_width: 1100.0, footer_height: 36.0 }|widgets=[Header, Body, ActionBar, Footer]"
        );
        assert_eq!(
            character_select_screen(CharacterSelectScreenState::Error).snapshot(),
            "route=character-select|group=character|state=error|title=Character Select|notice=Some(\"Failed to load the character list.\")|account_block_notice=None|empty_slots=None|selected_slot=None|characters=[]|buttons=[CharacterSelectButton { action: Create, enabled: false }, CharacterSelectButton { action: Menu, enabled: true }, CharacterSelectButton { action: Connect, enabled: false }, CharacterSelectButton { action: Delete, enabled: false }]|layout=UiShellLayout { group: Character, outer_margin: 32.0, panel_gap: 18.0, sidebar_width: 0.0, content_max_width: 1100.0, footer_height: 36.0 }|widgets=[Header, Body, ActionBar, Footer]"
        );
    }

    #[test]
    fn character_select_ready_and_action_denied_snapshots() {
        assert_eq!(
            character_select_screen(CharacterSelectScreenState::Ready).snapshot(),
            "route=character-select|group=character|state=ready|title=Character Select|notice=None|account_block_notice=Some(\"Account block item active.\")|empty_slots=Some(3)|selected_slot=Some(0)|characters=[CharacterSelectCharacter { slot_index: 0, name: \"Astra\", level: 380, class_name: \"Dark Knight\", guild_label: \"Guildless\", item_blocked: false, selected: true }, CharacterSelectCharacter { slot_index: 1, name: \"Selene\", level: 297, class_name: \"Fairy Elf\", guild_label: \"Night Watch\", item_blocked: true, selected: false }]|buttons=[CharacterSelectButton { action: Create, enabled: true }, CharacterSelectButton { action: Menu, enabled: true }, CharacterSelectButton { action: Connect, enabled: true }, CharacterSelectButton { action: Delete, enabled: true }]|layout=UiShellLayout { group: Character, outer_margin: 32.0, panel_gap: 18.0, sidebar_width: 0.0, content_max_width: 1100.0, footer_height: 36.0 }|widgets=[Header, Body, ActionBar, Footer]"
        );
        assert_eq!(
            character_select_screen(CharacterSelectScreenState::ActionDenied).snapshot(),
            "route=character-select|group=character|state=action-denied|title=Character Select|notice=Some(\"The selected character cannot be used.\")|account_block_notice=Some(\"Account block item active.\")|empty_slots=Some(3)|selected_slot=Some(1)|characters=[CharacterSelectCharacter { slot_index: 0, name: \"Astra\", level: 380, class_name: \"Dark Knight\", guild_label: \"Guildless\", item_blocked: false, selected: false }, CharacterSelectCharacter { slot_index: 1, name: \"Selene\", level: 297, class_name: \"Fairy Elf\", guild_label: \"Night Watch\", item_blocked: true, selected: true }]|buttons=[CharacterSelectButton { action: Create, enabled: true }, CharacterSelectButton { action: Menu, enabled: true }, CharacterSelectButton { action: Connect, enabled: true }, CharacterSelectButton { action: Delete, enabled: true }]|layout=UiShellLayout { group: Character, outer_margin: 32.0, panel_gap: 18.0, sidebar_width: 0.0, content_max_width: 1100.0, footer_height: 36.0 }|widgets=[Header, Body, ActionBar, Footer]"
        );
    }
}
