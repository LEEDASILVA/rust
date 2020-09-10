fn main() {
    let arr = [1, 2, 3, 4, 5];

    let arr_type: [i32; 5] = [1, 2, 3, 4, 5]; // same thing as above

    // element; length
    let arr_default = [2; 400] // a array with default values in it 

    // arr[2] // 3
    // arr[1] // 2
    // looping and array
    for n in arr.iter() {
        println!("{}", n);
    }

    for i in 0..arr.len() {
        println!("index {}", i);
    }
}