use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = String::from("www.bbc.co.uk");
    let port = String::from("80");
    let conn_details_0 = format!("{host}:{port}");
    let conn_details_1 = format!("Host: {host}");

    let mut conn = TcpStream::connect(conn_details_0)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(conn_details_1.as_bytes())?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    return Ok(());
}
