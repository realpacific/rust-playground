use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point { x: 0f64, y: 0f64 }
    }
}


struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(self: &Self) -> f64 {
        ((self.p2.x - self.p1.x) * (self.p2.y - self.p1.y)).abs()
    }

    fn perimeter(&self) -> f64 {
        2f64 * ((self.p2.x - self.p1.x) + (self.p2.y - self.p1.y)).abs()
    }

    fn translate(self: &mut Self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} {:?}", self.p1, self.p2)
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn consume(self) {
        println!("{}: {}", self.0, self.1)
    }
}

pub fn run() {
    let mut rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(5f64, 5f64),
    };
    println!("Area of rectangle = {}", rect.area());
    println!("Perimeter of rectangle = {}", rect.perimeter());

    rect.translate(5f64, 5f64);
    println!("Rectangle after translation = {}", rect);

    let pair = Pair(Box::new(1i32), Box::new(2i32));
    println!("{:?}", pair);
    pair.consume();

}