fn factorial(n: usize) -> usize {

    return (1..=n).product();
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    
    /* Assigning References */
    let x = 10;
    let y = 20;
    let mut r = &x;    
    assert!(*r == 10);
    
    /* Assigning a reference to a variable points some where new. */
    r = &y;    
    assert!(*r == 20);
    
    /* References to references. */
    let pnt = Point {x: 1000, y: 729};
    
    /* Pointers to pointers */
    let r: &Point = &pnt;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    
    /* The `.` operator dereferences as much as it needs. */
    assert_eq!(rrr.y, 729);
    
    /* Comparing References. */
    let a = 10;
    let b = 10;
    
    let ra = &a;
    let rb = &b;
    
    let rra = &ra;
    let rrb = &rb;
    
    /* The references point to the same value. */
    assert!(rra == rrb);
    assert!(ra == rb);
    
    /* But they don't point to the same address. */
    assert!(!std::ptr::eq(rra, rrb));
    assert!(!std::ptr::eq(ra, rb));
    
    /* Borrowing references to Arbitary Expressions. */
    let fr = &factorial(6);
    
    /* Arithmetic operators can see through one level of references. */
    assert_eq!(fr + &1009, 1729);
}
