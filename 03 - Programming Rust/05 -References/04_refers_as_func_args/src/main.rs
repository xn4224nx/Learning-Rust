/* Set a mutable global variable, global variables last the entire lifetime of 
the script. It must be intialised. */
static mut STASH: &i32 = &0;

fn set_stash_var(p: &'static i32) {   
    unsafe {STASH = p;}
}

fn print_stash_var() {
    unsafe {println!("Current stash = {}", &STASH);}
}

fn main() {
    print_stash_var();
    set_stash_var(&128);
    print_stash_var();
}
