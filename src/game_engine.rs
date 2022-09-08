use crate::mvc::{Direction, Model, View, Task};

pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    chosen_direction: Direction
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View) -> GameEngine {
        GameEngine {
            view,
            chosen_direction: Direction::NoDirection }
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: Direction) {
        self.chosen_direction = input;
    }

    fn set_view(&mut self, view: &mut dyn View) -> () {
        todo!()
    }
}

impl<'a> Task for GameEngine<'a> {
    fn run(&mut self) -> () {
        todo!()
    }
}
