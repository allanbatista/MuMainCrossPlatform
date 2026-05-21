use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const HOTKEYS_TITLE: &str = "Hotkeys";
const HOTKEY_LAYER_DEPTH: f32 = 5.2;

const ITEM_HOTKEYS: &[HotkeyBinding] = &[
    HotkeyBinding::new("Q", "Greater Healing Potion", Some(12), true),
    HotkeyBinding::new("W", "Greater Mana Potion", Some(8), true),
    HotkeyBinding::new("E", "Antidote", Some(3), true),
    HotkeyBinding::new("R", "Town Portal Scroll", Some(2), true),
];

const SKILL_HOTKEYS: &[HotkeyBinding] = &[
    HotkeyBinding::new("1", "Skill 1", None, true),
    HotkeyBinding::new("2", "Skill 2", None, true),
    HotkeyBinding::new("3", "Skill 3", None, false),
    HotkeyBinding::new("4", "Skill 4", None, false),
    HotkeyBinding::new("5", "Skill 5", None, false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeyBinding {
    pub slot: &'static str,
    pub binding: &'static str,
    pub count: Option<u16>,
    pub active: bool,
}

impl HotkeyBinding {
    pub const fn new(
        slot: &'static str,
        binding: &'static str,
        count: Option<u16>,
        active: bool,
    ) -> Self {
        Self {
            slot,
            binding,
            count,
            active,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeysScreenState {
    Compact,
}

impl HotkeysScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HotkeysScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: HotkeysScreenState,
    pub title: &'static str,
    pub layer_depth: f32,
    pub item_hotkeys: &'static [HotkeyBinding],
    pub skill_hotkeys: &'static [HotkeyBinding],
    pub skill_list_open: bool,
    pub skill_row_page: &'static str,
    pub current_skill: &'static str,
    pub prior_skill: Option<&'static str>,
    pub pet_commands_visible: bool,
}

pub fn hotkeys_screen() -> HotkeysScreen {
    HotkeysScreen {
        route: UiRoute::Hotkeys,
        layout: UiShellLayout::for_route(UiRoute::Hotkeys),
        widgets: UiShellWidgetSet::for_route(UiRoute::Hotkeys),
        state: HotkeysScreenState::Compact,
        title: HOTKEYS_TITLE,
        layer_depth: HOTKEY_LAYER_DEPTH,
        item_hotkeys: ITEM_HOTKEYS,
        skill_hotkeys: SKILL_HOTKEYS,
        skill_list_open: false,
        skill_row_page: "front",
        current_skill: "Skill 1",
        prior_skill: None,
        pet_commands_visible: false,
    }
}

impl HotkeysScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|layer_depth={}|item_hotkeys={:?}|skill_hotkeys={:?}|skill_list_open={}|skill_row_page={}|current_skill={}|prior_skill={:?}|pet_commands_visible={}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.layer_depth,
            self.item_hotkeys,
            self.skill_hotkeys,
            self.skill_list_open,
            self.skill_row_page,
            self.current_skill,
            self.prior_skill,
            self.pet_commands_visible,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::hotkeys_screen;

    #[test]
    fn hotkeys_snapshot_matches_legacy_compact_bar() {
        assert_eq!(
            hotkeys_screen().snapshot(),
            "route=hotkeys|group=world|state=compact|title=Hotkeys|layer_depth=5.2|item_hotkeys=[HotkeyBinding { slot: \"Q\", binding: \"Greater Healing Potion\", count: Some(12), active: true }, HotkeyBinding { slot: \"W\", binding: \"Greater Mana Potion\", count: Some(8), active: true }, HotkeyBinding { slot: \"E\", binding: \"Antidote\", count: Some(3), active: true }, HotkeyBinding { slot: \"R\", binding: \"Town Portal Scroll\", count: Some(2), active: true }]|skill_hotkeys=[HotkeyBinding { slot: \"1\", binding: \"Skill 1\", count: None, active: true }, HotkeyBinding { slot: \"2\", binding: \"Skill 2\", count: None, active: true }, HotkeyBinding { slot: \"3\", binding: \"Skill 3\", count: None, active: false }, HotkeyBinding { slot: \"4\", binding: \"Skill 4\", count: None, active: false }, HotkeyBinding { slot: \"5\", binding: \"Skill 5\", count: None, active: false }]|skill_list_open=false|skill_row_page=front|current_skill=Skill 1|prior_skill=None|pet_commands_visible=false|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }
}
