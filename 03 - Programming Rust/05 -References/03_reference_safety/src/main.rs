fn main() {
    
    let r;
    {
        let x = 1;
        r = &x;
    }
    
    //Major issue as `x` is not in scope and has been dropped.
    assert_eq!(*r, 1);
}
