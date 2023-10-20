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
}
