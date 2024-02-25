fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    /* Interprets the *const i64 as usize */
    let a_addr: usize = unsafe {
        /* Accessing the value of a raw pointer is dangerous! */
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);

    /* You can create pointers safely from integers */
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
