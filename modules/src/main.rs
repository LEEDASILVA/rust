// using modules
mod display {
    
    // this function is by default private
    // in its own module 
    pub fn print_ms() {
        something();
        println!("hello mate!");
    }
    // you can use a private function on
    // public ones so inside the module scope
    fn something() {
        println!("something");
    }

    // nest modules
    // must give the public access for the main module
    pub mod water {
        pub fn print_ms() {
            println!("drink water!!");
        }
    }
}

fn main() {
    display::print_ms();
    display::water::print_ms();
}
