use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const CHARACTER_DELETE_TITLE: &str = "Character Delete";
const CHARACTER_DELETE_PROMPT: &str = "Confirm the selected character for deletion.";
const READY_NOTICE: &str = "Submit the local security code to delete the selected character.";
const SUBMITTING_NOTICE: &str = "Deleting character...";
const MISSING_TARGET_NOTICE: &str = "Select a character before deleting it.";
const ERROR_NOTICE: &str = "Character deletion is unavailable.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterDeleteScreenState {
    Ready,
    Submitting,
    MissingTarget,
    Error,
}

impl CharacterDeleteScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Submitting => "submitting",
            Self::MissingTarget => "missing-target",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Ready => Some(READY_NOTICE),
            Self::Submitting => Some(SUBMITTING_NOTICE),
            Self::MissingTarget => Some(MISSING_TARGET_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterDeleteAction {
    Delete,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharacterDeleteButton {
    pub action: CharacterDeleteAction,
    pub enabled: bool,
}

impl CharacterDeleteButton {
    pub const fn new(action: CharacterDeleteAction, enabled: bool) -> Self {
        Self { action, enabled }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterDeleteScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: CharacterDeleteScreenState,
    pub title: &'static str,
    pub prompt: &'static str,
    pub notice: Option<&'static str>,
    pub buttons: [CharacterDeleteButton; 2],
}

pub fn character_delete_screen(state: CharacterDeleteScreenState) -> CharacterDeleteScreen {
    let buttons = match state {
        CharacterDeleteScreenState::Ready => [
            CharacterDeleteButton::new(CharacterDeleteAction::Delete, true),
            CharacterDeleteButton::new(CharacterDeleteAction::Cancel, true),
        ],
        CharacterDeleteScreenState::Submitting => [
            CharacterDeleteButton::new(CharacterDeleteAction::Delete, false),
            CharacterDeleteButton::new(CharacterDeleteAction::Cancel, false),
        ],
        CharacterDeleteScreenState::MissingTarget => [
            CharacterDeleteButton::new(CharacterDeleteAction::Delete, false),
            CharacterDeleteButton::new(CharacterDeleteAction::Cancel, true),
        ],
        CharacterDeleteScreenState::Error => [
            CharacterDeleteButton::new(CharacterDeleteAction::Delete, false),
            CharacterDeleteButton::new(CharacterDeleteAction::Cancel, true),
        ],
    };

    CharacterDeleteScreen {
        route: UiRoute::CharacterDelete,
        layout: UiShellLayout::for_route(UiRoute::CharacterDelete),
        widgets: UiShellWidgetSet::for_route(UiRoute::CharacterDelete),
        state,
        title: CHARACTER_DELETE_TITLE,
        prompt: CHARACTER_DELETE_PROMPT,
        notice: state.notice(),
        buttons,
    }
}

impl CharacterDeleteScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|prompt={}|notice={:?}|buttons={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.prompt,
            self.notice,
            self.buttons,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        character_delete_screen, CharacterDeleteAction, CharacterDeleteButton,
        CharacterDeleteScreenState,
    };

    #[test]
    fn ready_screen_renders_confirmation_actions() {
        let screen = character_delete_screen(CharacterDeleteScreenState::Ready);

        assert_eq!(screen.title, "Character Delete");
        assert!(screen.prompt.contains("Confirm"));
        assert!(screen.notice.is_some());
        assert_eq!(
            screen.buttons[0],
            CharacterDeleteButton::new(CharacterDeleteAction::Delete, true)
        );
        assert_eq!(
            screen.buttons[1],
            CharacterDeleteButton::new(CharacterDeleteAction::Cancel, true)
        );
        assert!(screen.snapshot().contains("route=character-delete"));
    }

    #[test]
    fn submitting_screen_disables_actions() {
        let screen = character_delete_screen(CharacterDeleteScreenState::Submitting);

        assert!(screen.notice.unwrap().contains("Deleting"));
        assert!(!screen.buttons[0].enabled);
        assert!(!screen.buttons[1].enabled);
    }

    #[test]
    fn missing_target_screen_keeps_cancel_enabled() {
        let screen = character_delete_screen(CharacterDeleteScreenState::MissingTarget);

        assert!(screen.notice.unwrap().contains("Select a character"));
        assert!(!screen.buttons[0].enabled);
        assert!(screen.buttons[1].enabled);
    }
}
