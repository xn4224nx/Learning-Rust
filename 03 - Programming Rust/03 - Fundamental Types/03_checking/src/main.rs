fn main() {
    let mut i = 1;
    
    /* This will overflow after the 9th iteration */
    for x in 0..9 {
        println!("{} {}", x, i);
        i *= 10;
    }
    
    /* Checked operations */
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128_i8).checked_div(-1), None);
    
    /* Do the addition and panic if it overflows. */
    let sum = 10_u8.checked_add(20).unwrap();
    println!("{}", sum);
    
    /* Wrapping - if the value overflows you get the modulus. So for the second
    assert the result will be (500*500)%(2**16). */
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    
    /* On signed values it can go negative. */
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    
    /* For bit operations if the shift distance is greater than the value it is 
    shifted around, ie 17-bit shift goes to a 1-bit shift */
    assert_eq!(5_i16.wrapping_shl(17), 10);
}
