use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_editor_admin::{ItemEditorAction, ItemEditorState, ItemEditorWorkflowState};

const ITEM_EDITOR_TITLE: &str = "Admin Item Editor";
const EMPTY_NOTICE: &str = "No item rows are loaded.";
const READY_NOTICE: &str = "Inspect, edit, save, export, or cancel item rows.";
const SAVED_NOTICE: &str = "Item editor changes were saved.";
const CANCELLED_NOTICE: &str = "Pending item editor changes were cancelled.";

pub use mu_editor_admin::ItemEditorState as AdminItemEditorScreenState;

#[derive(Debug, Clone, PartialEq)]
pub struct AdminItemEditorScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: AdminItemEditorScreenState,
    pub title: &'static str,
    pub notice: Option<String>,
    pub actions: &'static [ItemEditorAction],
}

pub fn admin_item_editor_screen(state: &ItemEditorState) -> AdminItemEditorScreen {
    AdminItemEditorScreen {
        route: UiRoute::AdminItemEditor,
        layout: UiShellLayout::for_route(UiRoute::AdminItemEditor),
        widgets: UiShellWidgetSet::for_route(UiRoute::AdminItemEditor),
        state: state.clone(),
        title: ITEM_EDITOR_TITLE,
        notice: notice_for_state(state),
        actions: state.toolbar_actions(),
    }
}

impl AdminItemEditorScreen {
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

fn notice_for_state(state: &ItemEditorState) -> Option<String> {
    match state.workflow_state {
        ItemEditorWorkflowState::Empty => Some(EMPTY_NOTICE.to_string()),
        ItemEditorWorkflowState::Ready | ItemEditorWorkflowState::Dirty => {
            Some(READY_NOTICE.to_string())
        }
        ItemEditorWorkflowState::Saved => Some(SAVED_NOTICE.to_string()),
        ItemEditorWorkflowState::Cancelled => Some(CANCELLED_NOTICE.to_string()),
        ItemEditorWorkflowState::Failed { ref message } => Some(message.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::admin_item_editor_screen;
    use mu_editor_admin::{ItemEditorAction, ItemEditorState, ItemEditorWorkflowState};

    #[test]
    fn item_editor_empty_snapshot_includes_empty_notice() {
        let state = ItemEditorState::default();
        let screen = admin_item_editor_screen(&state);

        assert_eq!(screen.route, crate::UiRoute::AdminItemEditor);
        assert_eq!(screen.actions, &[ItemEditorAction::Close]);
        assert_eq!(screen.state.workflow_state, ItemEditorWorkflowState::Empty);
        assert_eq!(screen.notice.as_deref(), Some("No item rows are loaded."));
        let snapshot = screen.snapshot();
        assert!(snapshot.contains("route=admin-item-editor|group=admin|title=Admin Item Editor"));
        assert!(snapshot.contains("actions=[Close]"));
    }

    #[test]
    fn item_editor_ready_snapshot_tracks_selected_row_and_actions() {
        let mut state = ItemEditorState::sample();
        state
            .rename_selected_row("Greater Sword")
            .expect("rename should work");
        state.set_search_query("sword");

        let screen = admin_item_editor_screen(&state);

        assert_eq!(
            screen.actions,
            &[
                ItemEditorAction::Save,
                ItemEditorAction::ExportLegacy,
                ItemEditorAction::ExportCsv,
                ItemEditorAction::Cancel,
                ItemEditorAction::Close,
            ]
        );
        assert_eq!(
            screen.notice.as_deref(),
            Some("Inspect, edit, save, export, or cancel item rows.")
        );
        let snapshot = screen.snapshot();
        assert!(snapshot.contains("state=dirty|rows=2|filtered_rows=1"));
        assert!(snapshot.contains("Greater Sword"));
    }
}
