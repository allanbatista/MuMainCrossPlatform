use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_gameplay::{MuHelperConfig, MuHelperDarkRavenMode, MuHelperExecutionState, MuHelperRuntime};

const MU_HELPER_TITLE: &str = "MU Helper";
const INACTIVE_NOTICE: &str = "Configure MU Helper before starting.";
const ACTIVE_NOTICE: &str = "MU Helper is running.";
const INVALID_NOTICE: &str = "Fix the helper rules before starting.";
const RESOURCE_LIMITED_NOTICE: &str = "MU Helper is waiting for resources.";
const SERVER_LIMITED_NOTICE: &str = "The server paused MU Helper.";

const INACTIVE_ACTIONS: &[MuHelperAction] = &[
    MuHelperAction::Start,
    MuHelperAction::Save,
    MuHelperAction::Reset,
    MuHelperAction::Close,
];

const ACTIVE_ACTIONS: &[MuHelperAction] = &[
    MuHelperAction::Stop,
    MuHelperAction::Save,
    MuHelperAction::Reset,
    MuHelperAction::Close,
];

const INVALID_ACTIONS: &[MuHelperAction] = &[
    MuHelperAction::FixConfig,
    MuHelperAction::Save,
    MuHelperAction::Reset,
    MuHelperAction::Close,
];

