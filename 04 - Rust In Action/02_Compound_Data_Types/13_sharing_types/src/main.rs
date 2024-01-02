/* Making an enum public, makes all elements public. */
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/* The fields in struct default to private. */
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

/* Each function defaults to private unless marked as public. */
impl File {
    pub fn new(name: &str) -> File {
        return File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        };
    }
}

fn main() {
    let f8 = File::new("f8.txt");
    println!("{:?}", f8);
}
