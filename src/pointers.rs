pub fn run() {
    {
        // & denotes reference
        let reference = &400i64;

        // &val will get i64
        // val will get &i64
        match reference {
            &val => println!("1. {}", val)
        }

        // Dereference before matching
        match *reference {
            val => println!("2. {}", val)
        }

        let not_a_ref = 3i32;

        // get reference using `ref`
        let ref a_ref = 3i32;

        match not_a_ref {
            // val is not &i32
            ref val => println!("Got a ref {:?}", val)
        }

        let mut mut_val = 88;
        match mut_val {
            // use `ref mut` to get reference
            ref mut m => {
                *m = *m + 1;
            }
        }
        // Value of `mut_val` changed after referential increment
        println!("After mutation {}", mut_val);

        match mut_val {
            // m is std::i32
            mut m => {
                m = m + 10;
            }
        }
        // Value of `mut_val` does not change
        println!("After Non reference assignment {}", mut_val);
    }

    // -------------------STRUCT------------------
    {
        // Destructuring a Struct
        #[derive(Debug)]
        struct City {
            gps: (f64, f64),
            name: String,
        }

        let city = City {
            gps: (27.34343, 86.3434),
            name: "A".to_string(),
        };

        match city {
            // This is match guard
            City { gps: (lat, lon), ref name } if *name == "A".to_string() => println!("The name is A"),
            City { gps: (lat, lon), name: name } => println!("{} {} {}", lat, lon, name),
        }
    }

    // -----------------BINDING--------------
    {
        fn get_age() -> u32 {
            22u32
        }

        let result = match get_age() {
            0 => "For real".to_string(),
            n @ 1..=12 => format!("{} {}", n, "y.o kid"),
            n @ 13..=19 => format!(" {} {}", n, "y.o teen"),
            n @ 20..=35 => format!("{} {}", n, "y.o mid aged"),
            n => format!("{} {}", n, "y.o old")
        };
        println!("At {}, you're a {}", get_age(), result);
    }
}