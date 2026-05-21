use crate::Bindings;
use serde::{Deserialize, Serialize};

const MOVEMENT_BINDING_PAIRS: &[(&str, &str)] = &[
    ("move_forward", "W"),
    ("move_backward", "S"),
    ("move_left", "A"),
    ("move_right", "D"),
    ("jump", "Space"),
    ("attack", "Mouse1"),
    ("interact", "E"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MovementBindings(Bindings);

impl MovementBindings {
    pub fn defaults() -> Self {
        Self(Bindings::from_pairs(MOVEMENT_BINDING_PAIRS))
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
        snapshot_bindings(&self.0, MOVEMENT_BINDING_PAIRS)
    }
}

impl Default for MovementBindings {
    fn default() -> Self {
        Self::defaults()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MovementCommand {
    pub source_x: u8,
    pub source_y: u8,
    pub step_count: u8,
    pub target_rotation: u8,
    pub directions: Vec<u8>,
}

impl MovementCommand {
    pub fn new(
        source_x: u8,
        source_y: u8,
        step_count: u8,
        target_rotation: u8,
        directions: impl IntoIterator<Item = u8>,
    ) -> Self {
        Self {
            source_x,
            source_y,
            step_count,
            target_rotation,
            directions: directions.into_iter().collect(),
        }
    }

    pub fn is_idle(&self) -> bool {
        self.step_count == 0 && self.directions.is_empty()
    }

    pub fn snapshot(&self) -> String {
        format!(
            "source={},{}|steps={}|rotation={}|directions=[{}]",
            self.source_x,
            self.source_y,
            self.step_count,
            self.target_rotation,
            format_u8_list(&self.directions),
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

fn format_u8_list(values: &[u8]) -> String {
    values
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::{MovementBindings, MovementCommand};

    #[test]
    fn movement_bindings_cover_the_core_actions() {
        let bindings = MovementBindings::default();

        assert_eq!(bindings.get("move_forward"), Some("W"));
        assert_eq!(bindings.get("move_left"), Some("A"));
        assert_eq!(bindings.get("jump"), Some("Space"));
        assert_eq!(
            bindings.snapshot(),
            "move_forward=W,move_backward=S,move_left=A,move_right=D,jump=Space,attack=Mouse1,interact=E"
        );
    }

    #[test]
    fn movement_command_snapshots_are_stable() {
        let command = MovementCommand::new(1, 2, 3, 4, [0x12, 0x34]);

        assert!(!command.is_idle());
        assert_eq!(
            command.snapshot(),
            "source=1,2|steps=3|rotation=4|directions=[18,52]"
        );
    }
}
