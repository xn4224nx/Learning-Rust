/*
 * UNIQ -- Report or filter out repeated lines
 */

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[clap(short = 'c', long = "count")]
    /// prefix lines by the number of occurrences
    count: bool,

    #[clap(short = 'd', long = "repeated")]
    /// only print duplicated lines, one for each group
    rep: bool,

    #[clap(short = 'D', long = "all-repeated")]
    /// print all duplicate lines
    all_rep: bool,

    #[clap(short = 'f', long = "skip-fields")]
    /// avoid comparing the first N fields
    skip_f: Option<usize>,

    #[clap(short = 'i', long = "ignore-case")]
    /// ignore differences in case when comparing
    ignore: bool,

    #[clap(short = 's', long = "skip-chars")]
    /// avoid comparing the first N characters
    skip_c: Option<usize>,

    #[clap(short = 'u', long = "unique")]
    /// only print unique lines
    unique: bool,

    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
}
