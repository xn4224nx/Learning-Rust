use rand::prelude::*;

fn one_in(denom: u32) -> bool {
    return thread_rng().gen_ratio(1, denom);
}

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

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();

        save_to.reserve(read_len);
        save_to.append(&mut tmp);

        return Ok(read_len);
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(2) {
        let err_msg = String::from("Permission denied!");
        return Err(err_msg);
    } else {
        return Ok(f);
    }
}

fn close(f: File) -> Result<File, String> {
    if one_in(2) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    } else {
        return Ok(f);
    }
}

fn main() {
    /* Create the file entity. */
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    /* Simulate the file being opened. */
    f4 = open(f4).unwrap();
    let f4_len = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_len);
    println!("{}", text);
}
