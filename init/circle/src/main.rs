// Create the structures `Circle` and `Point` and the methods
// necessary for this code to compile
// Methods:
//   - Point:
//     distance()
//   - Circle:
//     diameter -> returns the diameter of the circle
//     area -> returns the area of the circle
//     intersect -> returns true if the 2 circles intersect
// Associated Functions
//   - Circle:
//     new -> receives three 64 bits floating point numbers in the following
//     order: x, y and radius (x and y are the coordinates of the center).
//     and returns a new circle

#[derive(PartialEq, Debug)]
struct Point {
	x: f64,
	y: f64,
}

#[derive(PartialEq, Debug)]
struct Circle {
	center: Point,
	radius: f64,
}

impl Point {
	fn distance(&self, p: &Point) -> f64 {
		((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
	}
}

impl Circle {
	fn new(x: f64, y: f64, rad: f64) -> Circle {
		Circle {
			radius: rad,
			center: Point { x, y },
		}
	}
	fn area(&self) -> f64 {
		std::f64::consts::PI * self.radius.powi(2)
	}
	fn diameter(&self) -> f64 {
		self.radius * 2.0
	}
	fn intersect(&self, c: &Circle) -> bool {
		self.center.distance(&c.center) <= self.radius + c.radius
	}
}

fn main() {
	let circle = Circle::new(500.0, 500.0, 150.0);
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	let point_a = Point { x: 1.0, y: 1.0 };
	let point_b = Point { x: 0.0, y: 0.0 };
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);

	println!(
		"distance between {:?} and {:?} is {}",
		point_a,
		point_b,
		point_a.distance(&point_b)
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f64;

	fn approx_eq(a: f64, b: f64) -> bool {
		(a - b).abs() < f64::EPSILON
	}

	#[test]
	fn test_new_circle() {
		let circle = Circle::new(500.0, 400.0, 150.0);
		assert!(approx_eq(circle.radius, 150.0));
		assert!(approx_eq(circle.center.x, 500.0));
		assert!(approx_eq(circle.center.y, 400.0));
	}

	#[test]
	fn test_distance() {
		let a = Point { x: 0.0, y: 1.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), 1.0));
		let a = Point { x: 1.0, y: 0.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), 1.0));
		let a = Point { x: 1.0, y: 1.0 };
		let b = Point { x: 0.0, y: 0.0 };
		assert!(approx_eq(a.distance(&b), f64::sqrt(2.0)));
	}

	#[test]
	fn test_area() {
		let circle = Circle::new(500.0, 400.0, 150.0);
		assert!(approx_eq(circle.area(), 70685.83470577035));
	}

	#[test]
	fn test_intersection() {
		let circle = Circle::new(500.0, 500.0, 150.0);
		let circle1 = Circle::new(80.0, 115.0, 30.0);
		assert!(!circle.intersect(&circle1));
		let circle = Circle::new(100.0, 300.0, 150.0);
		let circle1 = Circle::new(80.0, 115.0, 100.0);
		assert!(circle.intersect(&circle1));
	}
}