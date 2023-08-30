fn main() {
    
    assert_eq!('*' as i32, 42);
    assert_eq!('ʘ' as i32, 664); 
    assert_eq!('ʘ' as u16, 0x298);
    assert_eq!('ʘ' as i8, -0x68);
    
    /* Print using a hex number, ʘ. */
    println!("{}", '\u{298}');
    
    /* Some standard char methods. */
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ʘ'.len_utf8(), 2);
    assert_eq!('ʘ'.len_utf16(), 1);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}
