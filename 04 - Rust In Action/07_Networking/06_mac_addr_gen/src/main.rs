use rand::RngCore;
use std::fmt::{self, Display};

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;

        /* Convert each of the bytes to hexadecimal. */
        return write!(
            f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        );
    }
}

impl MacAddress {
    fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);

        /* Set the MAC address to local and unicast. */
        octets[0] |= 0b_0000_0011;

        return MacAddress { 0: octets };
    }

    fn is_local(&self) -> bool {
        /* Check the second to last bit in on. */
        return (self.0[0] & 0b_0000_0010) == 0b_0000_0010;
    }

    fn is_unicast(&self) -> bool {
        /* Check the last bit in on. */
        return (self.0[0] & 0b_0000_0001) == 0b_0000_0001;
    }
}

fn main() {
    let rnd_mac = MacAddress::new();

    println!(
        "MAC = {}, Local = {}, Unicast = {}",
        rnd_mac,
        rnd_mac.is_local(),
        rnd_mac.is_unicast()
    )
}
