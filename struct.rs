// normal struct
struct Color {
    red: u8, // u6: 0-255
    green: u8,
    blue: u8
}

// tuple structs
struct TupColor(u8, u8, u8);

fn main() {
    // normal struct
    // color: red, green, blue
    let mut bg = Color { red: 255, green: 70, blue: 15 }; // type color, looks like js

    println!("{:?}", (bg.red, bg.green, bg.blue));

    // to change the values you must have mut !!
    bg.blue = 123;
    println!("{:?}", (bg.red, bg.green, bg.blue));

    // tuple struct
    let red = TupColor(255, 0, 0);
    println!("{}", (red.0, red.1, red.2));

    // change a value you must have mut !!
    red.2 = 60;

}


