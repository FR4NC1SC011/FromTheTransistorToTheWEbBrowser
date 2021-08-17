#![no_std]
#![no_main]

use uart_16550::SerialPort;

use core::panic::PanicInfo;
use rust_os::{serial_print, serial_println};

const SERIAL_IO_PORT: u16 = 0x3F8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // should_fail();
    // serial_println!("[test did not panic]");
    input_uart();
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

fn input_uart() {
    let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    serial_port.init();

    serial_println!("Input Something");
    loop {
        let data = serial_port.receive();
        serial_print!("{}", data as char);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    loop {}
}
