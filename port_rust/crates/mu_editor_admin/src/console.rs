use bevy::prelude::{App, Plugin, Resource};

fn normalize_line(message: impl Into<String>) -> String {
    message
        .into()
        .trim_end_matches(&['\r', '\n'][..])
        .to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EditorConsoleState {
    pub editor_entries: Vec<String>,
    pub game_entries: Vec<String>,
    pub log_file_path: Option<String>,
}

impl Resource for EditorConsoleState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct EditorConsolePlugin;

impl Plugin for EditorConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorConsoleState>();
    }
}

impl EditorConsoleState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_log_file_path(path: impl Into<String>) -> Self {
        let mut state = Self::default();
        state.set_log_file_path(path);
        state
    }

    pub fn log_editor(&mut self, message: impl Into<String>) {
        self.editor_entries.push(normalize_line(message));
    }

    pub fn log_game(&mut self, message: impl Into<String>) {
        self.game_entries.push(normalize_line(message));
    }

    pub fn clear_editor_log(&mut self) {
        self.editor_entries.clear();
    }

    pub fn clear_game_log(&mut self) {
        self.game_entries.clear();
    }

    pub fn set_log_file_path(&mut self, path: impl Into<String>) {
        self.log_file_path = Some(path.into());
    }

    pub fn clear_log_file_path(&mut self) {
        self.log_file_path = None;
    }

    pub fn editor_log_text(&self) -> String {
        self.editor_entries.join("\n")
    }

    pub fn game_log_text(&self) -> String {
        self.game_entries.join("\n")
    }

    pub fn snapshot(&self) -> String {
        format!(
            "editor_entries={:?}|game_entries={:?}|log_file_path={:?}",
            self.editor_entries, self.game_entries, self.log_file_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{EditorConsolePlugin, EditorConsoleState};
    use bevy::prelude::App;

    #[test]
    fn console_tracks_split_logs_and_file_path() {
        let mut state = EditorConsoleState::with_log_file_path("MuEditor/MuEditor_20260519.log");
        state.log_editor("editor ready\n");
        state.log_game("game ready");
        state.log_game("ping\r\n");

        assert_eq!(state.editor_entries, vec!["editor ready".to_string()]);
        assert_eq!(
            state.game_entries,
            vec!["game ready".to_string(), "ping".to_string()]
        );
        assert_eq!(state.editor_log_text(), "editor ready");
        assert_eq!(state.game_log_text(), "game ready\nping");
        assert_eq!(
            state.snapshot(),
            "editor_entries=[\"editor ready\"]|game_entries=[\"game ready\", \"ping\"]|log_file_path=Some(\"MuEditor/MuEditor_20260519.log\")"
        );
    }

    #[test]
    fn console_clears_both_buffers() {
        let mut state = EditorConsoleState::new();
        state.log_editor("editor");
        state.log_game("game");
        state.clear_editor_log();
        state.clear_game_log();
        state.clear_log_file_path();

        assert!(state.editor_entries.is_empty());
        assert!(state.game_entries.is_empty());
        assert_eq!(
            state.snapshot(),
            "editor_entries=[]|game_entries=[]|log_file_path=None"
        );
    }

    #[test]
    fn plugin_registers_the_resource() {
        let mut app = App::new();
        app.add_plugins(EditorConsolePlugin);

        let state = app.world().resource::<EditorConsoleState>();

        assert!(state.editor_entries.is_empty());
        assert!(state.game_entries.is_empty());
        assert_eq!(state.log_file_path, None);
    }
}
