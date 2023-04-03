// Define a trait
pub trait Summary {
    fn summarise_author(&self) -> String;
    
    fn summarise(&self) -> String {
        format!("(Read more {} ...)", self.summarise_author())
    }
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise())
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


impl Summary for NewsArticle {
    
    fn summarise_author(&self) -> String {
        format!("@{}", self.author)
    }
    
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    fn summarise_author(&self) -> String {
        format!("{}", self.username)
    }
}


pub struct Poem {
    pub title: String,
    pub author: String,
    pub text: String,
    pub style: String,
}


impl Summary for Poem {
    
    fn summarise_author(&self) -> String {
        format!("{}", self.author)
    }
}
