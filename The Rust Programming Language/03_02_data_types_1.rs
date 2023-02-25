fn main() {
    
    // defining a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // extract the elements of the tuple
    let (x, y, z) = tup;
    
    println!("x = {x}, y = {y}, z = {z}");
    
    let element_0 = tup.0;
    println!("{element_0}");
    
    // Define an array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];         // An array of size 5 with every element 3
    
    // Access elements of an array
    let element_1 = a[1];
    let element_2 = b[4];
    let element_3 = c[3];
    
    println!("{element_1} {element_2} {element_3}")
    
}
