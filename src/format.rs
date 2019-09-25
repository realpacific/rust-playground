use std::fmt::{Display, Formatter, Error};

// ------------------CITY-------------------
pub struct City {
    name: String,
    lat: f32,
    long: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "City({} @ {},{})", self.name, self.lat, self.long)
    }
}

pub fn provide_city() -> City {
    let city = City {
        name: "Patan".to_string(),
        lat: 27.89123,
        long: 86.23432,
    };
    return city;
}


pub fn print_cities() {
    let cities = [
        City { name: "Panauti".to_string(), lat: 27.8923, long: 86.34535 },
        City { name: "Banepa".to_string(), lat: 27.8923, long: 86.34532 },
        City { name: "Khopasi".to_string(), lat: 27.8923, long: 86.34533 }
    ];
    for (count, city) in cities.iter().enumerate() {
        println!("{}: {}", count, *city);
    }
}


// ------------------MATRIX-------------------

pub struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})\n", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}


pub fn provides_matrix() -> Matrix {
    let matrix = Matrix(1.9, 1.1, 1.2, 1.4);
    matrix
}


pub fn transpose(matrix: Matrix) -> Matrix {
    let new_matrix = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    new_matrix
}

