use bevy::prelude::{App, Plugin, Resource};

const DEFAULT_LANGUAGE_LOCALE: &str = "en";
const DEFAULT_LANGUAGE_NAME: &str = "English";
const DEFAULT_AVAILABLE_LOCALES: &[&str] = &[DEFAULT_LANGUAGE_LOCALE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorShellState {
    #[default]
    Closed,
    Open,
}

impl EditorShellState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }

    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorCoreAction {
    OpenEditor,
    ToggleItemEditor,
    ToggleSkillEditor,
    ToggleDevEditor,
    ToggleConsole,
    CloseEditor,
}

const CLOSED_ACTIONS: &[EditorCoreAction] = &[EditorCoreAction::OpenEditor];
const OPEN_ACTIONS: &[EditorCoreAction] = &[
    EditorCoreAction::ToggleItemEditor,
    EditorCoreAction::ToggleSkillEditor,
    EditorCoreAction::ToggleDevEditor,
    EditorCoreAction::ToggleConsole,
    EditorCoreAction::CloseEditor,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorCoreState {
    pub shell_state: EditorShellState,
    pub show_item_editor: bool,
    pub show_skill_editor: bool,
    pub show_dev_editor: bool,
    pub show_console: bool,
    pub hovering_ui: bool,
    pub previous_frame_hovering_ui: bool,
    pub language_locale: String,
    pub language_name: String,
    pub available_locales: Vec<String>,
}

impl Default for EditorCoreState {
    fn default() -> Self {
        Self {
            shell_state: EditorShellState::Closed,
            show_item_editor: false,
            show_skill_editor: false,
            show_dev_editor: false,
            show_console: true,
            hovering_ui: false,
            previous_frame_hovering_ui: false,
            language_locale: DEFAULT_LANGUAGE_LOCALE.to_string(),
            language_name: DEFAULT_LANGUAGE_NAME.to_string(),
            available_locales: DEFAULT_AVAILABLE_LOCALES
                .iter()
                .map(|locale| locale.to_string())
                .collect(),
        }
    }
}

impl Resource for EditorCoreState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct EditorCorePlugin;

impl Plugin for EditorCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorCoreState>();
    }
}

