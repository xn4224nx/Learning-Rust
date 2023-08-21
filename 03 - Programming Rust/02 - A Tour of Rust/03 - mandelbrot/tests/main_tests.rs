use mandelbrot::*;

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
