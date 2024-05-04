fn noop() {}

fn main() {
    let fn_ptr = noop as usize;
    let types_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize:     0x{:x}", fn_ptr);
    println!("noop as *const T:  {:p}", types_fn_ptr);
}
