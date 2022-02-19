#![no_std]

use core::panic::PanicInfo;

// function called when panicking
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

fn main() {}
