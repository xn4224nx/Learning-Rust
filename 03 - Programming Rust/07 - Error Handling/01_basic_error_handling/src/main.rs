fn div_res(numer: i32, denom: i32) -> Result<i32, &'static str> {
    /* Attempt the division. */
    return numer.checked_div(denom).ok_or_else(|| "division by zero");
}

fn main() {
    let top: i32 = 10;
    let bot: i32 = 0;

    let calc = div_res(top, bot);

    /* is_ok() and is_err() will not consume calc but  unwrap() will. */
    if calc.is_ok() {
        println!("{} / {} = {}", top, bot, calc.unwrap());
    } else if calc.is_err() {
        println!("{} / {} causes error: '{}'", top, bot, calc.err().unwrap());
    }

    /* Set a value if there is an error. */
    println!("{} / {} = {} ", top, bot, calc.unwrap_or(0));

    /* Return function result if error. */
    println!(
        "{} / {} = {} ",
        top,
        bot,
        calc.unwrap_or_else(|_error| { 0 })
    );
}
