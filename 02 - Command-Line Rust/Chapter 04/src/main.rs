/*
 * +++ Head +++
 *
 * Print the top lines or bytes of a file.
 *
 */

use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum ReadType {
    Bytes,
    Lines,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    /// File to read the top lines or bytes of.
    file: PathBuf,

    #[clap(short = 'n', long = "lines")]
    /// How many lines of the file would you like to read?
    nlines: Option<usize>,

    #[clap(short = 'c', long = "bytes")]
    /// How many bytes of the file would you like to read?
    nbytes: Option<usize>,
}

fn main() {
    let options = Options::parse();

    /* Confirm that both read options have not been selected. */
    if options.nbytes.is_some() && options.nlines.is_some() {
        panic!("The argument '--lines <LINES>' cannot be used with '--bytes <BYTES>'")
    }

    /* Verify the file exists. */
    if !options.file.is_file() {
        let err_file = options.file.to_str().unwrap_or("Unparsable File");
        panic!("File '{}' does not exist!", err_file);
    }

    /* The default if no options are provided. */
    let mut read_type = ReadType::Lines;
    let mut read_len = 10;

    /* Determine how much of the file needs to be read. */
    if options.nlines.is_some() {
        read_len = options.nlines.unwrap();
    } else if options.nbytes.is_some() {
        read_len = options.nbytes.unwrap();
        read_type = ReadType::Bytes;
    }

    if read_type == ReadType::Lines {
        /* Open the file */
        let fp = File::open(options.file).unwrap();
        let reader = BufReader::new(fp);

        /* Print the specified part of the file. */
        for (idx, raw_line) in reader.lines().enumerate() {
            if idx >= read_len {
                break;
            }

            let Ok(parsed_line) = raw_line else {
                continue;
            };

            println!("{}", parsed_line);
        }
    } else if read_type == ReadType::Bytes {
        let mut fp = File::open(options.file).unwrap();

        /* Read bytes from the start of the file. */
        fp.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = vec![0; read_len];
        fp.read_exact(&mut buffer).unwrap();

        println!("{:?}", buffer);
    }
}
