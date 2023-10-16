use std::rc::Rc;

fn main() {
    
    /* Rust can infer the type but they have been written for clarity. */
    let s: Rc<String> = Rc::new("Shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = t.clone();
    
    println!("{}\n", u);
    
    /* Normal string functions work on the three pointers to the string. */
    println!("{}\n{}\n{}", 
        s.contains("shira"),
        t.find("taki").unwrap(),
        u,
    );
}
