/*
 * +++ Robo Preacher +++
 *
 * To counter a crippling shortage in available vicars the Church of
 * England has instituted a project to provide parishioners with a
 * virtual vicar.
 *
 * This crude program will regurgitate whatever text you want with a
 * digital ecclesiastical veneer.
 */

const MAX_MSG_LINE_CHAR: usize = 50;
const NUM_MSG_LINES: usize = 5;

use clap::Parser;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "That would be an ecumenical matter!")]
    /// What is the liturgical message you wish to spread?
    message: String,

    #[clap(short = 'e', long = "eyes")]
    /// Give the preacher happy eyes.
    happy: bool,
}

fn main() {
    let options = Options::parse();
    let msg = options.message;

    /* Determine what the vicars eyes will be */
    let eye = if options.happy { '^' } else { 'o' };

    /* Split the message into MAX_MESSAGE_LINE sized chunks. */
    let mut msg_split: Vec<String> = vec![String::new(); NUM_MSG_LINES];

    for (idx, ms_char) in msg.chars().enumerate() {
        /* Don't print charcters beyond the limit of the program */
        if idx >= MAX_MSG_LINE_CHAR * NUM_MSG_LINES {
            break;
        }

        msg_split[idx / MAX_MSG_LINE_CHAR].push(ms_char);
    }

    println!(
        r#"
               __
         /\   |  \
        /  \  |  /
       |    | |        {}                                       
       \____/ |       /{}   
       ( {eye} {eye}) |      / {}
       /\__/\ |   __/  {}
      /\ qp /\|        {}
     /  |  |  |
    /|  |db| /3}}
    | \ |  | \|
    \  \|qp|/ |
     \__/  | ||
     |/||db|/ |
     |  |  | /|
     '''''''' |
    "#,
        msg_split[0], msg_split[1], msg_split[2], msg_split[3], msg_split[4]
    );
}
