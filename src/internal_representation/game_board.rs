// GameBoard is the internal representation of the game area where the snake
// moves and eats apples.

use crate::common::BOARD_SIZE;

use super::point::Point;

// The first column of the matrix doesn't work, hence we restrict the x range.
pub const X_LOWER_BOUND: usize = 1;
pub const X_UPPER_BOUND: usize = 7;
pub const Y_LOWER_BOUND: usize = 0;
pub const Y_UPPER_BOUND: usize = 7;

pub struct GameBoard {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let row: [Cell; BOARD_SIZE] = Default::default();
        let mut matrix: [[Cell; BOARD_SIZE]; BOARD_SIZE] = Default::default();

        for i in 0..BOARD_SIZE { matrix[i] = row; }
        GameBoard { board: matrix }
    }

    pub fn is_within_bounds(&self, point: Point) -> bool {
        X_LOWER_BOUND <= point.x && point.x <= X_UPPER_BOUND &&
        Y_LOWER_BOUND <= point.y && point.y <= Y_UPPER_BOUND
    }

    pub fn add_apple(&mut self, point: Point) {
        self.update_board_entry(point, Cell::Apple);
    }

    pub fn add_snake_segment(&mut self, point: Point) {
        self.update_board_entry(point, Cell::SnakeSegment);
    }

    pub fn erase_entry(&mut self, point: Point) {
        self.update_board_entry(point, Cell::Empty)
    }

    pub fn update_board_entry(&mut self, point: Point, value: Cell) {
        self.board[point.y][point.x] = value;
    }

    pub fn read_board_at(&mut self, point: Point) -> Cell {
        self.board[point.y][point.x]
    }
}

pub enum Cell {
    Apple, SnakeSegment, Empty
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}
