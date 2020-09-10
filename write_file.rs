use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("count not create file!!");

    file.write_all(b"welcome and come")
        .expect("cannot write to the file!!");
}
