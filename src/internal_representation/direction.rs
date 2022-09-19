#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDirection
}

impl Direction {
    pub fn get_opposite(direction: Direction) -> Direction {
        match direction {
            Direction::Up          => Direction::Down,
            Direction::Down        => Direction::Up ,
            Direction::Left        => Direction::Right,
            Direction::Right       => Direction::Left,
            Direction::NoDirection => Direction::NoDirection,
        }
    }
}
