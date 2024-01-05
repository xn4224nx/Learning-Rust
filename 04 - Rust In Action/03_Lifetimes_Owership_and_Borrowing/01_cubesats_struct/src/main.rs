#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);

    /* Return ownership back to the original scope. */
    return sat_id;
}

fn main() {
    /* Initalise the satellites. */
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    /* Check the status of the satellites. */
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    /* Check the status of the satellites AGAIN. */
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
