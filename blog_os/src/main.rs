
#![no_std]
#![no_main]
#![feature(panic_implementation)]


use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}


//======= Rust (Linux ??)
//
//
// #[cfg(target_os = "linux")]
//
//

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}




/*
============== OS dependent trampoline ============== 
==
==


//======= Mac OS
//
//


#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn main() -> ! 
{
    loop {}
}


//======= Windows
//
//

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}


#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn main() -> ! 
{
    loop {}
}


==
==
==============[END] OS dependent trampoline [END]============== 
*/
