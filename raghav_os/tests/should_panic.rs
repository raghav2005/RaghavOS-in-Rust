// all headers
#![no_std]
#![no_main]


// all crates to use
extern crate raghav_os;


// all methods to use
use core::panic::PanicInfo;
use raghav_os::{QemuExitCode, exit_qemu, serial_print, serial_println};


// all functions
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	should_fail();
	serial_println!("[test did not panic]");
	exit_qemu(QemuExitCode::Failed);
	loop {}
}

fn should_fail() {
	serial_print!("should_panic::should_fail...\t");
	assert_eq!(0, 1);
}
