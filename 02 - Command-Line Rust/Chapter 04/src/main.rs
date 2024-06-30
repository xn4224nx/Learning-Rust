/*
 * +++ Head +++
 *
 * Print the top lines or bytes of a file.
 *
 */

use clap::{Command, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    /// File to read the top lines or bytes of.
    file: PathBuf,

    #[clap(short = 'c', long = "bytes")]
    /// How many bytes of the file would you like to read?
    nbytes: Option<usize>,

    #[clap(short = 'n', long = "lines")]
    /// How many lines of the file would you like to read?
    nlines: Option<usize>,
}

fn main() {
    let options = Options::parse();
}
