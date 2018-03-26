#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 20
    };
    println!("Given {:?}", rect);
    println!("Area is {:?}", rect.area());
}
