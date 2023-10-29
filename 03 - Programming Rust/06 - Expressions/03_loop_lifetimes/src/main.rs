fn main() {
    'search: for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                println!("[{:>3}, {:>3}, {:>3}]", i, j, k);

                if i * j * k == 27 {
                    /* Exit all of the loops */
                    break 'search;
                }
            }
        }
    }
}
