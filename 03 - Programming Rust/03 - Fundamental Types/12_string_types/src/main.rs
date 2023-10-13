use regex::Regex;

fn main() {
    
    /* String Literals */
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);
    
    /* Multi-line Strings */
    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less.\n");
    
    /* Raw Strings */
    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*").unwrap();
    
    println!("{}\n{}\n", pattern, default_win_install_path); 
    
    /* Multi-line Raw Strings */
    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        following immediately by three pound signs('###'):
    "###);
    
    /* Byte Strings */
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    
    /* Strings in Memory */
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "(づ ◕‿◕ )づ";
    
    println!("{}\n{}\n{}\n", noodles, oodles, poodles);
    
    /* String Length - in bytes not chars*/
    assert_eq!("ᕕ( ᐛ ) ᕗ".len(), 14);
}
