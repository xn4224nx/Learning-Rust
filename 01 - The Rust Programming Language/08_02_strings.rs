fn main() {
    
    // Standard string definition
    let mut s0 = String::new();
    let s1 = String::from("zeroth string");
    
    // String from data
    let data = "intitial string";
    let s2 = data.to_string();
    
    let s3 = "another string".to_string();
    
    println!("s0 = '{s0}'\ns1 = '{s1}'\ns2 = '{s2}'\ns3 = '{s3}'");
    
    
    // Append to a string
    s0.push_str("new part of the string");
    println!("s0 = '{s0}'");
    
    s0.push(' ');
    
    s0.push_str(&s3);
    println!("s0 = '{s0}'");
    
    
    // Combining multiple strings
    let game_name_0 = String::from("tic");
    let game_name_1 = String::from("tac");
    let game_name_2 = String::from("toe");
    
    let game_name = format!("{game_name_0}-{game_name_1}-{game_name_2}");
    
    println!("The game is called '{}'", game_name);
    
    
    // Slicing a string
    let hello = String::from("Здравствуйте");
    let slice = &hello[0..4];
    
    println!("First two chars: {slice}");
    
    
    // Iterating over strings
    for c in hello.chars() {
        
        println!("{c}");
    }
    
    for b in hello.bytes() {
        println!("{b}");
    }
}
