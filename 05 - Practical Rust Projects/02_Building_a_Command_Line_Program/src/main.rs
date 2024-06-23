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

use std::env::args;

fn main() {
    let message = args()
        .nth(1)
        .unwrap_or(String::from("That would be an ecumenical matter!"));

    let mut msg_split: Vec<String> = vec![String::new(); NUM_MSG_LINES];

    /* Split the message into MAX_MESSAGE_LINE sized chunks. */
    for (idx, ms_char) in message.chars().enumerate() {
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
       ( o o) |      / {}
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
