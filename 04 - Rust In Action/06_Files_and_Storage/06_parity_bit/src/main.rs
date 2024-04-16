fn parity_bit(bytes: &[u8]) -> u8 {
    
    let mut n_ones: u32 = 0;
    
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }
    
    return (n_ones % 2 == 0) as u8;
}

fn main() {
    let val = b"abc";
    
    println!("input: {:?}", val);
    println!("output: {:08x}\n", parity_bit(val));
}
