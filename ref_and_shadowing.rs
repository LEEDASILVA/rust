fn main() {
    let mut x = 10;
    {
        let x = 15; // -> shadowing, using the key word let
    }

    let x = "X is a string";

    println!("x: {}", x);
    
    let x = true;

    println!("x: {}", x);
}