use std::io;
use std::io::BufRead;

fn main() {
    let mut i: i32 = 0;

    /* While Loop */
    println!("\nWhile Loop");

    while i < 4 {
        println!("\ti = {}", i);
        i += 1;
    }

    /* While Let Loop */
    println!("\nWhile Let Loop");
    let mut opt_val = Some(0);

    while let Some(n) = opt_val {
        if n > 9 {
            println!("\tLoop exited.");
            opt_val = None;
        } else {
            println!("\tLoop value = {}", n);
            opt_val = Some(n + 1)
        }
    }

    /* Loop Loop */
    println!("\nLoop Loop");

    loop {
        println!("\ti = {}", i);
        i += 1;
        if i > 10 {
            break;
        };
    }

    /* For Loop */
    println!("\nFor Loop");

    for j in 0..=7 {
        println!("\tj = {}", j);
    }

    /* For loops consume the value unless refernced with & */
    let errors = vec!["404", "File not found", "File access restricted"];

    println!("\nErrors:");
    for err in &errors {
        println!(
            "\tError: '{}', Error Adr: {:?}",
            err,
            std::ptr::addr_of!(err)
        );
    }

    println!("\nNumber of errors: {}\n", errors.len());

    /* Control Flows in Loops */

    let answer = loop {
        /* Read from stdin into `buffer */
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer);

        /* Check if the string matches what we want */
        if buffer.trim().to_lowercase().starts_with("answer:") {
            /* Remove the start of the buffer. */
            buffer = buffer.trim().replace("answer:", "");

            /* Return buffer as the value for answer. */
            break buffer;
        } else {
            println!("Give value that starts `answer:` you gave {}", buffer);
        }
    };

    println!("The line you gave was: `{}`", answer);
}
