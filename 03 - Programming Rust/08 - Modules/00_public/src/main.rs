/* Constants that are public. */
pub const ROOM_TEMP_CELS: f64 = 20.0;

/* Statics are almost the same thing. */
pub static ROOM_TEMP_FAHR: f64 = 68.0;

/* A public structures fields are accessable beyond the module. */
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn main() {
    println!("Room temp is {ROOM_TEMP_CELS}°C or {ROOM_TEMP_FAHR}°F");
}
