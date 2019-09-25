use std::convert::From;

#[derive(Debug)]
struct Number {
    value: f64
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {
            value: item as f64
        }
    }
}


impl From<String> for Number {
    fn from(item: String) -> Self {
        Number {
            value: item.parse().unwrap()
        }
    }
}


pub fn run() {
    // Using `From` traits
    println!("{:?}", Number::from(89i32));
    println!("{:?}", Number::from("23".to_string()));

    // The traits `Into` requires explicit type.
    let num: Number = 5.into();
    println!("{:?}", num);

    let parsed: i32 = "55".parse::<i32>().unwrap();
    println!("Parsed value is {}", parsed);

}