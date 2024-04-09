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
    
    let mut file_pos_idx = 0;
    
    for line in buffer.chunks(BYTES_PER_LINE) {
		print!("\n[0x{:08x}] ", file_pos_idx);
		for byte in line {
			print!("{:02x} ", byte);
		}
		
		/* Increase the index of the position within the file. */
		file_pos_idx += 1;
	}
	
	println!("\n\n");
	return Ok(());
}
