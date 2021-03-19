#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use simple_os::{QemuExitCode, exit_qemu, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[test_case]
fn should_fail() {
    serial_println!("Should_panic::Should_fail...\t");
    assert_eq!(0,1);
}
