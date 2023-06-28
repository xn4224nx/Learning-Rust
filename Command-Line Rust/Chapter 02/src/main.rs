extern crate getopts;
use getopts::Options;
use std::env;


fn main() {
    
    //Using https://docs.rs/getopts/latest/getopts/
    
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
