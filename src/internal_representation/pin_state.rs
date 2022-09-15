#[derive(Copy, Clone, PartialEq)]
pub enum PinState {
    High, Low
}

impl From<bool> for PinState {
    fn from(state: bool) -> Self {
        if state { Self::High } else { Self::Low }
    }
}
