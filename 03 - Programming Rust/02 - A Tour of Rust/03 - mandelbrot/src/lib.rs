use num::Complex;
use std::str::FromStr;

/// Determine if `c` is in the Mandelbrot set.
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        /* Return the number of iterations that it took to prove it wasn't in */
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    /* If it can't be proved to be outside the set. */
    return None;
}

/// Parse the commandline arguments that define image resolution.
pub fn parse_img_res<T: FromStr>(raw_res: &str, separator: char) -> Option<(T, T)> {
    /* Ensure the seperator exists in the res string. */
    return match raw_res.find(separator) {
        /* If it doesn't exist return None. */
        None => None,

        /* If it does parse the resolutions using the index of the sep. */
        Some(index) => {
            /* Try and extract the two halves of the resolution. */
            match (
                T::from_str(&raw_res[..index]),
                T::from_str(&raw_res[index + 1..]),
            ) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    };
}

/// Convert a string of containing two numbers to a complex float
pub fn parse_complex_res(raw_res: &str) -> Option<Complex<f64>> {
    return match parse_img_res(raw_res, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    };
}
