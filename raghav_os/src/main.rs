// don't link Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// don't mangle name of function
#[no_mangle]
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since linker looks for a function named `_start` by default
	loop {}
}
