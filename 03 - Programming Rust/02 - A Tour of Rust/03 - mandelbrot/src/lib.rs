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

/// Given the row & column of a pixel return the point on the complex plane.
///
/// `image_size` - the width & height of the image.
/// `pixel_coords` - the column & row of the pixel to be converted.
/// `upper_left` - The upper left coordinate of the image cover.
/// `lower_right` - The lower right coordinate of the image cover.
pub fn pixel_point_to_complex(
    image_size: (usize, usize),
    pixel_coords: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    return Complex {
        re: upper_left.re + pixel_coords.0 as f64 * width / image_size.0 as f64,
        im: upper_left.im - pixel_coords.1 as f64 * height / image_size.1 as f64,
    };
}

/// Render a rectangle of the Mandlebrot set into a buffer of pixels
///
///     `bounds` - gives the width & height of the buffer `pixels`.
///     `upper_left` - The upper left coordinate of pixel buffer.
///     `lower_right` - The lower right coordinate of pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    /* Check that there are enough pixels. */
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_point_to_complex(bounds, (col, row), upper_left, lower_right);

            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}
