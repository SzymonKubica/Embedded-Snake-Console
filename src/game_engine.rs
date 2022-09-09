use arrayvec::ArrayVec;

use crate::mvc::{Direction, Model, View, Task};

const MAX_SNAKE_LENGTH: usize = 56;

pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    game_board: [[u8; 7]; 8], // The board is an 8x7 matrix
    score: u8,
    chosen_direction: Direction
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View) -> GameEngine {
        GameEngine {
            view,
            game_board: initialize_board(),
            score: 0,
            chosen_direction: Direction::NoDirection }
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: Direction) {
        self.chosen_direction = input;
    }
}

impl<'a> Task for GameEngine<'a> {
    fn run(&mut self) -> () {
        todo!()
    }
}

fn initialize_board() -> [[u8; 7]; 8] {
    let row: [u8; 7] = Default::default();
    let mut matrix: [[u8; 7]; 8] = Default::default();

    for i in 0..8_usize {
        matrix[i] = row;
    }

    matrix
}

struct Snake {
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
struct SnakeSegment {
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
