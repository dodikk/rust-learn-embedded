
#![no_std]
#![no_main]
#![feature(panic_implementation)]


// error[E0468]: an `extern crate` loading macros must be at the crate root
//
//

#[macro_use]
extern crate lazy_static;








use core::panic::PanicInfo;


extern crate spin;
use spin::Mutex;


use core::fmt;
use core::fmt::{Write};


mod vga_buffer;
use vga_buffer::{Writer};


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

fn print_abstractionless()
{
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() 
    {
        unsafe 
        {
            *vga_buffer.offset(i as isize * 2    ) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb ;
        }
    }

}


#[no_mangle]
pub extern "C" fn _start() -> ! 
{
	// ::vga_buffer::print_something();

	use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();


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
