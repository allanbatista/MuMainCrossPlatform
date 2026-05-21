use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_editor_admin::{SkillEditorAction, SkillEditorState, SkillEditorWorkflowState};

const SKILL_EDITOR_TITLE: &str = "Admin Skill Editor";
const EMPTY_NOTICE: &str = "No skill rows are loaded.";
const READY_NOTICE: &str = "Inspect, edit, save, export, or cancel skill rows.";
const SAVED_NOTICE: &str = "Skill editor changes were saved.";
const CANCELLED_NOTICE: &str = "Pending skill editor changes were cancelled.";

pub use mu_editor_admin::SkillEditorState as AdminSkillEditorScreenState;

#[derive(Debug, Clone, PartialEq)]
pub struct AdminSkillEditorScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: AdminSkillEditorScreenState,
    pub title: &'static str,
    pub notice: Option<String>,
    pub actions: &'static [SkillEditorAction],
}

pub fn admin_skill_editor_screen(state: &SkillEditorState) -> AdminSkillEditorScreen {
    AdminSkillEditorScreen {
        route: UiRoute::AdminSkillEditor,
        layout: UiShellLayout::for_route(UiRoute::AdminSkillEditor),
        widgets: UiShellWidgetSet::for_route(UiRoute::AdminSkillEditor),
        state: state.clone(),
        title: SKILL_EDITOR_TITLE,
        notice: notice_for_state(state),
        actions: state.toolbar_actions(),
    }
}

impl AdminSkillEditorScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|title={}|notice={:?}|state={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.title,
            self.notice,
            self.state.snapshot(),
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

fn notice_for_state(state: &SkillEditorState) -> Option<String> {
    match state.workflow_state {
        SkillEditorWorkflowState::Empty => Some(EMPTY_NOTICE.to_string()),
        SkillEditorWorkflowState::Ready | SkillEditorWorkflowState::Dirty => {
            Some(READY_NOTICE.to_string())
        }
        SkillEditorWorkflowState::Saved => Some(SAVED_NOTICE.to_string()),
        SkillEditorWorkflowState::Cancelled => Some(CANCELLED_NOTICE.to_string()),
        SkillEditorWorkflowState::Failed { ref message } => Some(message.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::admin_skill_editor_screen;
    use mu_editor_admin::{SkillEditorAction, SkillEditorState, SkillEditorWorkflowState};

    #[test]
    fn skill_editor_empty_snapshot_includes_empty_notice() {
        let state = SkillEditorState::default();
        let screen = admin_skill_editor_screen(&state);

        assert_eq!(screen.route, crate::UiRoute::AdminSkillEditor);
        assert_eq!(screen.actions, &[SkillEditorAction::Close]);
        assert_eq!(screen.state.workflow_state, SkillEditorWorkflowState::Empty);
        assert_eq!(screen.notice.as_deref(), Some("No skill rows are loaded."));
        let snapshot = screen.snapshot();
        assert!(snapshot.contains("route=admin-skill-editor|group=admin|title=Admin Skill Editor"));
        assert!(snapshot.contains("actions=[Close]"));
    }

    #[test]
    fn skill_editor_ready_snapshot_tracks_selected_row_and_actions() {
        let mut state = SkillEditorState::sample();
        state
            .rename_selected_row("Greater Teleport")
            .expect("rename should work");
        state.set_search_query("greater");

        let screen = admin_skill_editor_screen(&state);

        assert_eq!(
            screen.actions,
            &[
                SkillEditorAction::Save,
                SkillEditorAction::ExportLegacy,
                SkillEditorAction::ExportCsv,
                SkillEditorAction::Cancel,
                SkillEditorAction::Close,
            ]
        );
        assert_eq!(
            screen.notice.as_deref(),
            Some("Inspect, edit, save, export, or cancel skill rows.")
        );
        let snapshot = screen.snapshot();
        assert!(snapshot.contains("state=dirty|rows=3|filtered_rows=1"));
        assert!(snapshot.contains("Greater Teleport"));
    }
}
