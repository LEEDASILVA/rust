// match is something like a switch on other languages

fn main() {
    // chage the number
    let number = 2;

    match number {
        1 => println!("one"),
        2...20 => println!("range from 2 to 20"),
        10 | 11 => println!("ten or eleven!"),
        "Chris" => println!("nice name, mate!"),
        _ => println!("none of them")
    }
}
