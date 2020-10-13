#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate rlibc;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// Defualt entry point - name mangling changes the signature of this method so some hash
// Linker requires a function called `_start` as entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    //
    loop {}
}

// Called on panc... required when excluding std library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
