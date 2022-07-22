// all headers
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raghav_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


// all methods to use
use core::panic::PanicInfo;


// all functions
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	test_main();

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	raghav_os::test_panic_handler(info)
}
