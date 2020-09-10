// normal struct
struct Color {
    red: u8, // u6: 0-255
    green: u8,
    blue: u8
}

fn main() {
    // normal struct
    // color: red, green, blue
    let mut bg = Color { red: 255, green: 70, blue: 15 }; // type color, looks like js
    print_color(&bg)
}

// if you dont pass the reference! and use the normal value, you will not be able to
// use the function again, because it passes the bg value and its no longer on the scope
// the parameter c is a reference to Color
fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue)
}

