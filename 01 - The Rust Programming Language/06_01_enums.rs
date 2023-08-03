// Define a enum
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    
    // The Address types each have a type
    // The types don't have to be the same
    V4(String),
    V6(String),   
}

enum Message {
    Quit,           // This has no data
    Move {x: i32, y:i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method here for the message enum
    }
}

fn main() {
    
    // Instancing enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Instancing the other type of enum
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Using methods of an enum
    let m = Message::Write(String::from("hello"));
    m.call();
    
    // Using the Option<T> 
    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;
}

