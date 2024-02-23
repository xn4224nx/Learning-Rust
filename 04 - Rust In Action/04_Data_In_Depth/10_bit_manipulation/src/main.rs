fn main() {
    let op_code: u16 = 0x71E4;

    let c = (op_code & 0xF000) >> 12;
    let x = (op_code & 0x0F00) >> 8;
    let y = (op_code & 0x00F0) >> 4;
    let d = (op_code & 0x000F) >> 0;

    assert_eq!(c, 0x7);
    assert_eq!(x, 0x1);
    assert_eq!(y, 0xE);
    assert_eq!(d, 0x4);

    let nnn = op_code & 0x0FFF;
    let ooo = op_code & 0x00FF;
    let ppp = op_code & 0x000F;

    assert_eq!(nnn, 0x1E4);
    assert_eq!(ooo, 0xE4);
    assert_eq!(ppp, 0x4);
}
