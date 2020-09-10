// hash map -> collection of key value pairs
// map[int]string -> Go lang
use std::collections::HashMap;
fn main() {
    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust programming", 50);
    marks.insert("Go programming", 96);
    marks.insert("C programming", 90);
    marks.insert("Js programming", 98);
    marks.insert("Java programming", 70);
    marks.insert("Python programming", 10);


    // length of hash map
    println!("how many: {}", marks.len());

    // get a single value
    match marks.get("Js programming") {
        Some(mark) => println!("you got {} for js", mark),
        None => println!("you did not study this language")
    }

    // removing value
    marks.remove("Python programming");

    // looping in hash map
    for (key, value) in &marks {
        println!("for {} you got {}%!", key, value);
    }

    // check fro value
    println!("did you... {}", marks.contains_key("Rust programming"))
}
