#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    /* Create an instance of the structure 'File' */
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    
    /* Access fields by reference. */
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
