use arrayvec::ArrayVec;

use crate::{mvc::Direction, common::BOARD_SIZE};

use super::point::Point;

pub const MAX_SNAKE_LENGTH: usize = BOARD_SIZE * BOARD_SIZE;

pub struct Snake {
    pub segments: ArrayVec<Point, MAX_SNAKE_LENGTH>,
    pub head: Point,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let head = Point::new(0, 0);
        let mut segments: ArrayVec<Point, MAX_SNAKE_LENGTH> = ArrayVec::new();
        segments.push(head);

        Snake {
            segments,
            head,
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


