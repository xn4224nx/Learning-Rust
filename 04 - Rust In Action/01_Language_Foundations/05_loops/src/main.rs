use rand::random;

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
    
    /* While Loop */
    let mut samples = vec![];
    
    while samples.len() < 10 {
        let sample = random::<u8>();
        
        if sample >= 100 {
            continue;
        }
        
        samples.push(sample);
    }
    println!("Samples = {:?}", samples);
    
    /* Endless Loops */
    loop {
    
        let rnd_num = random::<u8>();
        println!("rnd_num = {}", rnd_num);
        
        if rnd_num > 100 {
            break;
        }
    }
    
    /* Escaping from nested loops. */
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                println!("[{}, {}, {}]", x, y, z);
                
                if x + y + z > 100 {
                    break 'outer;
                }
            }
        }
    }
}
