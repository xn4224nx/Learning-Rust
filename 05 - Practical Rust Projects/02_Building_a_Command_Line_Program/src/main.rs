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

use std::env::args;

fn main() {
    let message = args()
        .nth(1)
        .unwrap_or(String::from("That would be an ecumenical matter!"));

    println!(
        r#"
               _
         /\   | |
        /  \  | /
       |    | |      {}
       \____/ |      |    
       (o o ) |   __/   
       /\__/\ |     
      /\ qp /\|     
     /  |  |  |
    /|  |db| /3}}
    | \ |  | \|
    \  \|qp|/ |
     \__/  | ||
     |/||db|/ |
     |  |  | /|
     ''''''''
    "#,
        message
    );
}
