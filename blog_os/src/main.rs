
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

#[no_mangle]
pub extern "C" fn _start() -> ! 
{
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
