fn main() {
    
    // Standard If Else Chain
    
    let number = 9;
    
    if number % 4 == 0 {   
        println!("The number is divisible by 4");   
        
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");  

    } else if number % 2 == 0 {
        println!("The number is divisible by 2");  
    
    } else {
        println!("The number is not divisible 4, 3, or 2.")
    }
    
    
    // Single Line Conditional
    let condtion = true;
    
    // Both options have to have the same type
    let number1 = if condtion {5} else {6};
    
    println!("A different number is {number1}");

    
    // Infinite Loop
    let mut loop_idx = 0;
    
    loop {
        if loop_idx > 4 {
            break;
        }
        
        println!("10 GOTO 20");
        loop_idx += 1;
    };
    
    
    // Return a value from a loop
    let mut sum = 0;
    loop_idx = 0;
    
    let loop_result = loop {
        sum += loop_idx;
        
        if loop_idx >= 100 {
            break sum * 1;
        }
        
        loop_idx += 1;
    };

    println!("The sum is {loop_result}");
    
    
    // Loop Labels
    let mut count = 0;
    
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("Remaining = {remaining}");
            
            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up;
            }
            
            remaining -=1;
        }
        count += 1;      
    }
    println!("End count = {count}");
    
    
    // While Loops
    let mut count_down = 10;
    
    while count_down > 0 {
        println!("{count_down}!");
        count_down -= 1;
    }
    println!("LIFTOFF!!!!!");
    
    
    // Looping over an array
    let tens = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    loop_idx = 0;
    
    while loop_idx < tens.len() {
        println!("The value is: {}", tens[loop_idx]);
        loop_idx += 1;
    }
    
    for element in tens {
        println!("The value is: {element}");
    }
    
    
    // For Loops
    for count_number in (1..11).rev() {
        println!("{count_number}!");
    }
    println!("LIFTOFF!!!!");

}