impl EditorCoreState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_enabled(&self) -> bool {
        self.shell_state.is_open()
    }

    pub fn open(&mut self) {
        self.shell_state = EditorShellState::Open;
    }

    pub fn close(&mut self) {
        self.shell_state = EditorShellState::Closed;
    }

    pub fn toggle(&mut self) {
        self.shell_state = if self.is_enabled() {
            EditorShellState::Closed
        } else {
            EditorShellState::Open
        };
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.shell_state = if enabled {
            EditorShellState::Open
        } else {
            EditorShellState::Closed
        };
    }

    pub fn is_showing_item_editor(&self) -> bool {
        self.show_item_editor
    }

    pub fn is_showing_skill_editor(&self) -> bool {
        self.show_skill_editor
    }

    pub fn is_showing_dev_editor(&self) -> bool {
        self.show_dev_editor
    }

    pub fn is_showing_console(&self) -> bool {
        self.show_console
    }

    pub fn set_show_item_editor(&mut self, show: bool) {
        self.show_item_editor = show;
    }

    pub fn set_show_skill_editor(&mut self, show: bool) {
        self.show_skill_editor = show;
    }

    pub fn set_show_dev_editor(&mut self, show: bool) {
        self.show_dev_editor = show;
    }

    pub fn set_show_console(&mut self, show: bool) {
        self.show_console = show;
    }

    pub fn toggle_item_editor(&mut self) {
        self.show_item_editor = !self.show_item_editor;
    }

    pub fn toggle_skill_editor(&mut self) {
        self.show_skill_editor = !self.show_skill_editor;
    }

    pub fn toggle_dev_editor(&mut self) {
        self.show_dev_editor = !self.show_dev_editor;
    }

    pub fn toggle_console(&mut self) {
        self.show_console = !self.show_console;
    }

    pub fn is_hovering_ui(&self) -> bool {
        self.hovering_ui
    }

    pub fn was_hovering_ui(&self) -> bool {
        self.previous_frame_hovering_ui
    }

    pub fn set_hovering_ui(&mut self, hovering: bool) {
        self.hovering_ui = hovering;
    }

    pub fn set_previous_frame_hovering_ui(&mut self, hovering: bool) {
        self.previous_frame_hovering_ui = hovering;
    }

    pub fn set_language_options<I, S>(
        &mut self,
        locale: impl Into<String>,
        language_name: impl Into<String>,
        available_locales: I,
    ) where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.language_locale = locale.into();
        self.language_name = language_name.into();

        let mut locales = available_locales
            .into_iter()
            .map(Into::into)
            .collect::<Vec<String>>();
        locales.sort_unstable();
        locales.dedup();
        if !locales.iter().any(|entry| entry == &self.language_locale) {
            locales.push(self.language_locale.clone());
            locales.sort_unstable();
            locales.dedup();
        }
        if locales.is_empty() {
            locales.push(self.language_locale.clone());
        }
        self.available_locales = locales;
    }

    pub fn toolbar_actions(&self) -> &'static [EditorCoreAction] {
        if self.is_enabled() {
            OPEN_ACTIONS
        } else {
            CLOSED_ACTIONS
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "state={}|editor_enabled={}|hovering_ui={}|previous_frame_hovering_ui={}|show_item_editor={}|show_skill_editor={}|show_dev_editor={}|show_console={}|language_locale={}|language_name={}|available_locales={:?}|actions={:?}",
            self.shell_state.as_str(),
            self.is_enabled(),
            self.hovering_ui,
            self.previous_frame_hovering_ui,
            self.show_item_editor,
            self.show_skill_editor,
            self.show_dev_editor,
            self.show_console,
            self.language_locale,
            self.language_name,
            self.available_locales,
            self.toolbar_actions(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{EditorCoreAction, EditorCorePlugin, EditorCoreState, EditorShellState};
    use bevy::prelude::App;

    #[test]
    fn closed_shell_uses_open_editor_action_only() {
        let state = EditorCoreState::default();

        assert_eq!(state.shell_state, EditorShellState::Closed);
        assert!(!state.is_enabled());
        assert!(state.is_showing_console());
        assert_eq!(state.toolbar_actions(), &[EditorCoreAction::OpenEditor]);
        assert_eq!(
            state.snapshot(),
            "state=closed|editor_enabled=false|hovering_ui=false|previous_frame_hovering_ui=false|show_item_editor=false|show_skill_editor=false|show_dev_editor=false|show_console=true|language_locale=en|language_name=English|available_locales=[\"en\"]|actions=[OpenEditor]"
        );
    }

    #[test]
    fn open_shell_keeps_panels_and_language_state() {
        let mut state = EditorCoreState::new();
        state.open();
        state.toggle_console();
        state.toggle_dev_editor();
        state.set_language_options("pt", "Português", ["en", "pt", "en"]);
        state.set_hovering_ui(true);
        state.set_previous_frame_hovering_ui(false);

        assert!(state.is_enabled());
        assert!(!state.is_showing_console());
        assert!(state.is_showing_dev_editor());
        assert_eq!(state.language_locale, "pt");
        assert_eq!(state.language_name, "Português");
        assert_eq!(
            state.available_locales,
            vec!["en".to_string(), "pt".to_string()]
        );
        assert_eq!(
            state.toolbar_actions(),
            &[
                EditorCoreAction::ToggleItemEditor,
                EditorCoreAction::ToggleSkillEditor,
                EditorCoreAction::ToggleDevEditor,
                EditorCoreAction::ToggleConsole,
                EditorCoreAction::CloseEditor,
            ]
        );
    }

    #[test]
    fn plugin_registers_the_resource() {
        let mut app = App::new();
        app.add_plugins(EditorCorePlugin);

        let state = app.world().resource::<EditorCoreState>();
        assert_eq!(state.shell_state, EditorShellState::Closed);
    }
}
