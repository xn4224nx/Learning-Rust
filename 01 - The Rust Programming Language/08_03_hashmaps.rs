use std::collections::HashMap;

fn main() {
    
    // Setting up a hash map
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    
    // Default to zero if the key is not found
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    println!("key = {team_name}, value = {score}");
    
    
    // Iterate over a hash map in an arbitary order
    for (key, value) in &scores {
        
        println!("{key}: {value}");
    }
    
    
    // Hash Map Ownership
    let field_name = String::from("Colour");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // You can not use the two values that were inserted into the hash map
    // println!("{field_name}: {field_value}") 
    
    
    // Updating a Hash Map record
    map.insert(String::from("Colour"), String::from("Red"));
    println!("{:?}", map);
    
    // Only add if the key doesn't exist
    map.entry(String::from("Colour")).or_insert(String::from("Yellow"));
    map.entry(String::from("Number")).or_insert(String::from("Five"));
    
    println!("{:?}", map);
    
    
    // Update a value based on the old value
    let text = "Hello world wonderful world!";
    
    let mut word_map = HashMap::new();
    
    for word in text.split_whitespace() {
        
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", word_map);
}
