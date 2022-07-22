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
extern crate x86_64;


// all methods to use
use core::panic::PanicInfo;


// all mods
mod vga_buffer;
mod serial;


// all struct / trait outlines
pub trait Testable {
	fn run(&self) -> ();
}


// all constants / enums / statics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}


// all implementations
impl<T> Testable for T
where
	T: Fn(),
{
	fn run(&self) {
		serial_print!("{}...\t", core::any::type_name::<T>());
		self();
		serial_println!("[ok]");
	}
}


// testing stuff
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

	serial_println!("[failed]\n");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed);
	
	loop {}

}


// all functions
// function called on panic
#[cfg(not(test))]
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

pub fn exit_qemu(exit_code: QemuExitCode) {
	
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}

}
