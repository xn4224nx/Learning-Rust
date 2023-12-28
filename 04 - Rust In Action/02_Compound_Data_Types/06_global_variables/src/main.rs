use rand::random;

static mut ERROR: i32 = 0;

fn main() {
    let result: i32 = random::<bool>() as i32;
    println!("random result = {}", result);

    /* Change the value of a mutable global variable. */
    unsafe {
        ERROR = result;
    }

    /* Check the mutable global variable */
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured!");
        }
    }
}
