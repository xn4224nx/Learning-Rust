/*
 * +++ Word Count +++
 *
 * Print the lines of text, words and bytes of a file, file or STDIN.
 *
 */

use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
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

/// Count the words, bytes and chars in a string.
fn count_within_a_string(input: String) -> (usize, usize, usize) {
    let mut wrd_cnt = 0;
    let mut char_cnt = 0;
    let mut in_whitespace = true;

    /* Iterate over the chars in the string. */
    for str_char in input.chars() {
        /* Check for white space, indicating the end of a word. */
        if !in_whitespace && str_char.is_whitespace() {
            wrd_cnt += 1;
            in_whitespace = true;

        /* Wait until non-white space (ie the next word) resumes. */
        } else if !str_char.is_whitespace() {
            in_whitespace = false;
        }

        char_cnt += 1;
    }

    return (wrd_cnt, input.len(), char_cnt);
}

fn main() {
    let args = Args::parse();

    /* If no files have been provided read from STDIN. */
    let mut stdin_input = String::new();
    io::stdin()
        .read_line(&mut stdin_input)
        .expect("Error reading STDIN.");

    println!("{:?}", count_within_a_string(stdin_input));
}
