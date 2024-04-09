use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
	println!("Hello world!");
}"#;

fn main() -> std::io::Result<()> {
    /* Make space for the program's input. */
    let mut buffer: Vec<u8> = vec![];

    /* Read the input into the buffer. */
    INPUT.read_to_end(&mut buffer)?;

    for (idx, line) in buffer.chunks(BYTES_PER_LINE).enumerate() {
        print!("\n[0x{:08x}] ", idx);
        for byte in line {
            print!("{:02x} ", byte);
        }
    }

    println!("\n\n");
    return Ok(());
}
