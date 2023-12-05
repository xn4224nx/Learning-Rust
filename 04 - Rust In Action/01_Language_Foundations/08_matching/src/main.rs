use rand::random;

fn main() {
    loop {
        let rnd_num = random::<u8>();

        match rnd_num {
            /* Match a single value. */
            0 => {
                break;
            }

            /* Match an inclusive range. */
            10..=20 => {
                print!("{} ", rnd_num);
            }

            /* Match either of the values */
            40 | 80 | 100 => {
                println!();
            }

            /* Default action if no other matches are found. */
            _ => {}
        }
    }
}
