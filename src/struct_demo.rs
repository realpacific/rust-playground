use std::fmt::{Display, Error, Formatter};

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}


pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Rect with p1{} and p2{} = ", self.p1, self.p2)
    }
}

pub fn rect_area(rect: &Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rect.p1;
    let Point { x: x2, y: y2 } = rect.p2;

    let length = x2 - x1;
    let breadth = y2 - y1;
    (length * breadth).abs()
}

