/* static == global */
static HELLO_WORLD: &str = "Hello world!";
static mut COUNTER: u32 = 0;

fn main() {
    use std::slice;

    /* Dereference a Raw Pointer */
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    /* Call an unsafe function */
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    /* Creating a Safe Abstraction Over Unsafe Code */
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    println!("{:?}", a);
    println!("{:?}", b);

    /* Using `extern` Functions to Call External Code */
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }

    /* Accessing or Modifying a Mutable Static Variable */
    println!("Global variable = '{}'", HELLO_WORLD);

    unsafe {
        COUNTER += 1;
        println!("{}", COUNTER);
    }
}
