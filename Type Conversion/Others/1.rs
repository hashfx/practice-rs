/*

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.__, "The point is (0, 0)");
    assert_eq!(format!(__), "The point is (0, 0)");

    println!("Success!")
}

*/

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}
