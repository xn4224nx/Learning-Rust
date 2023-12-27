#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        return File {
            name: String::from(name),
            data: Vec::new(),
        };
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        return f;
    }
    
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        
        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        
        return read_len; 
    }
}

fn open(f: &mut File) -> bool {
    return true;
}

fn close(f: &mut File) -> bool {
    return true;
}

fn main() {
    let f3_data : Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("2.txt", &f3_data);
    
    let mut buffer: Vec<u8> = vec![];
    
    open(&mut f3);
    let f3_len = f3.read(&mut buffer);
    close(&mut f3);
    
    let text = String::from_utf8_lossy(&buffer);
    
    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_len);
    println!("{}", text);    
}

























