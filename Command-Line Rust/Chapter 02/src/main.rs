use clap::{App, Arg};


fn main() {
    
    /* Process the Programs Arguments */
    let arg_matches = App::new("test_for_echo")
        .version("0.1.0")
        .author("Ben Mouncer")
        .about("Rust Echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();
     
     /* Create Variables based on the Arguments */
     let text = arg_matches.values_of_lossy("text").unwrap();
     let omit_newline = arg_matches.is_present("omit_newline");
     
     /* Create the String to Print */
     let output = text.join(" ");
     
     /* Print to STDOUT */
     if omit_newline {
        print!("{}", output)
     
     } else {
        println!("{}", output)
     }             
}

