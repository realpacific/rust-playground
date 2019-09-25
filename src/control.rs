pub fn run() {
    let x = 10i32;
    let y = {
        2 * x
    };
    println!("y = {}", y);


    let mut i = 0;
    'outer: loop {
        let mut j = 0;
        'inner: loop {
            if j == 10 { break 'inner; }
            print!("*");
            j = j + 1;
        }
        println!();
        if i == 5 { break 'outer; }
        i = i + 1;
    }


    let list = vec![1, 2, 3, 4, 5, 6, 7];
    for item in list.iter() {
        if item % 2 == 0 { println!("{} is even.", item); }
    }
    for item in list.into_iter() {
        if item % 2 != 0 { println!("{} is odd.", item); }
    }

    let mut names = vec!["hello", "world", "!"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "hello" => "Hello",
            _ => "World"
        }
    }
    println!("{:?}", names)
}