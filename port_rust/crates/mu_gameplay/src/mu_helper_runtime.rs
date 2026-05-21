use bevy::prelude::{App, Plugin, Resource};

use crate::{MuHelperConfig, MuHelperConfigError};

const INVALID_CONFIG_NOTICE: &str = "Fix the helper rules before starting.";
const INACTIVE_NOTICE: &str = "Configure MU Helper before starting.";
const ACTIVE_NOTICE: &str = "MU Helper is running.";
const RESOURCE_LIMITED_NOTICE: &str = "MU Helper is waiting for resources.";
const SERVER_LIMITED_NOTICE: &str = "The server paused MU Helper.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MuHelperExecutionState {
    #[default]
    Inactive,
    Active,
    InvalidConfig,
    ResourceLimited,
    ServerLimited,
}

impl MuHelperExecutionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Active => "active",
            Self::InvalidConfig => "invalid-config",
            Self::ResourceLimited => "resource-limited",
            Self::ServerLimited => "server-limited",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Inactive => Some(INACTIVE_NOTICE),
            Self::Active => Some(ACTIVE_NOTICE),
            Self::InvalidConfig => Some(INVALID_CONFIG_NOTICE),
            Self::ResourceLimited => Some(RESOURCE_LIMITED_NOTICE),
            Self::ServerLimited => Some(SERVER_LIMITED_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MuHelperRuntime {
    config: MuHelperConfig,
    config_error: Option<MuHelperConfigError>,
    resource_limit_reason: Option<String>,
    server_limit_reason: Option<String>,
    active: bool,
    total_cost: u32,
}

impl Resource for MuHelperRuntime {}

#[derive(Debug, Default, Clone, Copy)]
pub struct MuHelperRuntimePlugin;

impl Plugin for MuHelperRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MuHelperRuntime>();
    }
}

impl MuHelperRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(&self) -> &MuHelperConfig {
        &self.config
    }

    pub fn execution_state(&self) -> MuHelperExecutionState {
        if self.config_error.is_some() {
            return MuHelperExecutionState::InvalidConfig;
        }

        if self.server_limit_reason.is_some() {
            return MuHelperExecutionState::ServerLimited;
        }

        if self.resource_limit_reason.is_some() {
            return MuHelperExecutionState::ResourceLimited;
        }

        if self.active {
            MuHelperExecutionState::Active
        } else {
            MuHelperExecutionState::Inactive
        }
    }

    pub fn status_detail(&self) -> Option<String> {
        if let Some(error) = &self.config_error {
            return Some(error.to_string());
        }

        if let Some(reason) = &self.server_limit_reason {
            return Some(reason.clone());
        }

        if let Some(reason) = &self.resource_limit_reason {
            return Some(reason.clone());
        }

        None
    }

    pub fn total_cost(&self) -> u32 {
        self.total_cost
    }

    pub fn is_active(&self) -> bool {
        matches!(self.execution_state(), MuHelperExecutionState::Active)
    }

    pub fn load_config(&mut self, config: MuHelperConfig) -> Result<(), MuHelperConfigError> {
        self.config = config;

        match self.config.validate() {
            Ok(()) => {
                self.config_error = None;
                Ok(())
            }
            Err(error) => {
                self.config_error = Some(error.clone());
                self.active = false;
                Err(error)
            }
        }
    }

    pub fn start(&mut self) -> bool {
        if let Err(error) = self.config.validate() {
            self.config_error = Some(error);
            self.active = false;
            return false;
        }

        self.config_error = None;

        if self.server_limit_reason.is_some() || self.resource_limit_reason.is_some() {
            self.active = false;
            return false;
        }

        self.active = true;
        true
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn set_resource_limit(&mut self, reason: impl Into<String>) {
        self.resource_limit_reason = Some(reason.into());
        self.active = false;
    }

    pub fn clear_resource_limit(&mut self) {
        self.resource_limit_reason = None;
    }

    pub fn set_server_limit(&mut self, reason: impl Into<String>) {
        self.server_limit_reason = Some(reason.into());
        self.active = false;
    }

    pub fn clear_server_limit(&mut self) {
        self.server_limit_reason = None;
    }

    pub fn add_cost(&mut self, cost: u32) {
        self.total_cost = self.total_cost.saturating_add(cost);
    }

    pub fn apply_server_status_update(
        &mut self,
        pause: bool,
        money: u32,
        consume_money: bool,
    ) -> bool {
        if pause {
            self.set_server_limit("server requested pause");
            return false;
        }

        self.clear_server_limit();
        let started = self.start();

        if started && consume_money && money > 0 {
            self.add_cost(money);
        }

        started
    }
}

#[cfg(test)]
mod tests {
    use super::{MuHelperExecutionState, MuHelperRuntime, MuHelperRuntimePlugin};
    use crate::{MuHelperConfig, MU_HELPER_MAX_HUNTING_RANGE};
    use bevy::prelude::App;

    #[test]
    fn runtime_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(MuHelperRuntimePlugin);

        let runtime = app.world().resource::<MuHelperRuntime>();
        assert_eq!(runtime.execution_state(), MuHelperExecutionState::Inactive);
        assert_eq!(runtime.total_cost(), 0);
        assert_eq!(runtime.config(), &MuHelperConfig::default());
    }

    #[test]
    fn runtime_tracks_active_inactive_and_cost() {
        let mut runtime = MuHelperRuntime::new();
        assert!(runtime.start());
        assert_eq!(runtime.execution_state(), MuHelperExecutionState::Active);

        runtime.add_cost(125);
        assert_eq!(runtime.total_cost(), 125);

        runtime.stop();
        assert_eq!(runtime.execution_state(), MuHelperExecutionState::Inactive);
    }

    #[test]
    fn runtime_tracks_resource_and_server_limits() {
        let mut runtime = MuHelperRuntime::new();

        runtime.set_resource_limit("missing potions");
        assert_eq!(
            runtime.execution_state(),
            MuHelperExecutionState::ResourceLimited
        );
        assert_eq!(runtime.status_detail(), Some("missing potions".to_string()));
        assert!(!runtime.start());

        runtime.clear_resource_limit();
        runtime.set_server_limit("server pause");
        assert_eq!(
            runtime.execution_state(),
            MuHelperExecutionState::ServerLimited
        );
        assert_eq!(runtime.status_detail(), Some("server pause".to_string()));
        assert!(!runtime.apply_server_status_update(true, 10, true));
    }

    #[test]
    fn runtime_marks_invalid_config() {
        let mut runtime = MuHelperRuntime::new();
        let config = MuHelperConfig {
            hunting_range: MU_HELPER_MAX_HUNTING_RANGE + 1,
            ..MuHelperConfig::default()
        };

        let error = runtime.load_config(config).unwrap_err();

        assert_eq!(
            runtime.execution_state(),
            MuHelperExecutionState::InvalidConfig
        );
        assert_eq!(runtime.status_detail(), Some(error.to_string()));
        assert!(!runtime.start());
    }

    #[test]
    fn server_status_update_resumes_and_accumulates_cost() {
        let mut runtime = MuHelperRuntime::new();

        assert!(runtime.apply_server_status_update(false, 175, true));
        assert_eq!(runtime.execution_state(), MuHelperExecutionState::Active);
        assert_eq!(runtime.total_cost(), 175);
        assert!(runtime.is_active());
    }
}
