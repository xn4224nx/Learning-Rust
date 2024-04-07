static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    return &noop_local as *const i32;
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let box_str = Box::new("b");
    let box_int = Box::new(456);
    let fn_int = noop();
    
    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_str: {:p}", Box::into_raw(box_str));
    println!("boxed_int: {:p}", Box::into_raw(box_int));
    println!("fn_int:    {:p}", fn_int);
}
