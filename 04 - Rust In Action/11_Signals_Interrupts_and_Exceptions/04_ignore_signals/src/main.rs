use libc::{raise, signal, SIGTERM, SIG_DFL, SIG_IGN};

fn main() {
    unsafe {
        /* Ignore the sigterm */
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("ok");

    unsafe {
        /* Resets the sigterm to its default. */
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!(" not ok");
}
