/*
 * --- Text User Interface Wordle ---
 * 
 * An implementation of the word guessing game Wordle. The game is
 * works through a text user interface using the ncurses package. 
 */ 

use cursive::{self, views::TextView};

fn main() {
    
    let mut siv = cursive::default();
    
    /* Add content to the app. */
    siv.add_layer(TextView::new("Hello world!"));
    
    /* Start the event loop. */
    siv.run();
}
