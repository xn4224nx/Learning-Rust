use std::ops::Add;
use std::fmt;


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        
        if  self.count < 50 {
            self.count += 1;
            Some(self.count)
            
        } else {
            None
        }
    }
}


//#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other:Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    
    }

}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*Waves arms furiously*");
    }
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    
    // New Counter
    let mut my_counter = Counter::new();
    
    for _x in 0..10 {
        my_counter.next();
    }
    
    println!("Total counts = {}", my_counter.count);
    
    println!("{}", Point{x:1, y:0} + Point{x:2, y:3});
    
    let length = Millimeters(10)+ Meters(1);
    println!("{}", length.0);
    
    let person = Human;
    person.fly();
    
    Pilot::fly(&person);
    Wizard::fly(&person);
    
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    let test_point = Point{x:0, y:0};
    
    println!("{}", test_point);
    
    test_point.outline_print();

}
