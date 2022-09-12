use arrayvec::ArrayVec;

use crate::mvc::Direction;

pub const DIMENSION: usize = 8;
pub const MAX_SNAKE_LENGTH: usize = DIMENSION * DIMENSION;
// The first column of the matrix doesn't work, hence we restrict the x range.
pub const X_LOWER_BOUND: usize = 1;
pub const X_UPPER_BOUND: usize = 7;
pub const Y_LOWER_BOUND: usize = 0;
pub const Y_UPPER_BOUND: usize = 7;

pub struct Snake {
    pub segments: ArrayVec<Point, MAX_SNAKE_LENGTH>,
    pub head: Point,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            segments: ArrayVec::new(),
            head: Point::new(0, 0),
            direction: Direction::Right
        }
    }

    pub fn move_head(&mut self) {
        let new_head: Point = match self.direction {
            Direction::Left        => self.head.translate_left(),
            Direction::Right       => self.head.translate_right(),
            Direction::Up          => self.head.translate_up(),
            Direction::Down        => self.head.translate_down(),
            Direction::NoDirection => self.head // Unreachable.
        };
        self.head = new_head;
        self.add_segment(self.head);
    }

    pub fn move_tail(&mut self) -> Point {
        self.segments.remove(0)
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn add_segment(&mut self, segment: Point) {
        self.segments.push(segment)
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x_coordinate: usize, y_coordinate: usize) -> Point {
        Point { x: x_coordinate, y: y_coordinate }
    }

    pub fn translate_left(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn translate_right(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }

    pub fn translate_up(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }

    pub fn translate_down(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
}

