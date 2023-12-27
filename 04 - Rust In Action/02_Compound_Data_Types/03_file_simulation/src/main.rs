#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    return true;
}

fn close(f: &mut File) -> bool {
    return true;
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    /* Make a copy of the data. */
    let mut tmp = f.data.clone();
    let read_len = tmp.len();

    /* Ensure that there is enough space to fit the data. */
    save_to.reserve(read_len);
    save_to.append(&mut tmp);

    return read_len;
}

fn main() {
    /* Create an instance of the file struct. */
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    /* Interact with the file. */
    open(&mut f2);
    let f2_len = read(&f2, &mut buffer);
    close(&mut f2);

    /* Convert the vec of u8 to a String. */
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_len);
    println!("{}", text);
}
