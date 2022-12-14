use arrayvec::ArrayVec;

use crate::common::BOARD_SIZE;

use super::point::Point;
use super::direction::Direction;

pub const MAX_SNAKE_LENGTH: usize = BOARD_SIZE * BOARD_SIZE;

pub struct Snake {
    pub segments: ArrayVec<Point, MAX_SNAKE_LENGTH>,
    pub head: Point,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let head = Point::new(0, 0);
        let mut segments: ArrayVec<Point, MAX_SNAKE_LENGTH> = ArrayVec::new();
        segments.push(head);

        Snake { segments, head, direction: Direction::Right }
    }

    pub fn look_ahead(&mut self) -> Point {
        match self.direction {
            Direction::Up          => self.head.translate_up(),
            Direction::Down        => self.head.translate_down(),
            Direction::Left        => self.head.translate_left(),
            Direction::Right       => self.head.translate_right(),
            Direction::NoDirection => self.head // Shouldn't be reached.
        }
    }
    pub fn move_forward(&mut self) {
        self.head = self.look_ahead();
        self.segments.push(self.head);
    }

    pub fn advance_tail(&mut self) -> Point {
        self.segments.remove(0)
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // We don't allow changing the direction of the snake to the opposite
        // of the current direction. That would cause the snake to crash into
        // itself. We also don't overwrite the direction if it didn't change.
        if self.direction == new_direction ||
           self.direction == Direction::get_opposite(new_direction) { return; }

        match new_direction {
            Direction::NoDirection => (), // Snake cannot be stationary.
            _ => self.direction = new_direction
        };
    }
}


