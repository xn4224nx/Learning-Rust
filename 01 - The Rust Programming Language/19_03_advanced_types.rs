use std::fmt;
use std::io::Error;


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// The Never Type that Never Returns
fn bar() -> ! {
    panic!();
}

fn main() {
    
    // Using the Newtype Pattern to Implement External Traits on External Types
    let w = Wrapper(vec![String::from("Hello"), String::from("world!")]);
    println!("w = {}", w);
    
    // Creating Type Synonyms with Type Aliases
    type Kilometers = i32;
    
    let x: i32 = 5;
    let y: Kilometers = 5;
    
    println!("x + y = {}", x + y);
    
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let f: Thunk = Box::new(|| println!("hi"));
    
    bar();
}

