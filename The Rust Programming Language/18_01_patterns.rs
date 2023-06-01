fn main() {
    
    /* Basic Match Arms */
    
    let mut x: Option<i32> = None; //Some(4);
    
    x = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    
    println!("{:?}", x);
    
    
    /* Conditional if let Expressions */
    
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "-1".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background!");
        
    } else if is_tuesday {
        println!("Tuesday is a green day.");
        
    } else if let Ok(age) = age {
        
        if age > 30 {
            println!("Using purple as a background.");
        
        } else {
            println!("Using orange as the background.");
        }
        
    } else {
        println!("Using blue as the background.");
    }
    
    
    /* for loops */
    
    let v = vec!['a', 'b', 'c'];
    
    for (value, index) in v.iter().enumerate() {
    
        println!("{}:\"{}\"", index, value);
    }
    
    for value in &v {
    
        println!("{}", *value);
    }
    
    
    /* let statements */
    let (x, y, z) = (1, 2, 3);
    
    
    /* Function Parameters */
    
    fn print_coords(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    
    let point = (3, 5);
    print_coords(&point);
    
}
