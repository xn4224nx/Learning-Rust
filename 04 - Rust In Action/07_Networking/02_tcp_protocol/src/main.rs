use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    crude_get("www.bbc.co.uk").expect("Connection failed!");
}

fn crude_get<S: AsRef<str> + std::fmt::Display>(url: S) -> std::io::Result<()> {
    /* Construct the connection details. */
    let port = String::from("80");
    let conn_details_0 = format!("{url}:{port}");
    let conn_details_1 = format!("Host: {url}");

    /* Connect to the host. */
    let mut conn = TcpStream::connect(conn_details_0)?;
    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;
    conn.write_all(conn_details_1.as_bytes())?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    return Ok(());
}
