use arrayvec::ArrayVec;

use crate::mvc::Direction;
use crate::common::MAX_SNAKE_LENGTH;

pub struct Snake {
    segments: ArrayVec<SnakeSegment, MAX_SNAKE_LENGTH>,
    head: SnakeSegment,
    current_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            segments: ArrayVec::new(),
            head: SnakeSegment::new(0, 0),
            current_direction: Direction::Right
        }
    }

    pub fn move_snake(&mut self) {
        self.move_head(self.current_direction);
        self.advance_tail();
    }

    pub fn move_and_grow(&mut self) {
        self.move_head(self.current_direction);
    }

    pub fn move_head(&mut self, direction: Direction) {
        let new_head: SnakeSegment = match direction {
            Direction::Left        => self.head.translate_left(),
            Direction::Right       => self.head.translate_right(),
            Direction::Up          => self.head.translate_up(),
            Direction::Down        => self.head.translate_down(),
            Direction::NoDirection => self.head // Unreachable.
        };
        self.head = new_head;
        self.add_segment(self.head);
    }

    pub fn add_segment(&mut self, segment: SnakeSegment) {
        self.segments.push(segment)
    }


    pub fn advance_tail(&mut self) -> SnakeSegment {
        self.segments.remove(0)
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.current_direction = direction;
    }
}

#[derive(Copy, Clone)]
pub struct SnakeSegment {
    x_coordinate: u8,
    y_coordinate: u8,
}

impl SnakeSegment {
    pub fn new(x_coordinate: u8, y_coordinate: u8) -> SnakeSegment {
        SnakeSegment { x_coordinate, y_coordinate }
    }

    pub fn translate_left(&self) -> SnakeSegment {
        SnakeSegment::new(self.x_coordinate + 1, self.y_coordinate)
    }

    pub fn translate_right(&self) -> SnakeSegment {
        SnakeSegment::new(self.x_coordinate - 1, self.y_coordinate)
    }

    pub fn translate_up(&self) -> SnakeSegment {
        SnakeSegment::new(self.x_coordinate, self.y_coordinate + 1)
    }

    pub fn translate_down(&self) -> SnakeSegment {
        SnakeSegment::new(self.x_coordinate, self.y_coordinate - 1)
    }
}
