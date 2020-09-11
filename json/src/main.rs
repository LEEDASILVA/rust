extern crate serde_json;
extern crate serde;
// this attribute allows 
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

// trades
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male:bool
}

// parse and use json data, maping the data to a struct
// decoding/ encoding / structures/ ....
fn main() {
    // raw string
    let json_str = r#"
        {
            "name": "someone",
            "age": 24,
            "is_male": true
        }
    "#;

    // returns a type result, so if the json is invalid it will give error
    // so we have to use unwrap
    let result = serde_json::from_str(json_str);

    if result.is_ok() {
        // map the values and put on the structure
        let p: Person = result.unwrap();
        // p["name"] it will give you the json string so -> "someone"
        // if you want the rust string you must use as_str().unwrap() but
        // it will return an option so use match
        // or p["name"] and or let p: JsonValue = result.unwrap():
        println!("The name is {}", p.name);
        println!("The name is {}", p.age);
        println!("The name is {}", p.is_male);
    } else {
        println!("could not parse json")
    }
}

