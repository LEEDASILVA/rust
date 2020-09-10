// 
struct Person {
    name: String,
    age: u8
}


// defining a trait, samething similar to Go interface{}
trait HasVoiceBox {
    // Speak
    // if you have a voice box you can speak
    fn speak(&self);
    // Check if can speak
    // if you have a voice box we can check if you can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("hello, my name is {}", self.name);
    }
    fn can_speak(&self) -> bool {
        self.age > 3
    }
}

fn main() {
    let person = Person { name: String::from("bob"), age: 10 };

    if person.can_speak() {
        person.speak();
    } else {
        println!("{} can speak stupid", person.name)
    }
}
