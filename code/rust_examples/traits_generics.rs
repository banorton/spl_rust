// Traits and generics

use std::fmt::Display;

// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

// Generic function with trait bounds
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Multiple trait bounds
fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let article = Article {
        headline: String::from("Rust is awesome"),
        content: String::from("Learn Rust today!"),
    };
    print_summary(&article);

    let point_int = Point::new(5, 10);
    let point_float = Point::new(3.0, 4.0);
    println!("Distance: {}", point_float.distance_from_origin());
}
