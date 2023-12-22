use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    /* Create a file object. */
    let f_obj = File::open("./data/spring_view_du_fu.txt").unwrap();
    let mut reader = BufReader::new(f_obj);

    /* String to store lines from the file. */
    let mut line = String::new();

    loop {
        /* Reading from the file can fail. */
        let len = reader.read_line(&mut line).unwrap();

        /* If the line is empty the file is at an end. */
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        /* Empty the string  */
        line.truncate(0)
    }
}
