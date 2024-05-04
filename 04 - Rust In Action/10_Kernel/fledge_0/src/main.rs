/* Prepare for not having as OS. */
#![no_std]
#![no_main]
#![feature(lang_items)]
/* Unlock the LLVM compiler's intrinsic functions. */
#![feature(core_intrinsics)]

use core::fmt::{self, Write};
use core::intrinsics;
/* Allow the panic handler to inspect where the error occured. */
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Colour {
    Black = 0x0,
    White = 0xF,
    Blue = 0x1,
    BrightBlue = 0x9,
    Green = 0x2,
    BrightGreen = 0xA,
    Cyan = 0x3,
    BrightCyan = 0xB,
    Red = 0x4,
    BrightRed = 0xC,
    Magenta = 0x5,
    BrightMagenta = 0xD,
    Brown = 0x6,
    Yellow = 0xE,
    Gray = 0x7,
    DarkGrey = 0x8,
}

struct Cursor {
    position: isize,
    foreground: Colour,
    background: Colour,
}

impl Cursor {
    fn colour(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        return fg | bg;
    }

    fn print(&mut self, text: &[u8]) {
        let colour = self.colour();
        let framebuffer = 0xb8000 as *mut u8;

        for &char in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(char);
                framebuffer.offset(self.position + 1).write_volatile(colour);
            }
            self.position += 2;
        }
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        return Ok(());
    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    let mut cur = Cursor {
        position: 0,
        foreground: Colour::White,
        background: Colour::Red,
    };

    for _ in 0..(80 * 25) {
        cur.print(b" ");
    }
    cur.position = 0;
    write!(cur, "{}", info).unwrap();

    loop {
        unsafe {
            hlt();
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = b"Hello, world!";

    let mut cur = Cursor {
        position: 0,
        foreground: Colour::BrightBlue,
        background: Colour::Yellow,
    };
    cur.print(text);

    panic!("help!");

    loop {
        hlt();
    }
}
