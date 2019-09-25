use std::mem;

pub fn provides_arrays() -> usize {
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let array2: [i32; 5] = [6, 7, 8, 9, 10];

    slice(&array1);
    slice(&array2[0..3]);

    return mem::size_of_val(&array1);
}


fn slice(slice: &[i32]) {
    println!("Slice {:?}", slice)
}