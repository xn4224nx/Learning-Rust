use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    /* Create a file object. */
    let f_obj = File::open("./data/spring_view_du_fu.txt").unwrap();
    let reader = BufReader::new(f_obj);

    for raw_line in reader.lines() {
        /* Reading from the file can fail. */
        let line = raw_line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
