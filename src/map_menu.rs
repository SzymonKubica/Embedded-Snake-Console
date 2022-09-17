use crate::common::BOARD_SIZE;
use crate::internal_representation::game_board::BoardCell;
use crate::libs::time_util::millis;
use crate::user_interface::{self, MAPS_NUMBER};

const INTERACTION_INTERVAL: u32 = 500; // miliseconds

pub struct MapMenu {
    current_map_index: usize,
    last_interaction_timestamp: u32
}
impl MapMenu {
    pub fn new() -> MapMenu {
        MapMenu { current_map_index: 0, last_interaction_timestamp: millis() }
    }

    pub fn is_time_for_interaction(&self) -> bool {
        millis() - self.last_interaction_timestamp >= INTERACTION_INTERVAL
    }

    pub fn register_interaction_at(&mut self, timestamp: u32) {
        self.last_interaction_timestamp = timestamp;
    }

    pub fn print_current_map(&self) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
        match self.current_map_index {
            0 => user_interface::print_up_down_arrows(),
            _ => self.get_current_map()
        }
    }

    pub fn get_current_map(&self) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
        user_interface::print_map(self.current_map_index)
    }

    pub fn scroll_down(&mut self) {
        self.current_map_index = (self.current_map_index + MAPS_NUMBER - 1) % MAPS_NUMBER;
    }

    pub fn scroll_up(&mut self) {
        self.current_map_index = (self.current_map_index + 1) % MAPS_NUMBER;
    }
}
