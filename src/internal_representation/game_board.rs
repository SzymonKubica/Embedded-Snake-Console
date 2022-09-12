// GameBoard is the internal representation of the game area where the snake
// moves and eats apples.

pub struct GameBoard {
    board: [[Cell; DIMENSION]; DIMENSION],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let row: [Cell; DIMENSION] = Default::default();
        let mut matrix: [[Cell; DIMENSION]; DIMENSION] = Default::default();

        for i in 0..DIMENSION { matrix[i] = row; }
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
