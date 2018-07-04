
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
#![deny(warnings)]




#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]


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

#[macro_use]
mod serial;




use core::panic::PanicInfo;


extern crate spin;
extern crate uart_16550;

extern crate x86_64;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};


#[cfg(test)]
extern crate array_init;


lazy_static! 
{
    static ref IDT: InterruptDescriptorTable = 
    {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() 
{
    IDT.load();
}


extern "x86-interrupt" 
fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!(
        "EXCEPTION: BREAKPOINT\n{:#?}"
      , stack_frame);
}

/// This function is called on panic.
//
#[panic_implementation]
#[no_mangle]
#[cfg(not(feature = "integration-test"))]
#[cfg(not(test))]
pub fn panic(_info: &PanicInfo) -> ! 
{
	println!("{}", _info);

    loop {}
}


/// This function is called on panic.
//
#[panic_implementation]
#[no_mangle]
#[cfg(feature = "integration-test")]
#[cfg(not(test))]
pub fn panic(_info: &PanicInfo) -> ! 
{
    serial_println!("{}", _info);

    unsafe { exit_qemu(); }

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


#[cfg(not(feature = "integration-test"))]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! 
{
    init_idt();

	// ::vga_buffer::print_something();

	println!("Hello World{}", "!");



	use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();


    serial_println!("Hello Host{}", "!");


    // invoke a breakpoint exception
    x86_64::instructions::int3();

    // panic!("Test failure");

    loop {}
}


#[cfg(feature = "integration-test")]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! 
{
    // ???
    init_idt();

    // TODO: invoke some test methods

    unsafe { exit_qemu(); }

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
