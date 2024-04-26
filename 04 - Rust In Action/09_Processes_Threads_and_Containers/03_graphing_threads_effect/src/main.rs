use std::{env, thread, time};

fn main() {
    /* Collect the command line arguments. */
    let args: Vec<String> = env::args().collect();

    /* Ensure that there are enough arguments. */
    if args.len() != 3 {
        panic!("Two arguments needed.");
    }

    /* Parse the number of iterations. */
    let iterations: usize = args[2]
        .parse::<usize>()
        .expect("The second argument must be an integer");

    let method = &args[1];

    /* Run the thread allocation timing simulations. */
    for n in 1..=iterations {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();

        /* Construct the threads. */
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handle);
        }

        /* Unwind the threads. */
        while let Some(handle) = handlers.pop() {
            let _ = handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
