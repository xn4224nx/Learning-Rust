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

    /* Collect the type of simulation and check validity */
    let raw_method = args[1].trim().to_lowercase();
    let method = &raw_method.as_str();
    
    let valid_methods = vec!["spin", "sleep"];
    if !valid_methods.contains(&method) {
        panic!("'{}' is an invalid method!", method);
    }

    /* Run the thread allocation timing simulations. */
    for n in 1..=iterations {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();

        /* Construct the threads. */
        for _m in 0..n {
            let start = time::Instant::now();

            /* Create a thread that yields after the pause. */
            if method == &valid_methods[1] {
                let handle = thread::spawn(|| {
                    let start = time::Instant::now();
                    let pause = time::Duration::from_millis(20);

                    while start.elapsed() < pause {
                        thread::yield_now()
                    }
                });
                handlers.push(handle);

            /* Create a thread that sleeps. */
            } else if method == &valid_methods[0] {
                let handle = thread::spawn(|| {
                    let pause = time::Duration::from_millis(20);
                    thread::sleep(pause);
                });
                handlers.push(handle);

            /* Make Sure a valid thread is executed. */
            } else {
                panic!(
                    "Method '{}' has reached an area it should not have.",
                    method
                );
            }  
        }

        /* Unwind the threads. */
        while let Some(handle) = handlers.pop() {
            let _ = handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
