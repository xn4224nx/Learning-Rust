/*
 * --- Text User Interface Wordle ---
 *
 * An implementation of the word guessing game Wordle. The game is
 * works through a text user interface using the ncurses package.
 */

use cursive::event::Key;
use cursive::views::{Dialog, TextView};
use cursive::{self};

fn main() {
    let mut siv = cursive::default();

    /* Add content to the app. */
    siv.add_layer(Dialog::around(TextView::new("Hello world!")).button("Ok", |x| x.quit()));

    /* Pressing the esc key exits the program. */
    siv.add_global_callback(Key::Esc, |x| x.quit());

    /* Start the event loop. */
    siv.run();
}
