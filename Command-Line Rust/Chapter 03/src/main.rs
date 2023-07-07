use on_the_catwalk;

fn main() {
    
    if let Err(e) = on_the_catwalk::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
