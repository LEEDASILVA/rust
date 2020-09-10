use std::io;

fn main() {
    let mut input = String::new();
    
    println!("say something!");

    match io::stdin().read_line(&mut input) {
        // Ok -> no problem
        // b is the number of bytes, but you can put a _ to ignore it
        Ok(_) => {
            println!("finally you said, {}", input.trim().to_uppercase())
        },
        Err(e) => println!("no no no no {}", e)
    }
}
