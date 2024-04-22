use std::net::{ToSocketAddrs, SocketAddr};
use std::time::Duration;

use clap::{App, Arg};
use rand;

/* https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html */

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 443));
    let mut addrs_iter = addr.to_socket_addrs().unwrap();

    assert_eq!(Some(addr), addrs_iter.next());
    assert!(addrs_iter.next().is_none());
}
