fn main() {
    
    assert!((-1.0 / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5.0);
    assert_eq!((-1.01_f64).floor(), -2.0);

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
}
