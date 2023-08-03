fn main() {
    
    another_function();
    distance_print(5, 'm');

    
    // An expression {}, outputting to a statement
    let z = {
    
        // A statement
        let x = 3;
        
        // An expression, add a ; to turn it into a statement
        x + 1
    };
    
    distance_print(z, 's');
    
    // Basic returning function
    let y = return_five();
    println!("The number returned is {y}");
    
    // Another basic returning function
    let a = increment(10);
    distance_print(a, 'v');
}

fn another_function() {
    println!("Another function.");    
}

fn distance_print(x: i32, unit_label: char) {
    println!("The measurment is : {x}{unit_label}");    
}

fn return_five() -> i32 {
    5
}

fn increment(x: i32) -> i32 {
    x + 1
}
