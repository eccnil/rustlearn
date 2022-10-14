#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }
    fn contains(&self, other:&Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Self {
        Self { width: (size), height: (size) }
    }
}
fn main() {
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle::square(31);
    println!(
        "he area of the rectangle {:#?} is {} square pixels",
        rect1,
        rect1.area()
    );
    dbg!(rect1.contains(&rect2));
    dbg!(rect1);
}
