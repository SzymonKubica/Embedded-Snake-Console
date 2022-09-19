#[derive(Copy, Clone)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Point {
    pub fn new(x_coordinate: i8, y_coordinate: i8) -> Point {
        Point { x: x_coordinate, y: y_coordinate }
    }

    pub fn translate_up(&self)    -> Point { Point::new(self.x, self.y - 1) }
    pub fn translate_down(&self)  -> Point { Point::new(self.x, self.y + 1) }
    pub fn translate_left(&self)  -> Point { Point::new(self.x - 1, self.y) }
    pub fn translate_right(&self) -> Point { Point::new(self.x + 1, self.y) }
}
