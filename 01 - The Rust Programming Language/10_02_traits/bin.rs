use aggregator::{Summary, Tweet, Poem, notify};

fn main() {
	
	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from(
			"of course, as you probably already know, people"
		),
		reply: false,
		retweet: false,
	};
	
	println!("1 new tweet: {}", tweet.summarise());
	
	notify(&tweet);
	
	let poem = Poem {
		title: String::from("Let me not to the marriage of true minds"),
		author: String::from("William Shakespeare"),
		text: String::from("Let me not to the marr...."),
		style: String::from("Sonnet"),
	};
	
	println!("The poem is: {}", poem.summarise());
	
	
}
