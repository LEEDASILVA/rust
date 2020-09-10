// traits its an interface like java

struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn ro_string(&self) -> String{
        return format!("My name is {} and i am {}", self.name, self.age);
    }
}

fn main() {
    let mut someone = Person { name: String::from("Lee"), age: 21 };

    // to_string traid
    println!("{}", someone.to_string()); // My name is ____ and i am ____
}