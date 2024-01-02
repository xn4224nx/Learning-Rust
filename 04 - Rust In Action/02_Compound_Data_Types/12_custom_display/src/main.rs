#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        };
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "<{} ({})>", self.name, self.state);
    }
}

impl File {
    fn new(name: &str) -> File {
        return File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        };
    }
}

fn main() {
    let f7 = File::new("f7.txt");

    /* The default way without impl. "File { name: "f7.txt", data: [], state: Closed }" */
    println!("{:?}", f7);

    /* The impl way created in this script. "<f7.txt (CLOSED)>" */
    println!("{}", f7);
}
