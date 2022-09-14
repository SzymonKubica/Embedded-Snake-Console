use super::direction::Direction;

pub struct ControllerInput {
    pub toggle_signal: bool,
    pub direction: Direction
}

impl ControllerInput {
    pub fn new(toggle_signal: bool, direction: Direction) -> ControllerInput {
        ControllerInput { toggle_signal, direction }
    }
}

impl Default for ControllerInput {
    fn default() -> Self {
        Self { toggle_signal: false, direction: Direction::NoDirection }
    }
}

