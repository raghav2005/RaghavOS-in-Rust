extern crate uart_16550;
extern crate spin;
extern crate lazy_static;


use serial::uart_16550::SerialPort;
use serial::spin::Mutex;
use serial::lazy_static::lazy_static;


lazy_static! {
	pub static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = unsafe {
			SerialPort::new(0x3F8)
		};
		serial_port.init();
		Mutex::new(serial_port)
	};
}


// prints to host through serial interface
#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
	};
}

// prints to host through serial interace + appends newline
#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use core::fmt::Write;
	SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}
