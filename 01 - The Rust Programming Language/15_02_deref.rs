use std::ops::Deref;

/// Custom smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    
    fn new(x: T) -> MyBox<T> {
        return MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    
    /* Create an instance of the custom smart pointer */
    let a = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
