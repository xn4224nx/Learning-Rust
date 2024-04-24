use hickory_resolver::config::*;
use hickory_resolver::Resolver;
use std::env;

fn main() {
    /* Read the 2nd argument as the name of the file to read. */
    let url = env::args().nth(1).expect("Usage: dns_resolver URL");

    // Construct a new Resolver with default configuration options
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(url).unwrap();

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let address = response.iter().next().expect("no addresses returned!");

    print!("{} ", address);

    if address.is_ipv4() {
        println!("IPv4")
    } else if address.is_ipv6() {
        println!("IPv4")
    } else {
        println!("IPv Unknown")
    }
}
