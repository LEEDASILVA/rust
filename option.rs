// option represent a value or no value at all
// so rust forces us to take care of the options
// 
fn main() {
    // if we try to get the character from index 8
    // so it will return an option an
    // not a value by its self
    let name = String::from("Maria");

    // this ` name.chars().nth(8)` will return an option
    // will be `some` or `none`
    println!("char at 8 : {}", match name.chars().nth(8) {
        Some(c) => c.to_string(),
        None => "no chart ant index 8".to_string()
    });

    // defining a function that returns an `option`
    println!("something : {}", match get_occupation("something") {
        Some(o) => o,
        None => "error"
    })

}

// this function will return an option
// an if the return is good, so if the some exist it will be a string
fn get_occupation(name: &str) -> Option<&str> {
    // so if the name is something or michael
    // it wil return a some, if not "_" it returns a None 
    match name {
        "something" => Some("it is something"),
        "michael" => Some("hahaha"),
        _ => None
    }
}