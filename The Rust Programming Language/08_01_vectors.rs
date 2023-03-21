fn main() {
    
    // Create a new vector that holds the i32 type
    let _my_vector: Vec<i32> = Vec::new();
    
    
    // Create another vector with predefined values
    let predef_vector = vec![1, 2, 3, 4, 5];
    
    
    // Create an epty vector and add to it
    let mut mutable_vector = Vec::new();
    
    mutable_vector.push(5);
    mutable_vector.push(6);
    mutable_vector.push(7);
    mutable_vector.push(8);
    mutable_vector.push(9);
    
    
    // Read elements of the vector
    
    // if the number is beyond the vecotr a panic happens
    let third: &i32 = &predef_vector[2];
    println!("The third element is {third}");
    println!("The first element is {}", &mutable_vector[0]); 
    
    // When accessing values beyond the vector it returns None
    let second: Option<&i32> = predef_vector.get(1);
    
    match second {
        Some(second) => println!("The second element is {second}."),
        None => println!("There is no second element."),
    }
    
    let first = &mutable_vector[0];
    println!("The first element is: {first}");
    mutable_vector.push(6);
    
    // This one won't work as the vector has changed
    // println!("The first element is: {first}");
    
    
    // Print all the elements in a vector
    for i in &predef_vector {
        println!("{i}");
    }
    
    // Modify the values in a vector
    for i in &mut mutable_vector {
        *i += 100;
    }
    
    for i in &mutable_vector {
        println!("{i}");
    }
    
    
    // Vectors in enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(11.3),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    
    
    // Dropping a vector
    {
        let _v = vec![1, 2, 3, 4, 5];
    }
    
    // Cant access _v here
}
