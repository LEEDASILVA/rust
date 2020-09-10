// Write a program that prints a riddle, receives input from the user
// and checks that the answer is correct

// The program must allow indefinite number of trials and only quit after the
// correct answer is given

// Every time the user introduces an incorrect answer the program must
// print the riddle
// Before quitting the program must print the number of trials

// Riddle: I am the beginning of the end, and the end of time and
// space. I am essential to creation, and I surround every place. What
// am I?

// Answer: The letter e

use std::io;

fn main() {
	let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
	let answer = "e";

	println!("{}", riddle);
	println!("what is the answer??");

	// so match is like the C `switch`!
	/*
		match number {
		// Match a single value
		1 => println!("One!"),
		// Match several values
		2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
		// Match an inclusive range
		13..=19 => println!("A teen"),
		// Handle the rest of cases
		_ => println!("Ain't special"),
	}*/
	one_option(answer)
}

fn one_option(answer: &str) {
	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	if guess.trim() != answer {
		one_option(answer);
	}
	println!("correct!!!!!!!!!!!!!!!")
}
