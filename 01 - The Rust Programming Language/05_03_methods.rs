#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) ->  u32 {
        return self.width * self.height
    }
    
    fn width(&self) -> bool {
        return self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height
    }
    
    // Associated Function (Creates as new instance
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}   

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };
    
    let rect2 = Rectangle {
        width: 40,
        height: 5,
    };
    
    if rect0.width() {
        println!("The area of the rectangle is {} square pixels.",
            rect0.area()
        );
    }
    
    println!("Can rect0 hold rect1? {}", rect0.can_hold(&rect1));
    println!("Can rect0 hold rect2? {}", rect0.can_hold(&rect2));
    
    // Create a new instance that is a Square
    let square0 = Rectangle::square(3);
    
    println!("{:?}", square0);
}
