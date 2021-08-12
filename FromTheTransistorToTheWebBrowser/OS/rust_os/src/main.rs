#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod uart;
pub mod vga_buffer;

use core::panic::PanicInfo;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////

// #[macro_export]
// macro_rules! print
// {
// 	($($args:tt)+) => ({
// 			use core::fmt::Write;
// 			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
// 	});
// }
// #[macro_export]
// macro_rules! println
// {
// 	() => ({
// 		print!("\r\n")
// 	});
// 	($fmt:expr) => ({
// 		print!(concat!($fmt, "\r\n"))
// 	});
// 	($fmt:expr, $($args:tt)+) => ({
// 		print!(concat!($fmt, "\r\n"), $($args)+)
// 	});
// }

// ///////////////////////////////////
// START
// ///////////////////////////////////

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");

    serial_println!("This is my operating system!");
    serial_println!("I'm so awesome. If you start typing, I'll show you what you typed!");

    #[cfg(test)]
    test_main();

    loop {}
}

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
    println!("[ok]");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
