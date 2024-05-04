/* Prepare for not having as OS. */
#![no_std]
#![no_main]

/* Unlock the LLVM compiler's intrinsic functions. */
#![feature(core_intrinsics)]
use core::intrinsics;

/* Allow the panic handler to inspect where the error occured. */
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    
    unsafe {
        /* Increment the pointer by one and set the background to cyan */
        framebuffer.offset(1).write_volatile(0x30);
    }
    
    loop {}
}
