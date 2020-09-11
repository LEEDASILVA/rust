extern crate regex;
use regex::Regex;

fn main() {
    // r"" -> raw string perfect for regexs
    // unwrap -> unwrap the result so we get the Regex struct
    // it can return an error if the regex is unvalid
    // ()-> captures
    let result = Regex::new(r"(\w{9})").unwrap();
    let text = "something"

    println!("found? {}", re.is_match(text));

    // return an option
    // found the match, get the first then this can fail so we
    // call the unwrap and then call as_str to get the match as a string
    match re.captures(text) {
        // we can use this more simpler => println!("found: {}", &caps[0]),
        Some(caps) => println!("found: {}", caps.get(0).unwrap().as_str()),
        None => println!("no match")
    }
}
