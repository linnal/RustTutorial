#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn greater_than(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }
}

fn main() {
    let rect = Rectangle{ width: 30, height: 20};
    let rect2 = Rectangle{ width: 50, height: 20};
    
    println!("Given {:?}", rect);
    println!("Area is {:?}", rect.area());
    println!("rect is greater than rect2? {:?}", rect.greater_than(&rect2));
}
