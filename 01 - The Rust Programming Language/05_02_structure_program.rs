
fn main() {
    
    let scale = 2;
    
    let rect0 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    // Prints the value of  expression
    dbg!(&rect0);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect0)
    );
    
    println!("rect0 is {:?}", rect0);
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height
}

// The below line allows the structure to be printed
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
