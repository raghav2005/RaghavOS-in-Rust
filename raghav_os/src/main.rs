// all headers
// don't link Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

// implement custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(raghav_os::test_runner)]

// change name of generated function to something other than `main`
#![reexport_test_harness_main = "test_main"]


// all crates to use
extern crate x86_64;
extern crate raghav_os;


// all methods to use
use core::panic::PanicInfo;
use raghav_os::println;


// all mods


// all struct / trait outlines


// all constants / enums / statics


// all implementations


// all testing stuff
// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	raghav_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}


// all functions
// function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

// don't mangle name of function
#[no_mangle]
pub extern "C" fn _start() -> ! {
	
	// this function is the entry point, since linker looks for a function named `_start` by default

	println!("Hello World{}", "!");
	
	#[cfg(test)]
	test_main();

	loop {}

}
