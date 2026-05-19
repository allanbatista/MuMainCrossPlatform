pub mod auth;
pub mod console;
pub mod core;
pub mod dev_editor;
pub mod item_editor;

pub use auth::{AdminAccessState, AdminAuthState, AdminPlugin};
pub use console::{EditorConsolePlugin, EditorConsoleState};
pub use core::{EditorCoreAction, EditorCorePlugin, EditorCoreState, EditorShellState};
pub use dev_editor::{
    DevEditorDebugFlags, DevEditorDefaultCameraOverride, DevEditorGraphicsState,
    DevEditorOrbitalCameraOverride, DevEditorPlugin, DevEditorRenderToggles, DevEditorState,
};
pub use item_editor::{
    ItemEditorAction, ItemEditorColumn, ItemEditorColumnVisibility, ItemEditorError,
    ItemEditorPlugin, ItemEditorRow, ItemEditorState, ItemEditorWorkflowState,
};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
