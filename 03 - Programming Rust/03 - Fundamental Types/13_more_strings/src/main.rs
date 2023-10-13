fn main() {
    
    /* Convert &str to String */
    let error_msg = "too many pets".to_string();
    println!("{}\n", error_msg);
    
    /* Format macro */
    let direction = format!("{}°{:02}'{:02}''N", 24, 5, 23);
    println!("{}\n", direction);
    
    /* Combine vectors of strings */
    let bits = vec!["veni", "vidi", "vici"];
    println!("{}\n{}", bits.concat(), bits.join(", "));
    
    /* Contains Function */
    let snack_name = "peanut";
    let sub_str = "nut";
    
    println!("\"{}\" in \"{}\" : {}\n", 
                snack_name, sub_str, 
                snack_name.contains(sub_str));
    
    /* Replace Function */
    println!("{}\n", "(•‿•)".replace("•", "o"));
    
    /* Trim Function */
    println!("{}\n", "     clean \n\n\n\t    ".trim());
    
    /* Starts With Function */
    for word in "veni, vidi, vici".split(", ") {
        if word.starts_with("v") && word.ends_with("i") {
            println!("{}", word)
        }
    }
    
    
    
}
