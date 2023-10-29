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

    /* Find the square root of the first perfect square in the series. */
    let mut n = 5;

    let sqrt = 'outer: loop {
        for i in 1.. {
            let square = i * i;

            if square == n {
                /* A square root has been found. */
                break 'outer n;
            }

            if square > n {
                /* `n` is  not a perfect square. */
                break;
            }
        }

        n += 1;
    };

    println!("{} is a perfect square.", sqrt);
}
