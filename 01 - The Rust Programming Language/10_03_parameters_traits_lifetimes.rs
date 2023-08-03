use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    
    if x.len() > y.len() {
        
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    
    fn level (&self) ->i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str

where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {

    let x = 5;
    let r = &x;

    println!("r: {}", r);
    
    let string1 = String::from("abcd");
    
    {
        let string2 = "xyz";
    
        let result = longest(string1.as_str(), string2);
        println!("The longest string is \"{}\"", result);
    }
    
    let novel = String::from("Call meIshmael. Some years ago...");
    let first_sentence = novel.split('.')
                                .next()
                                .expect("Could not find a '.'");
                                
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    let s: &'static str = "I have a static lifetime";
    println!("{}", s);
    
    let string3 = String::from("a very long string");
    let string4 = String::from("a short string");
    
    let result2 = longest_with_an_announcement(&string3, &string4, "hey you");
}
