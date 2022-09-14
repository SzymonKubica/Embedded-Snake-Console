#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    NoDirection
}

impl Direction {
    pub fn get_opposite(direction: Direction) -> Direction {
        match direction {
            Direction::Left        => Direction::Right,
            Direction::Right       => Direction::Left,
            Direction::Up          => Direction::Down,
            Direction::Down        => Direction::Up ,
            Direction::NoDirection => Direction::NoDirection,
        }
    }
}
