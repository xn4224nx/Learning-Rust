fn main() {
    
    let r;
    {
        let x = 1;
        r = &x;
        
        /* Now it is within scope and works */
        assert_eq!(*r, 1);
    }
}
