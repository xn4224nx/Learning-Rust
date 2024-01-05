#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: u64) -> StatusMessage {
    return StatusMessage::Ok;
}

fn main() {
    /* Satellites are represented by integers. */
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;
    
    /* Check the satellites status. */
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
    
    /* Check the satellites status again. */
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);    
}
