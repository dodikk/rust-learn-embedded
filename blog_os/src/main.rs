
#![no_std]


// for unit testing
//
// // #![no_main]
//
//
#![cfg_attr(not(test), no_main)] 

// We get a few warnings about unused items, because we no longer compile our _start function.
// Silencing them...
//
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]


#![feature(panic_implementation)]


// silence code style enforcing
// no other people on the team. no project support expected.
//
#![allow(non_snake_case)]



// error[E0468]: an `extern crate` loading macros must be at the crate root
//
//

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;
// use vga_buffer::{Writer};







use core::panic::PanicInfo;


extern crate spin;
// use spin::Mutex;


// use core::fmt;
// use core::fmt::{Write};




/// This function is called on panic.
//
#[panic_implementation]
#[no_mangle]
#[cfg(not(test))]
pub fn panic(_info: &PanicInfo) -> ! 
{
	println!("{}", _info);
    loop {}
}


/*
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
*/

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! 
{
	// ::vga_buffer::print_something();

	println!("Hello World{}", "!");

	use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();


    // panic!("Test failure");

    loop {}
}


#[cfg(test)]
pub fn _main()
{
	// IDLE
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
