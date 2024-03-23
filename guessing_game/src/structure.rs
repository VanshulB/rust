use std::fmt::Display;

#[allow(dead_code)]

pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle { length, width }
    }

    fn calculate_area(&self) -> u32 {
        self.width * self.length
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length: {}\nWidth: {}", self.length, self.width)
    }
}

pub fn run() {
    let abc = Rectangle::new(10, 20);
    println!("{}", abc);
    println!("The area of the rectangle is {}", abc.calculate_area())
}
