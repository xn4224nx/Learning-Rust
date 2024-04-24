use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

/* The upstream error type is lost. */
fn main() -> Result<(), Box<dyn Error>> {
    /* Generate a error. */
    let _f = File::open("non-file.txt")?;

    /* Generate another type of error. */
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    return Ok(());
}
