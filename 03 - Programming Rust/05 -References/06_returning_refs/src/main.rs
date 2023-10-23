fn smallest_in_slice(v: &[i32]) -> &i32 {
    
    let mut curr_smallest = &v[0];
    
    for r in &v[1..] {
        if *curr_smallest > *r {
            curr_smallest = r;
        }
    }
    
    return curr_smallest;
}

fn main() {
    
    let nums = vec![3, 2, 1, 6];
    let parabola = vec![9, 4, 1, 0, 1, 4, 9];   
    let parab_min: &i32;
    
    println!("{}", smallest_in_slice(&nums));
    
    {
        parab_min = smallest_in_slice(&parabola);
        assert_eq!(*parab_min, 0);
    }
    
    assert_eq!(*parab_min, 0);
}
