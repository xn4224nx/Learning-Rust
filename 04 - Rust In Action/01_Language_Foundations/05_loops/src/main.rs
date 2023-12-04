fn main() {
    
    /* A Basic For loop */
    let my_vec0 = vec![1, 2, 3, 4, 5, 6, 7];
    for item in my_vec0 {
        println!("{}", item);
    }
    
    /* Non consuming for loop. */
    let my_vec1 = vec![1, 2, 3, 4, 5, 6, 7];
    for item in &my_vec1 {
        println!("{}", item);
    }    
    
    /* Using the loop again. */
    println!("my_vec1[0] = {}", my_vec1[0]);
    
    /* Mutate a container within loop. */
    let mut my_vec2 = vec![1, 2, 3, 4, 5, 6, 7];
    for item in &mut my_vec2 { *item = *item + 1; }
    println!("my_vec2 = {:?}", my_vec2);
    
    /* Anonymous For Loops */
    for _ in 0..=10 { println!("1"); }

    /* Using an index variable (slow) */
    for i in 0..my_vec2.len() {
        let item = my_vec2[i];
        println!("my_vec2[{}] = {}", i, item);
    }
}
