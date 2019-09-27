pub fn run() {
    use std::mem;

    let color = "green";

    let print = || println!("{}", color);
    print();
    print();

    let mut count = 0;

    let mut increment = || count += 1;

    increment();
    increment();
    increment();
    increment();
    println!("{}", count);
}