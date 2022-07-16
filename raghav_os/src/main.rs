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

static HELLO: &[u8] = b"Hello World! Welcome to the RHVSY OS!";

// don't mangle name of function
#[no_mangle]
pub extern "C" fn _start() -> ! {
	// this function is the entry point, since linker looks for a function named `_start` by default
	
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	loop {}
}
