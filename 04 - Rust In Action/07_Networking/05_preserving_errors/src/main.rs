use std::fs::File;
use std::io::Error;
use std::net::{AddrParseError, Ipv6Addr};
use std::{error, fmt, io, net};

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/* Defer to the default method implementation. */
impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(error: io::Error) -> Self {
        return UpstreamError::IO(error);
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(error: net::AddrParseError) -> Self {
        return UpstreamError::Parsing(error);
    }
}

fn main() -> Result<(), UpstreamError> {
    /* Generate another type of error. */
    let _localhost = "::::1".parse::<Ipv6Addr>()?;

    /* Generate a error. */
    let _f = File::open("non-file.txt")?;

    return Ok(());
}
