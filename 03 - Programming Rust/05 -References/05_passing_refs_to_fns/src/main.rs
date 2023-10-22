// This could be written as fn g(p: &i32)
fn g<'a>(a: &'a i32) {

    println!("{}", a);
}

fn main() {
    
    let x = 10;
    g(&x);   
}
