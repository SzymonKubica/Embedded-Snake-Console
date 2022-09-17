use crate::common::{BOARD_SIZE, MAX_SCORE};
use crate::internal_representation::game_board::BoardCell;
use crate::libs::time_util::millis;
use crate::user_interface::{self as UI, MAPS_NUMBER};

const INTERACTION_INTERVAL: u32 = 500; // miliseconds

pub struct Map {
    current_map_index: usize,
    last_interaction_timestamp: u32
}
impl Map {
    pub fn new() -> Map {
        Map { current_map_index: 0, last_interaction_timestamp: millis() }
    }

    pub fn is_time_for_interaction(&self) -> bool {
        millis() - self.last_interaction_timestamp >= INTERACTION_INTERVAL
    }

    pub fn register_interaction_at(&mut self, timestamp: u32) {
        self.last_interaction_timestamp = timestamp;
    }

    pub fn print_current_map(&self) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
        match self.current_map_index {
            0 => UI::print_up_down_arrows(),
            _ => self.get_current_map()
        }
    }

    pub fn get_current_map(&self) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
        UI::print_map(self.current_map_index)
    }

    pub fn get_max_score(&self) -> usize {
        MAX_SCORE - count_obstacles(self.get_current_map())
    }

    pub fn get_previous(&mut self) {
        self.current_map_index = (self.current_map_index + MAPS_NUMBER - 1) % MAPS_NUMBER;
    }

    pub fn get_next(&mut self) {
        self.current_map_index = (self.current_map_index + 1) % MAPS_NUMBER;
    }
}

fn count_obstacles(map: [[BoardCell; BOARD_SIZE]; BOARD_SIZE]) -> usize {
    let mut count: usize = 0;
    for row in map {
        row.iter()
            .filter(|cell| **cell == BoardCell::Obstacle)
            .for_each(|_| count += 1); }
    count
}
