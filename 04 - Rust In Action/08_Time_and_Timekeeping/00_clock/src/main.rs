use chrono::{DateTime, Local};
use clap::{App, Arg};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        return Local::now();
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        );

    let args = app.get_matches();

    let action = args.values_of("action").unwrap();
    let std = args.value_of("std").unwrap();


    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
