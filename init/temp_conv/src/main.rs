// Modify the body of the functions fahrenheit_to_celsius and
// celsius_to_fahrenheit to
// Make this code compile and pass the test
fn main() {
	println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
	println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
	// T(C) = (T(F) - 32) / 1.8
	((f - 32.0) / 1.8).round()
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
	// T(F) = T(C) * 9/5 + 32
	(c * 1.8 + 32.0).round()
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f64::EPSILON;

	fn eql(a: f64, b: f64) -> bool {
		(b - a).abs() < EPSILON
	}

	#[test]
	fn test_f_to_c() {
		assert!(eql(fahrenheit_to_celsius(20.0), -6.666666666666666));
		assert!(eql(fahrenheit_to_celsius(83.0), 28.333333333333332));
	}

	#[test]
	fn test_c_to_f() {
		assert!(eql(celsius_to_fahrenheit(27.0), 80.6));
		assert!(eql(celsius_to_fahrenheit(0.0), 32.0))
	}
}
