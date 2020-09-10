// load dependancies
use std::fs::File;
use std::io::prelude::*;

fn main() {                 //   path
    let mut file = File::open("info.txt").expect("can not open the file!");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("opps can not read the file!");

    println!("File content: \n\n{}", content);
}