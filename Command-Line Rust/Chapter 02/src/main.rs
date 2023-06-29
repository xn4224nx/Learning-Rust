use clap::{App, Arg};


fn main() {
    
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
     
     println!("{:#?}", arg_matches);                   
}
