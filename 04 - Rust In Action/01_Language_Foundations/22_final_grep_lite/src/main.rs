use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    
    /* Iterate over the file. */
    for raw_line in reader.lines() {
        let line = raw_line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}


fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .arg(Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false))
        .get_matches();
    
    /* Extract the regex pattern from the command line arguments. */
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    
    /* Extract the input file to search or we will use stdin. */
    let input = args.value_of("input").unwrap_or("-");
    
    /* Read lines via stdin. */
    if input == "-" {
        let raw_stdin = io::stdin();
        let reader = raw_stdin.lock();
        process_lines(reader, re);
    
    /* Or read the file give in the args. */
    } else {
        let file_obj = File::open(input).unwrap();
        let reader = BufReader::new(file_obj);
        process_lines(reader, re);
    }
}