const LIMITED_ACTIONS: &[MuHelperAction] = &[
    MuHelperAction::Retry,
    MuHelperAction::Save,
    MuHelperAction::Reset,
    MuHelperAction::Close,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuHelperAction {
    Start,
    Stop,
    Save,
    Reset,
    Retry,
    FixConfig,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MuHelperConfigSummary {
    pub hunting_range: u8,
    pub obtaining_range: u8,
    pub max_seconds_away: u8,
    pub long_range_counter_attack: bool,
    pub return_to_original_position: bool,
    pub basic_skill_id: u16,
    pub activation_skill_ids: [u16; 2],
    pub buff_skill_ids: [u16; 3],
    pub use_combo: bool,
    pub potion_threshold: u8,
    pub heal_threshold: u8,
    pub heal_party_threshold: u8,
    pub buff_cast_interval: u16,
    pub use_heal_potion: bool,
    pub use_drain_life: bool,
    pub use_dark_raven: bool,
    pub dark_raven_mode: MuHelperDarkRavenMode,
    pub repair_item: bool,
    pub pick_extra_items: bool,
    pub extra_items: usize,
}

impl MuHelperConfigSummary {
    pub fn from_config(config: &MuHelperConfig) -> Self {
        Self {
            hunting_range: config.hunting_range,
            obtaining_range: config.obtaining_range,
            max_seconds_away: config.max_seconds_away,
            long_range_counter_attack: config.long_range_counter_attack,
            return_to_original_position: config.return_to_original_position,
            basic_skill_id: config.basic_skill_id,
            activation_skill_ids: [
                config.activation_skills[0].skill_id,
                config.activation_skills[1].skill_id,
            ],
            buff_skill_ids: config.buff_skill_ids,
            use_combo: config.use_combo,
            potion_threshold: config.potion_threshold,
            heal_threshold: config.heal_threshold,
            heal_party_threshold: config.heal_party_threshold,
            buff_cast_interval: config.buff_cast_interval,
            use_heal_potion: config.use_heal_potion,
            use_drain_life: config.use_drain_life,
            use_dark_raven: config.use_dark_raven,
            dark_raven_mode: config.dark_raven_mode,
            repair_item: config.repair_item,
            pick_extra_items: config.pick_extra_items,
            extra_items: config.extra_items.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MuHelperScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: MuHelperScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub status_detail: Option<String>,
    pub config: MuHelperConfigSummary,
    pub total_cost: u32,
    pub actions: &'static [MuHelperAction],
}

pub use mu_gameplay::MuHelperExecutionState as MuHelperScreenState;

pub fn mu_helper_screen(runtime: &MuHelperRuntime) -> MuHelperScreen {
    let state = runtime.execution_state();
    let (notice, actions) = match state {
        MuHelperExecutionState::Inactive => (Some(INACTIVE_NOTICE), INACTIVE_ACTIONS),
        MuHelperExecutionState::Active => (Some(ACTIVE_NOTICE), ACTIVE_ACTIONS),
        MuHelperExecutionState::InvalidConfig => (Some(INVALID_NOTICE), INVALID_ACTIONS),
        MuHelperExecutionState::ResourceLimited => (Some(RESOURCE_LIMITED_NOTICE), LIMITED_ACTIONS),
        MuHelperExecutionState::ServerLimited => (Some(SERVER_LIMITED_NOTICE), LIMITED_ACTIONS),
    };

    MuHelperScreen {
        route: UiRoute::MuHelper,
        layout: UiShellLayout::for_route(UiRoute::MuHelper),
        widgets: UiShellWidgetSet::for_route(UiRoute::MuHelper),
        state,
        title: MU_HELPER_TITLE,
        notice,
        status_detail: runtime.status_detail(),
        config: MuHelperConfigSummary::from_config(runtime.config()),
        total_cost: runtime.total_cost(),
        actions,
    }
}

impl MuHelperScreen {
    pub fn snapshot(&self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|status_detail={:?}|config={:?}|total_cost={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.status_detail,
            self.config,
            self.total_cost,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        mu_helper_screen, MuHelperConfigSummary, MuHelperScreenState, ACTIVE_ACTIONS,
        INACTIVE_ACTIONS, INVALID_ACTIONS, LIMITED_ACTIONS,
    };
    use mu_gameplay::{MuHelperConfig, MuHelperRuntime};

    fn default_summary() -> MuHelperConfigSummary {
        MuHelperConfigSummary::from_config(&MuHelperConfig::default())
    }

    #[test]
    fn inactive_and_active_snapshots_include_actions_and_config_summary() {
        let runtime = MuHelperRuntime::new();
        let inactive = mu_helper_screen(&runtime);

        assert_eq!(
            inactive.snapshot(),
            "route=mu-helper|group=gameplay|state=inactive|title=MU Helper|notice=Some(\"Configure MU Helper before starting.\")|status_detail=None|config=MuHelperConfigSummary { hunting_range: 6, obtaining_range: 8, max_seconds_away: 10, long_range_counter_attack: false, return_to_original_position: true, basic_skill_id: 0, activation_skill_ids: [0, 0], buff_skill_ids: [0, 0, 0], use_combo: false, potion_threshold: 40, heal_threshold: 60, heal_party_threshold: 60, buff_cast_interval: 0, use_heal_potion: false, use_drain_life: false, use_dark_raven: false, dark_raven_mode: Cease, repair_item: false, pick_extra_items: false, extra_items: 0 }|total_cost=0|actions=[Start, Save, Reset, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(inactive.actions, INACTIVE_ACTIONS);
        assert_eq!(inactive.config, default_summary());

        let mut runtime = MuHelperRuntime::new();
        assert!(runtime.start());
        runtime.add_cost(175);
        let active = mu_helper_screen(&runtime);

        assert_eq!(
            active.snapshot(),
            "route=mu-helper|group=gameplay|state=active|title=MU Helper|notice=Some(\"MU Helper is running.\")|status_detail=None|config=MuHelperConfigSummary { hunting_range: 6, obtaining_range: 8, max_seconds_away: 10, long_range_counter_attack: false, return_to_original_position: true, basic_skill_id: 0, activation_skill_ids: [0, 0], buff_skill_ids: [0, 0, 0], use_combo: false, potion_threshold: 40, heal_threshold: 60, heal_party_threshold: 60, buff_cast_interval: 0, use_heal_potion: false, use_drain_life: false, use_dark_raven: false, dark_raven_mode: Cease, repair_item: false, pick_extra_items: false, extra_items: 0 }|total_cost=175|actions=[Stop, Save, Reset, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
        assert_eq!(active.actions, ACTIVE_ACTIONS);
    }

    #[test]
    fn blocked_states_surface_their_reasons() {
        let mut runtime = MuHelperRuntime::new();

        let error = runtime
            .load_config(MuHelperConfig {
                hunting_range: 7,
                ..MuHelperConfig::default()
            })
            .unwrap_err();
        let invalid = mu_helper_screen(&runtime);

        assert_eq!(invalid.state, MuHelperScreenState::InvalidConfig);
        assert_eq!(invalid.actions, INVALID_ACTIONS);
        assert_eq!(invalid.status_detail, Some(error.to_string()));
        assert_eq!(
            invalid.snapshot(),
            "route=mu-helper|group=gameplay|state=invalid-config|title=MU Helper|notice=Some(\"Fix the helper rules before starting.\")|status_detail=Some(\"hunting range 7 exceeds the legacy limit of 6\")|config=MuHelperConfigSummary { hunting_range: 7, obtaining_range: 8, max_seconds_away: 10, long_range_counter_attack: false, return_to_original_position: true, basic_skill_id: 0, activation_skill_ids: [0, 0], buff_skill_ids: [0, 0, 0], use_combo: false, potion_threshold: 40, heal_threshold: 60, heal_party_threshold: 60, buff_cast_interval: 0, use_heal_potion: false, use_drain_life: false, use_dark_raven: false, dark_raven_mode: Cease, repair_item: false, pick_extra_items: false, extra_items: 0 }|total_cost=0|actions=[FixConfig, Save, Reset, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        runtime = MuHelperRuntime::new();
        runtime.set_resource_limit("missing potions");
        let resource_limited = mu_helper_screen(&runtime);
        assert_eq!(resource_limited.state, MuHelperScreenState::ResourceLimited);
        assert_eq!(resource_limited.actions, LIMITED_ACTIONS);
        assert_eq!(
            resource_limited.snapshot(),
            "route=mu-helper|group=gameplay|state=resource-limited|title=MU Helper|notice=Some(\"MU Helper is waiting for resources.\")|status_detail=Some(\"missing potions\")|config=MuHelperConfigSummary { hunting_range: 6, obtaining_range: 8, max_seconds_away: 10, long_range_counter_attack: false, return_to_original_position: true, basic_skill_id: 0, activation_skill_ids: [0, 0], buff_skill_ids: [0, 0, 0], use_combo: false, potion_threshold: 40, heal_threshold: 60, heal_party_threshold: 60, buff_cast_interval: 0, use_heal_potion: false, use_drain_life: false, use_dark_raven: false, dark_raven_mode: Cease, repair_item: false, pick_extra_items: false, extra_items: 0 }|total_cost=0|actions=[Retry, Save, Reset, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );

        runtime = MuHelperRuntime::new();
        runtime.set_server_limit("server requested pause");
        let server_limited = mu_helper_screen(&runtime);
        assert_eq!(server_limited.state, MuHelperScreenState::ServerLimited);
        assert_eq!(server_limited.actions, LIMITED_ACTIONS);
        assert_eq!(
            server_limited.snapshot(),
            "route=mu-helper|group=gameplay|state=server-limited|title=MU Helper|notice=Some(\"The server paused MU Helper.\")|status_detail=Some(\"server requested pause\")|config=MuHelperConfigSummary { hunting_range: 6, obtaining_range: 8, max_seconds_away: 10, long_range_counter_attack: false, return_to_original_position: true, basic_skill_id: 0, activation_skill_ids: [0, 0], buff_skill_ids: [0, 0, 0], use_combo: false, potion_threshold: 40, heal_threshold: 60, heal_party_threshold: 60, buff_cast_interval: 0, use_heal_potion: false, use_drain_life: false, use_dark_raven: false, dark_raven_mode: Cease, repair_item: false, pick_extra_items: false, extra_items: 0 }|total_cost=0|actions=[Retry, Save, Reset, Close]|layout=UiShellLayout { group: Gameplay, outer_margin: 20.0, panel_gap: 16.0, sidebar_width: 320.0, content_max_width: 1200.0, footer_height: 36.0 }|widgets=[Body, Sidebar, ActionBar, Overlay]"
        );
    }
}
