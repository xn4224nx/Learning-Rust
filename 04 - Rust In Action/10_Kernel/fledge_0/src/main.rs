/* Prepare for not having as OS. */
#![no_std]
#![no_main]
#![feature(lang_items)]
/* Unlock the LLVM compiler's intrinsic functions. */
#![feature(core_intrinsics)]
use core::intrinsics;

/* Allow the panic handler to inspect where the error occured. */
use core::panic::PanicInfo;

use x86_64::instructions::hlt;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        /* Increment the pointer by one and set the background to cyan */
        framebuffer.offset(1).write_volatile(0x30);
    }

    loop {
        hlt();
    }
}
