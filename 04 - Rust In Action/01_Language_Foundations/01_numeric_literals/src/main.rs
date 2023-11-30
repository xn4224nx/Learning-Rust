fn main() {
    
    /* Integer Declaration */
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22_i32;
    
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    
    let one_milli: i64 = 1_000_000;
    println!("({})^2 = {}", one_milli, one_milli.pow(2));
    
    /* Arrays must all have the same type. */
    let forty_twos = [
        42.0,
        42_f32,
        42.0_f32,
    ];
    
    println!("{:02}", forty_twos[2]);
}
