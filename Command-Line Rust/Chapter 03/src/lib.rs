use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

/// Main Program Function 
pub fn run(config: Config) -> MyResult<()> {
    
    /* Iterate over the inputed filenames. */
    for filename in config.files {
        
        /* Check that the file can be opened. */
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                
                let mut last_num = 0;
                
                /* Print the contents of the file */
                for (line_num, line) in file.lines().enumerate() {
                    
                    /* Extract the line. */
                    let line = line?;
                    
                    /* Print the line */
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    
                    } else if config.nonblank_lines {
                        if !line.empty() {
                        
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        
                        } else {
                            println!();
                        }

                    } else {
                        println!("{}", line);
                    }
                }
            },
        }
    }
    
    return Ok(())
}

/// Open a file as text or read STDIN
pub fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {

    return match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
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
                .multiple(true)
                .default_value("-")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number all output lines.")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number non-empty lines.")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();
    
    return Ok(Config {
        files: arg_matches.values_of_lossy("files").unwrap(),
        number_lines: arg_matches.is_present("number_lines"),
        number_nonblank_lines: arg_matches.is_present("omit_newline"),
    
    })
}
