use mandelbrot::*;
use num::Complex;

#[test]
fn test_parse_img_res() {
    assert_eq!(parse_img_res::<i32>("", ','), None);
    assert_eq!(parse_img_res::<i32>("10,", ','), None);
    assert_eq!(parse_img_res::<i32>(",10", ','), None);
    assert_eq!(parse_img_res::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_img_res::<i32>("10,20xy", ','), None);
    assert_eq!(parse_img_res::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_img_res::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex_res() {
    assert_eq!(
        parse_complex_res("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex_res("-0.0625"), None);
}
