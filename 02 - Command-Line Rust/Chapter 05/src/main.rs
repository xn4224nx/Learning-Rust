/*
 * +++ Word Count +++
 *
 * Print the lines of text, words and bytes of a file, file or STDIN.
 *
 */

use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about,
    after_help = "The wc utility displays the number of lines, words, and bytes contained 
in each input file, or standard input (if no file is specified) to the
standard output. A line is defined as a string of characters delimited
by a <newline> character. Characters beyond the final <newline> charac-
ter will not be included in the line count."
)]
struct Args {
    /// The files to read analyse, will use STDIN if not specified.
    files: Option<Vec<String>>,

    #[clap(short = 'c', long = "bytes")]
    /// Count the number of bytes in each input file.
    bytes: bool,

    #[clap(short = 'l', long = "lines")]
    /// The number of lines in each input file.
    lines: bool,

    #[clap(short = 'm', long = "chars")]
    /// The number of chars in each input file.
    chars: bool,

    #[clap(short = 'w', long = "words")]
    /// The number of words in each input file.
    words: bool,

    #[clap(short = 'L', long = "max-line-length")]
    /// The length of the longest line in each input file.
    max_line_len: bool,
}

fn main() {
    let args = Args::parse();
}
