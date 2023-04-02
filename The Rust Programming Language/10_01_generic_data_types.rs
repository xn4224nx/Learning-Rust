// Generic Max function
fn list_max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest
}


// Generic Structures
struct Point <T> {
    
    x: T,
    y: T,
}

struct MixedPoint <T, U> {
    
    x: T,
    y: U,
}

struct GPoint <X1, Y1> {
    x: X1,
    y: Y1,
}

// Generic Methods
impl<T> Point<T> {
    
    // Generic Getter Methods
    fn x(&self) -> &T {
        return &self.x
    }
    
    fn y(&self) -> &T {
        return &self.y
    }
}

// Type Limited Methods 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Mixing up Types
impl <X1, Y1> GPoint <X1, Y1> {
    
    fn mixup <X2, Y2> (self, other: GPoint <X2, Y2>) -> GPoint <X1, Y2> {
        GPoint {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    
    // Using Generic Functions
    
    let number_list = vec![0, 2, 3, 44, 2, 56];
    let char_list = vec!['w', 'g', 'y', 'a', '0'];
    
    println!("The largest number is {}", list_max(&number_list));
    println!("The largest char is {}", list_max(&char_list));
    
    // Using a Generic Structure
    
    let int_point = Point{x: 5, y: 10};
    let float_point = Point{x: 0.1, y: -0.9};
    
    println!("Point 1 -> [{} : {}]", int_point.x(), int_point.y());
    println!("Point 2 -> [{} : {}]", float_point.x(), float_point.y());
    
    let multi_point = MixedPoint{x: 10, y: -9.9};
    
    println!("Point 3 -> [{} : {}]", multi_point.x, multi_point.y);
    println!("Distance from origin: {}", float_point.distance_from_origin());
    
    
    let p1 = GPoint {x:4, y: 10.4};
    let p2 = GPoint {x:"Hello", y: 'c'};
    
    let p3 = p1.mixup(p2);
    
    println!("New Point  = {} : {}", p3.x, p3.y);
}
