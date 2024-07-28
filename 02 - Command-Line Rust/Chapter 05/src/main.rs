/*
 * +++ Word Count +++
 *
 * Print the lines of text, words and bytes of a file, file or STDIN.
 *
 */

use clap::Parser;
use std::{
    cmp::max,
    fs::File,
    io::{self, prelude::*, BufReader},
    ops::Add,
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

#[derive(Default)]
struct FileStats {
    wrd_total: usize,
    line_total: usize,
    char_total: usize,
    byte_total: usize,
    longest_line: usize,
}

impl Add for FileStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            wrd_total: self.wrd_total + other.wrd_total,
            line_total: self.line_total + other.line_total,
            char_total: self.char_total + other.char_total,
            byte_total: self.byte_total + other.byte_total,
            longest_line: max(self.longest_line, other.longest_line),
        }
    }
}

/// Count the words, chars and bytes in a string.
fn count_within_a_string(input: &String) -> (usize, usize, usize) {
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
fn output_stats(filename: String, args: &Args, stat: &FileStats) {
    let pad_size = 8;

    /* Print file statistics. */
    if args.lines {
        print!("{:>width$}", stat.line_total, width = pad_size);
    }
    if args.words {
        print!("{:>width$}", stat.wrd_total, width = pad_size);
    }
    if args.bytes {
        print!("{:>width$}", stat.byte_total, width = pad_size);
    }
    if args.chars {
        print!("{:>width$}", stat.char_total, width = pad_size);
    }
    if args.max_line_len {
        print!("{:>width$}", stat.longest_line, width = pad_size);
    }
    /* Print the filename. */
    if args.files.is_some() {
        println!(" {}", filename);
    }
}

/// Determine the key characteristics of a file buffer
fn analyse_buffer<R>(mut fp_buff: BufReader<R>) -> FileStats
where
    R: std::io::Read,
{
    let mut stat: FileStats = Default::default();
    let mut line_buf = String::new();

    /* Read the file line by line. */
    while fp_buff.read_line(&mut line_buf).unwrap() != 0 {
        /*  Examine the line. */
        let (wrd_cnt, cha_cnt, byt_cnt) = count_within_a_string(&line_buf);

        /* Update the totals. */
        stat.wrd_total += wrd_cnt;
        stat.byte_total += byt_cnt;
        stat.char_total += cha_cnt;
        stat.line_total += 1;

        /* Had the new longest line been found? */
        if cha_cnt > stat.longest_line {
            stat.longest_line = cha_cnt;
        };

        /* Empty the line buffer. */
        line_buf.clear();
    }
    return stat;
}

fn main() {
    let mut args = Args::parse();

    /* Refuse to further process when both chars and bytes have been selected. */
    if args.bytes && args.chars {
        panic!("the argument '--chars' cannot be used with '--bytes'")
    }

    /* If no options have been selected set the output to lines, words, bytes. */
    if !args.bytes && !args.lines && !args.chars && !args.words && !args.max_line_len {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    /* If no files have been provided read from STDIN. */
    if args.files.is_none() {
        /* Read STDIN and count the key parts */
        let f_stat = analyse_buffer(BufReader::new(io::stdin()));

        /* Report back about the STDIN */
        output_stats(String::from(""), &args, &f_stat);
        println!();

    /* Otherwise Iterate over every file and calculate its statistics. */
    } else {
        /* Make a record of the totals */
        let mut tot_stat: FileStats = Default::default();

        for raw_filepath in args.files.as_ref().unwrap() {
            /* Verify that the file exists. */
            if !raw_filepath.is_file() {
                let err_filename = raw_filepath.to_str().unwrap_or("Unparsable Filename");
                eprintln!("{}: File Access Error (os error 2)", err_filename);
                continue;
            };

            /* Try and open the file. */
            let Ok(fp) = File::open(&raw_filepath) else {
                let err_filename = raw_filepath.to_str().unwrap_or("Unparsable Filename");
                eprintln!("{}: File Read Error", err_filename);
                continue;
            };

            /* Analyse the file */
            let f_stat = analyse_buffer(BufReader::new(fp));

            /* Report back about the file */
            output_stats(raw_filepath.display().to_string(), &args, &f_stat);
            tot_stat = tot_stat + f_stat;
        }

        if args.files.as_ref().expect("ERROR no Files!").len() >= 2 {
            output_stats(String::from("total"), &args, &tot_stat);
        }
    }
}
