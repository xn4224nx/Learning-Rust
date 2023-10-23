struct Anime {

    name: &'static str,
    bechdel_pass: bool,
}

fn main() {
    
    /* x is a standard variable. */
    let x = 10;
    
    /* &x is immutable reference to x */
    let r = &x;
    
    /* *r de-references the r pointer. */
    assert!(*r == 10);
    
    /* Create a mutable value and a reference to it */
    let mut y = 32;
    let m = &mut y;
    
    /* Change the original value through the reference. */
    *m += 32;
    
    assert!(*m == 64);
    
    /* Create a structure instance and a ref to it. */
    let aria = Anime{name: "Aria - The Animation", bechdel_pass: true};
    let aria_ref = &aria;
    
    /* Derefencing happens automatically after a `.` */
    println!("Name: '{}'\nPass: {}\n", aria_ref.name, aria_ref.bechdel_pass);
    
    /* The `.` operator can borrow to the left if needed. */
    let mut v = vec![2, 3, 9, 1];
    v.sort();
    
    /* The last line is equivilent to the one below. */
    (&mut v).sort();
    println!("Sorted Vector = {:?}\n", v);
}
