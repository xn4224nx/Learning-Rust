/*
 * +++ Word Count +++
 *
 * Print the lines of text, words and bytes of a file, file or STDIN.
 *
 */

use clap::Parser;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};

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
    files: Option<Vec<PathBuf>>,

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

    return (wrd_cnt, char_cnt, input.len());
}

/// Output a files statistics
fn output_stats(
    filename: String,
    args: &Args,
    wrd_total: usize,
    line_total: usize,
    char_total: usize,
    byte_total: usize,
    longest_line: usize,
) {
    let pad_size = 8;

    /* Print file statistics. */
    if args.bytes {
        print!("{:>width$}", byte_total, width = pad_size);
    }

    if args.lines {
        print!("{:>width$}", line_total, width = pad_size);
    }

    if args.chars {
        print!("{:>width$}", char_total, width = pad_size);
    }

    if args.words {
        print!("{:>width$}", wrd_total, width = pad_size);
    }

    if args.max_line_len {
        print!("{:>width$}", longest_line, width = pad_size);
    }

    /* Print the filename. */
    if args.files.is_some() {
        println!(" {}", filename);
    }
}

fn main() {
    let mut args = Args::parse();

    /* If no options have been selected set the output to lines, words, bytes. */
    if !args.bytes && !args.lines && !args.chars && !args.words && !args.max_line_len {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    /* If no files have been provided read from STDIN. */
    if args.files.is_none() {
        let mut stdin_input = String::new();
        io::stdin()
            .read_line(&mut stdin_input)
            .expect("Error reading STDIN.");

        let (wrd_cnt, cha_cnt, byt_cnt) = count_within_a_string(stdin_input);
        output_stats(
            String::from(""),
            &args,
            wrd_cnt,
            1,
            cha_cnt,
            byt_cnt,
            cha_cnt,
        );

    /* Otherwise Iterate over every file and calculate its statistics. */
    } else {
        for raw_filepath in args.files.as_ref().unwrap() {
            /* Verify that the file exists. */
            if !raw_filepath.is_file() {
                let err_filename = raw_filepath.to_str().unwrap_or("Unparsable Filename");
                println!("File Access Error - '{}'", err_filename);
                continue;
            };

            /* Try and open the file. */
            let Ok(fp) = File::open(&raw_filepath) else {
                let err_filename = raw_filepath.to_str().unwrap_or("Unparsable Filename");
                println!("File Read Error - '{}'", err_filename);
                continue;
            };

            let mut wrd_total = 0;
            let mut lines = 0;
            let mut cha_total = 0;
            let mut byt_total = 0;
            let mut longest_line = 0;

            /* Read the file line by line. */
            let file = BufReader::new(fp);
            for raw_line in file.lines() {
                let Ok(line) = raw_line else {
                    continue;
                };

                /*  Examine the line. */
                let (wrd_cnt, cha_cnt, byt_cnt) = count_within_a_string(line);

                /* Update the totals. */
                wrd_total += wrd_cnt;
                byt_total += byt_cnt;
                cha_total += cha_cnt;
                lines += 1;

                /* Had the new longest line been found? */
                if cha_cnt > longest_line {
                    longest_line = cha_cnt;
                };
            }

            /* Report back about the file */
            output_stats(
                raw_filepath.display().to_string(),
                &args,
                wrd_total,
                lines,
                cha_total,
                byt_total,
                longest_line,
            );
        }
    }
}
