use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
       let val = String::from("hi");
       println!("val is {}", val);
       tx.send(val).unwrap(); 

    });
    
    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);
    
    
    thread::spawn(move || {
        
        let msg = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for word in msg {
            tx1.send(word).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for recieved in rx {
        println!("Got: {}", recieved);
        
    }
}
