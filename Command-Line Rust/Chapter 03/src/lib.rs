use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("Hello, world!");
    dbg!(config);
    return Ok(())
}

/// Extract and Process the Arguments 
pub fn get_args() -> MyResult<Config> {
    
    let arg_matches = App::new("on_the_catwalk")
        .version("0.1.0")
        .author("Ben Mouncer")
        .about("Rust Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Files to read and combined.")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number all output lines.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number non-empty lines.")
                .takes_value(false),
        )
        .get_matches();
    
    return Ok(Config {
        files: arg_matches.values_of_lossy("files").unwrap(),
        number_lines: arg_matches.is_present("number_lines"),
        number_nonblank_lines: arg_matches.is_present("omit_newline"),
    
    })
}
