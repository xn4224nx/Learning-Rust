fn main() {
    
    let fruit_icns = vec!['🍑', '🍈', '🍎', '🍋', '🍇'];
    
    /* Element does't exist. */
    let buff_overflow = fruit_icns[5];
    assert_eq!(buff_overflow, '🍌')   
}
