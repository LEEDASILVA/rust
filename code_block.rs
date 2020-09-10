// code block -> scoops...
fn main() {
    // here
    let x = 10;
    {
        let y = 5;
        println!("x: {}, y: {}", x, y);
        // isolated
    }
    // this will give error because y is not in the scoop
    // println!("x: {}, y: {}", x, y); 
}