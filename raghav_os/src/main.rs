// don't link Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

// implement custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// change name of generated function to something other than `main`
#![reexport_test_harness_main = "test_main"]


// all crates to use
use core::panic::PanicInfo;


// all mods
mod vga_buffer;


// testing stuff
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
}

#[test_case]
fn trivial_assertion() {
	print!("trivial assertion... ");
	assert_eq!(1, 1);
	print!("[ok]");
}


// all functions
// function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}


// all public functions
// don't mangle name of function
#[no_mangle]
pub extern "C" fn _start() -> ! {
	
	// this function is the entry point, since linker looks for a function named `_start` by default

	println!("Hello World{}", "!");
	
	#[cfg(test)]
	test_main();

	loop {}

}
