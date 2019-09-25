use crate::struct_demo::{Rectangle, Point};

mod format;
mod arrays_and_slices;
mod struct_demo;

fn main() {
//    println!("Hello, world!");
//    println!("\nCity: {} \n", format::provide_city());
//    format::print_cities();
//
//    println!("\nMatrix:\n{}\n", format::provides_matrix());
//    println!("\nTranspose Matrix:\n{}\n", format::transpose(format::provides_matrix()));
//
//
//    println!("{:?}", arrays_and_slices::provides_arrays());

    let rect = Rectangle {
        p1: Point { x: 1.0, y: 0.0 },
        p2: Point { x: 10.0, y: 10.0 },
    };
    print!("Area of rectangle {:} {:?}", rect, struct_demo::rect_area(&rect));
}

