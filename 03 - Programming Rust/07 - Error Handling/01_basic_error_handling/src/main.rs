fn div_res(numer: i32, denom: i32) -> Result<i32, &'static str> {
    
    /* Attempt the division. */
    return numer.checked_div(denom).ok_or_else(
        || {"division by zero"}
    );
}

fn main() {

    /* Basic fn usage */
    println!("{:?}", div_res(10, 5));
}
