use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const OPTIONS_TITLE: &str = "Options";
const READY_NOTICE: &str = "Changes will be applied on save.";
const SAVING_NOTICE: &str = "Saving settings...";
const SAVED_NOTICE: &str = "Settings saved.";
const ERROR_NOTICE: &str = "Settings could not be saved.";

const OPTION_SECTIONS: &[OptionsSection] = &[
    OptionsSection::Video,
    OptionsSection::Audio,
    OptionsSection::Controls,
    OptionsSection::Performance,
    OptionsSection::Network,
    OptionsSection::Locale,
];

const OPTION_TOGGLES: &[OptionsToggle] = &[
    OptionsToggle::Fullscreen,
    OptionsToggle::VSync,
    OptionsToggle::ReduceEffects,
    OptionsToggle::MuteAudio,
    OptionsToggle::RememberServer,
    OptionsToggle::RememberUsername,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsScreenState {
    Ready,
    Saving,
    Saved,
    Error,
}

impl OptionsScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Saving => "saving",
            Self::Saved => "saved",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Ready => Some(READY_NOTICE),
            Self::Saving => Some(SAVING_NOTICE),
            Self::Saved => Some(SAVED_NOTICE),
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsSection {
    Video,
    Audio,
    Controls,
    Performance,
    Network,
    Locale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionsToggle {
    Fullscreen,
    VSync,
    ReduceEffects,
    MuteAudio,
    RememberServer,
    RememberUsername,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptionsScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: OptionsScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub sections: &'static [OptionsSection],
    pub toggles: &'static [OptionsToggle],
}

pub fn options_screen(state: OptionsScreenState) -> OptionsScreen {
    OptionsScreen {
        route: UiRoute::Options,
        layout: UiShellLayout::for_route(UiRoute::Options),
        widgets: UiShellWidgetSet::for_route(UiRoute::Options),
        state,
        title: OPTIONS_TITLE,
        notice: state.notice(),
        sections: OPTION_SECTIONS,
        toggles: OPTION_TOGGLES,
    }
}

impl OptionsScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|sections={:?}|toggles={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.sections,
            self.toggles,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{options_screen, OptionsScreenState};

    #[test]
    fn options_ready_snapshot_lists_sections_and_toggles() {
        assert_eq!(
            options_screen(OptionsScreenState::Ready).snapshot(),
            "route=options|group=authentication|state=ready|title=Options|notice=Some(\"Changes will be applied on save.\")|sections=[Video, Audio, Controls, Performance, Network, Locale]|toggles=[Fullscreen, VSync, ReduceEffects, MuteAudio, RememberServer, RememberUsername]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }

    #[test]
    fn options_progress_and_error_snapshots() {
        assert_eq!(
            options_screen(OptionsScreenState::Saving).snapshot(),
            "route=options|group=authentication|state=saving|title=Options|notice=Some(\"Saving settings...\")|sections=[Video, Audio, Controls, Performance, Network, Locale]|toggles=[Fullscreen, VSync, ReduceEffects, MuteAudio, RememberServer, RememberUsername]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
        assert_eq!(
            options_screen(OptionsScreenState::Error).snapshot(),
            "route=options|group=authentication|state=error|title=Options|notice=Some(\"Settings could not be saved.\")|sections=[Video, Audio, Controls, Performance, Network, Locale]|toggles=[Fullscreen, VSync, ReduceEffects, MuteAudio, RememberServer, RememberUsername]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }
}
