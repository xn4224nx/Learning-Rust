/* The ref r lasts as long as the instance of the struct does. */
struct S<'a> {
    r: &'a i32
}

/* Struct that references another struct */
struct D<'a> {
    s: &'a S<'a>
}


fn main() {
    
    let x = 10;
    let s = S { r: &x };

    assert_eq!(*s.r, 10);
    
    let d = D { s: &s };
    
    assert_eq!(*d.s.r, 10);
}
