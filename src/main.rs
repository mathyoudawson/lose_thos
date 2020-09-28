#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Defualt entry point - name mangling changes the signature of this method so some hash
// Linker requires a function called `_start` as entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
        loop {}
}

// Called on panc... required when excluding std library
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        loop {}
}
