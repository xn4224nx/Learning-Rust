use regex::Regex;

fn main() {
    
    let search_term = "picture";
    let re_search = Regex::new("picture").unwrap();
    
    let quote = "
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        
        /* Search using the contains function. */
        if line.contains(search_term) {
            println!("{}: {}", i, line);
        }
        
        /* Search using regex. */
        let contains_substr = re_search.find(line);
        match contains_substr {
            Some(extr) => println!("{}: {} {}", i, line, extr.as_str()),
            None => (),
        }
    }
}
