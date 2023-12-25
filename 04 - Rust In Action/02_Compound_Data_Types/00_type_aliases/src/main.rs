#![allow(unused_variables)]

/* Define the new 'File' type. */
type File = String;

fn open(f: &mut File) -> bool {
    return true;
}

fn close(f: &mut File) -> bool {
    return true;
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    /* This macro causes a panic if encountered.  The ! return type means the
    fn never returns */
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    let mut path: Vec<u8> = vec![];
    open(&mut f1);
    //read(&mut f1, &mut path);
    close(&mut f1);
}
