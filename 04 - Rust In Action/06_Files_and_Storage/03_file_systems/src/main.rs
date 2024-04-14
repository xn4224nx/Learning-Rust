use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    /* Read the 2nd argument as the name of the file to read. */
    let u_input = env::args().nth(1).expect("Usage: file_systems FILENAME");

    /* These are not ensured to be UTF-8 complient. */
    let fpath = PathBuf::from(u_input);

    println!("\nPath             = {}", fpath.display());
    println!("File Name        = {:?}", fpath.file_name().unwrap());
    println!("File Extension   = {:?}", fpath.extension().unwrap());
    println!("File Stem        = {:?}\n", fpath.file_stem().unwrap());

    println!("Path Has Root    = {}", fpath.has_root());
    println!("Path Is Absolute = {}", fpath.is_absolute());
    println!("Path Is Relative = {}", fpath.is_relative());
    println!("Parent           = {:?}\n", fpath.parent().unwrap());

    /* Try and open the file and handle the file not being there. */
    match File::open(fpath) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file due to {error}"),
    };
}
