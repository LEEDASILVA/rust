// impl -> implementation key word
// add methods to a struct!

struct Rectangle {
    width: u32,
    height: u32
}

// specify functions for the struct
impl Rectangle {
    // using the reference to self you can fetch the values in the structure 
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let mut my_rect = Rectangle { width: 10, height: 5 };

    // Rectangle: 10*5
    my_rect.print_description();

    println!("is square {}", my_rect.is_square)
}