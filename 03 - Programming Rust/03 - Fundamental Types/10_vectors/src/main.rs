fn main() {
    
    /* Vectors are resizable arrays allocated on the heap. */
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    
    /* Add elements dynamically. */
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
    
    /* Create a vector by repeating values. */
    let h_list = vec![0; 1000];
    assert_eq!(h_list.len(), 1000);
    assert_eq!(h_list[243], 0);
    
    /* Create a vector without the macro. */
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("new");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "new", "pets",]);
    
    /* Build a vector from a iterator. */
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
    
    /* Reverse a vector. */
    let mut palindrome = vec!["madam", "I", "m", "addam",];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["addam", "m", "I", "madam",]);
    
    /* Vectors with Capacity */
    let mut v_cap: Vec<i32> = Vec::with_capacity(2);
    assert_eq!(v_cap.len(), 0);
    assert_eq!(v_cap.capacity(), 2);
    
    v_cap.push(1);
    v_cap.push(2);
    assert_eq!(v_cap.len(), 2);
    assert_eq!(v_cap.capacity(), 2);
    
    v_cap.push(3);
    assert_eq!(v_cap.len(), 3);
    
    println!("Capacity is now {}", v_cap.capacity());    
}

