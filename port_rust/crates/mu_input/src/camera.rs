use crate::Bindings;
use serde::{Deserialize, Serialize};

const CAMERA_BINDING_PAIRS: &[(&str, &str)] = &[
    ("cycle_camera", "F9"),
    ("toggle_zoom_lock", "F10"),
    ("reset_camera", "F11"),
    ("zoom", "MouseWheel"),
    ("rotate_orbital", "MouseDrag"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CameraBindings(Bindings);

impl CameraBindings {
    pub fn defaults() -> Self {
        Self(Bindings::from_pairs(CAMERA_BINDING_PAIRS))
    }

    pub fn from_bindings(bindings: Bindings) -> Self {
        Self(bindings)
    }

    pub fn get(&self, action: &str) -> Option<&str> {
        self.0.get(action)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter()
    }

    pub fn into_inner(self) -> Bindings {
        self.0
    }

    pub fn snapshot(&self) -> String {
        snapshot_bindings(&self.0, CAMERA_BINDING_PAIRS)
    }
}

impl Default for CameraBindings {
    fn default() -> Self {
        Self::defaults()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum CameraMode {
    #[default]
    Default,
    Orbital,
    FreeFly,
}

impl CameraMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Orbital => "orbital",
            Self::FreeFly => "free-fly",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CameraTourState {
    pub mode: CameraMode,
    pub waypoint_index: usize,
    pub waypoint_total: usize,
    pub looped: bool,
    pub blend_distance: f64,
    pub interpolation: String,
    pub position: Option<[f64; 3]>,
    pub look_at: Option<[f64; 3]>,
    pub move_acceleration: Option<f64>,
    pub distance_level: Option<f64>,
    pub delay: Option<f64>,
}

impl Default for CameraTourState {
    fn default() -> Self {
        Self {
            mode: CameraMode::Default,
            waypoint_index: 0,
            waypoint_total: 0,
            looped: false,
            blend_distance: 0.0,
            interpolation: "linear".to_string(),
            position: None,
            look_at: None,
            move_acceleration: None,
            distance_level: None,
            delay: None,
        }
    }
}

impl CameraTourState {
    pub fn new(
        mode: CameraMode,
        waypoint_total: usize,
        blend_distance: f64,
        interpolation: impl Into<String>,
        looped: bool,
    ) -> Self {
        Self {
            mode,
            waypoint_total,
            blend_distance,
            interpolation: interpolation.into(),
            looped,
            ..Self::default()
        }
    }

    pub fn with_waypoint(
        mut self,
        waypoint_index: usize,
        position: [f64; 3],
        look_at: [f64; 3],
        move_acceleration: f64,
        distance_level: f64,
        delay: f64,
    ) -> Self {
        self.waypoint_index = waypoint_index;
        self.position = Some(position);
        self.look_at = Some(look_at);
        self.move_acceleration = Some(move_acceleration);
        self.distance_level = Some(distance_level);
        self.delay = Some(delay);
        self
    }

    pub fn advance(&mut self) -> bool {
        if self.waypoint_total == 0 {
            return false;
        }

        let last_index = self.waypoint_total - 1;
        if self.waypoint_index < last_index {
            self.waypoint_index += 1;
            true
        } else if self.looped {
            self.waypoint_index = 0;
            true
        } else {
            false
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "mode={}|waypoint={}/{}|loop={}|blend={}|interpolation={}|position={}|look_at={}|move_acceleration={}|distance_level={}|delay={}",
            self.mode.as_str(),
            self.waypoint_index,
            self.waypoint_total,
            self.looped,
            format_float(self.blend_distance),
            self.interpolation,
            format_optional_triplet(self.position),
            format_optional_triplet(self.look_at),
            format_optional_float(self.move_acceleration),
            format_optional_float(self.distance_level),
            format_optional_float(self.delay),
        )
    }
}

fn snapshot_bindings(bindings: &Bindings, pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(action, _)| format!("{action}={}", bindings.get(action).unwrap_or("unset")))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_optional_triplet(values: Option<[f64; 3]>) -> String {
    values
        .map(format_triplet)
        .unwrap_or_else(|| "none".to_string())
}

fn format_optional_float(value: Option<f64>) -> String {
    value
        .map(format_float)
        .unwrap_or_else(|| "none".to_string())
}

fn format_triplet(values: [f64; 3]) -> String {
    [
        format_float(values[0]),
        format_float(values[1]),
        format_float(values[2]),
    ]
    .join(",")
}

fn format_float(value: f64) -> String {
    let mut text = format!("{value:.3}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::{CameraBindings, CameraMode, CameraTourState};

    #[test]
    fn camera_bindings_cover_the_core_controls() {
        let bindings = CameraBindings::default();

        assert_eq!(bindings.get("cycle_camera"), Some("F9"));
        assert_eq!(bindings.get("reset_camera"), Some("F11"));
        assert_eq!(
            bindings.snapshot(),
            "cycle_camera=F9,toggle_zoom_lock=F10,reset_camera=F11,zoom=MouseWheel,rotate_orbital=MouseDrag"
        );
    }

    #[test]
    fn camera_tour_state_snapshots_are_stable() {
        let camera = CameraTourState::new(CameraMode::Orbital, 8, 300.0, "smooth", true)
            .with_waypoint(
                0,
                [14700.0, 490.0, 12800.0],
                [13095.641, 170.0, 12922.459],
                16.0,
                8.0,
                0.0,
            );

        assert_eq!(
            camera.snapshot(),
            "mode=orbital|waypoint=0/8|loop=true|blend=300|interpolation=smooth|position=14700,490,12800|look_at=13095.641,170,12922.459|move_acceleration=16|distance_level=8|delay=0"
        );
    }
}
