use chrono::Local;

fn main() {
    /* Time in the system's local time. */
    let now = Local::now();
    
    println!("{}", now);
}
