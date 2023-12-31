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

impl File {
    fn new(name: &str) -> File {
        return File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        };
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        
        /* Catch opening a file that isn't open. */
        if self.state != FileState::Open {
            return Err(String::from("File must be open to read!"));
        }
        
        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        
        save_to.reserve(read_len);
        save_to.append(&mut tmp);
         
        return Ok(read_len);
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    return Ok(f);
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    return Ok(f);
}

fn main() {
    let mut f6 = File::new("f6.txt");
    
    let mut buffer: Vec<u8> = vec![];
    
    if f6.read(&mut buffer).is_err() {
        println!("Error checking is working.");
    }
    
    f6 = open(f6).unwrap();
    let f6_len = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();
    
    let text = String::from_utf8_lossy(&buffer);
    
    println!("{:?}", f6);
    println!("{} is {} bytes long", &f6.name, f6_len);
    println!("{}", text);    
}
