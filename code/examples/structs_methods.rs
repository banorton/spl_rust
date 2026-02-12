// Structs and methods

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }

    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::square(10);
    let mut rect3 = Rectangle::new(20, 40);

    println!("rect1: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    rect3.scale(2);
    println!("Scaled rect3: {:?}", rect3);
}
