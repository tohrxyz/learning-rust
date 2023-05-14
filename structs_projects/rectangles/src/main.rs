#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 60,
    };
    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 25,
        height: 40,
    };

    println!(
        "rect1 can hold rect2 -> {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "rect1 can hold rect3 -> {}",
        rect1.can_hold(&rect3)
    );

}