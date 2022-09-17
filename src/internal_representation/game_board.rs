// GameBoard is the internal representation of the game area where the snake
// moves and eats apples.

use crate::common::BOARD_SIZE;

use super::point::Point;

// The first column of the matrix doesn't work, hence we restrict the x range.
pub const X_LOWER_BOUND: usize = 0;
pub const X_UPPER_BOUND: usize = 7;
pub const Y_LOWER_BOUND: usize = 0;
pub const Y_UPPER_BOUND: usize = 7;

pub struct GameBoard {
    board: [[BoardCell; BOARD_SIZE]; BOARD_SIZE],
}

impl GameBoard {
    pub fn new(board: [[BoardCell; BOARD_SIZE]; BOARD_SIZE]) -> GameBoard {
        GameBoard { board }
    }
    pub fn is_within_bounds(&self, point: Point) -> bool {
        let x = point.x as usize;
        let y = point.y as usize;
        X_LOWER_BOUND <= x && x <= X_UPPER_BOUND &&
        Y_LOWER_BOUND <= y && y <= Y_UPPER_BOUND
    }

    pub fn add_apple(&mut self, point: Point) {
        self.update_board_entry(point, BoardCell::Apple);
    }

    pub fn add_snake_segment(&mut self, point: Point) {
        self.update_board_entry(point, BoardCell::Snake);
    }

    pub fn erase_entry(&mut self, point: Point) {
        self.update_board_entry(point, BoardCell::Empty)
    }

    pub fn update_board_entry(&mut self, point: Point, value: BoardCell) {
        self.board[point.y as usize][point.x as usize] = value;
    }

    pub fn read_board_at(&mut self, point: Point) -> BoardCell {
        self.board[point.y as usize][point.x as usize]
    }

    pub fn get_screen(&self) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
        self.board
    }

    pub fn reset(&mut self) {
        self.board = Default::default();
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        Self { board: Default::default() }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BoardCell {
    Apple, Snake, Empty, Obstacle,
}

impl Default for BoardCell {
    fn default() -> Self {
        Self::Empty
    }
}
impl BoardCell {
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}
