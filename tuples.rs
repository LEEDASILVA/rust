// several variables in on place
// A general way of grouping together a number of values into one compound type.
// - Tuples have a fixed length
// - Comma-separated list of values inside parenthesis
// - Each position in the tuple has a type (and they don't have to be the same)
// - To get the individual values of the tuple we can use pattern matching to destructure a tuple value like this:
fn main() {
    let tup1 = (20, "string", 25.0, false, 35); // tuple of several types

    println!("{}", tup1.2); // will print 25.0 because its on position 2

    let tup2 = (20, "string", 25.0, false, (1, 4, 6)); // nested tuples
    
    println!("{}", (tup2.4).2); // will print 6

    let tup3 = (45, 6.7, "pc");
    let (a, b, c) = tup3
    println!("{} {} {}", a, b, c); // will print 6
}