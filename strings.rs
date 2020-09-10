// to types of strings:
// primitive
// the struct, the data type string
fn main() {
    // data type string
    let mut string = String::from("hello world");

    // some methods
    println!("{}", string.len());
    println!("{}", string.is_empty());
    for token in string.split_whitespace() {
        println!("{}", token);
    }

    println!("contains world {}", string.contains("world"));

    string.push_str(", and welcome!");
    println!("{}", string);
}