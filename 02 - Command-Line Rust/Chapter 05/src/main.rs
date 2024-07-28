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
    if args.lines {
        print!("{:>width$}", line_total, width = pad_size);
    }

    if args.words {
        print!("{:>width$}", wrd_total, width = pad_size);
    }

    if args.bytes {
        print!("{:>width$}", byte_total, width = pad_size);
    }

    if args.chars {
        print!("{:>width$}", char_total, width = pad_size);
    }

    if args.max_line_len {
        print!("{:>width$}", longest_line, width = pad_size);
    }

    /* Print the filename. */
    if args.files.is_some() {
        println!(" {}", filename);
    }
}

/// Determine the key characteristics of a file buffer
fn analyse_buffer<R>(mut fp_buff: BufReader<R>) -> (usize, usize, usize, usize, usize)
where
    R: std::io::Read,
{
    let mut wrd_total = 0;
    let mut lines = 0;
    let mut cha_total = 0;
    let mut byt_total = 0;
    let mut longest_line = 0;
    let mut line_buf = String::new();

    /* Read the file line by line. */
    while fp_buff.read_line(&mut line_buf).unwrap() != 0 {
        /*  Examine the line. */
        let (wrd_cnt, cha_cnt, byt_cnt) = count_within_a_string(&line_buf);

        /* Update the totals. */
        wrd_total += wrd_cnt;
        byt_total += byt_cnt;
        cha_total += cha_cnt;
        lines += 1;

        /* Had the new longest line been found? */
        if cha_cnt > longest_line {
            longest_line = cha_cnt;
        };

        /* Empty the line buffer. */
        line_buf.clear();
    }

    return (wrd_total, lines, cha_total, byt_total, longest_line);
}

fn main() {
    let mut args = Args::parse();

    /* Refuse to further process when both chars and bytes have been selected. */
    if args.bytes && args.chars {
        panic!("the argument '--chars' cannot be used with '--bytes'")
    }

    /* If no options have been selected set the output to lines, words, bytes. */
    if [
        args.bytes,
        args.lines,
        args.chars,
        args.words,
        args.max_line_len,
    ]
    .iter()
    .all(|x| x == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    /* If no files have been provided read from STDIN. */
    if args.files.is_none() {
        /* Read STDIN and count the key parts */
        let (wrd_total, lines, cha_total, byt_total, longest_line) =
            analyse_buffer(BufReader::new(io::stdin()));

        /* Report back about the STDIN */
        output_stats(
            String::from(""),
            &args,
            wrd_total,
            lines,
            cha_total,
            byt_total,
            longest_line,
        );
        println!();

    /* Otherwise Iterate over every file and calculate its statistics. */
    } else {
        /* Make a record of the totals */
        let mut all_wrd_total = 0;
        let mut all_lines = 0;
        let mut all_cha_total = 0;
        let mut all_byt_total = 0;
        let mut all_longest_line = 0;

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
            let (wrd_total, lines, cha_total, byt_total, longest_line) =
                analyse_buffer(BufReader::new(fp));

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

            all_wrd_total += wrd_total;
            all_lines += lines;
            all_cha_total += cha_total;
            all_byt_total += byt_total;
            if longest_line > all_longest_line {
                all_longest_line = longest_line;
            };
        }

        if args.files.as_ref().expect("ERROR no Files!").len() >= 2 {
            output_stats(
                String::from("total"),
                &args,
                all_wrd_total,
                all_lines,
                all_cha_total,
                all_byt_total,
                all_longest_line,
            );
        }
    }
}
