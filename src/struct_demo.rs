use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy)]
struct Point {
    pub x: f32,
    pub y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}


struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Rect with p1{} and p2{}", self.p1, self.p2)
    }
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = rect.p1;
    let Point { x: x2, y: y2 } = rect.p2;

    let length = x2 - x1;
    let breadth = y2 - y1;
    (length * breadth).abs()
}


fn square(point: &Point, side: f32) -> Rectangle {
    let new_point = Point {
        x: point.x + side,
        y: point.y + side,
    };
    Rectangle {
        p1: *point,
        p2: new_point,
    }
}

pub fn run() {
    println!("\n");
    let rect = Rectangle {
        p1: Point { x: 1.0, y: 0.0 },
        p2: Point { x: 10.0, y: 10.0 },
    };
    println!("Area of rectangle {:} = {:?}", rect, rect_area(&rect));
    let side = 5_f32;
    let square = square(&rect.p1, side);
    println!("The square with width of {} is {:}", side, square);
    println!("Area of square= {:?}", rect_area(&square));
}