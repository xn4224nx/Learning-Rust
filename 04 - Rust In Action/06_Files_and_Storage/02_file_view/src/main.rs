use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() {
    /* Read the 2nd argument as the name of the file to read. */
    let fname = env::args().nth(1).expect("Usage: file_view FILENAME");

    /* Try and open the file. */
    let mut file_ptr = File::open(&fname).expect("Unable to open file.");

    let mut f_pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    /* Loop until read_exact returns Err, ie when it runs out of data. */
    while let Ok(_) = file_ptr.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", f_pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!("");
        f_pos += BYTES_PER_LINE
    }
}
