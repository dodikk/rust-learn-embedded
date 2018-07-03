
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

#[no_mangle]
pub extern "C" fn _start() -> ! 
{
    loop {}
}



//======= Mac OS
//
//

#[no_mangle]
pub extern "C" fn main() -> ! 
{
    loop {}
}


//======= Windows
//
//

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

// #[no_mangle]
// pub extern "C" fn main() -> ! 
// {
//     loop {}
// }

