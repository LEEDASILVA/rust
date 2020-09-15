// Why does this code not compile?

// a) `ref` cannot be used for compound data types
// b) Strings are always immutable
// c) There cannot be more than one immutable reference to the same variable
// d) A reference cannot be printed by `println!`

fn main() {
	let s = String::from("hello");
	let ref r2 = s;
	let r1 = &s;

	println!("{}, {}", r1, r2);
}
