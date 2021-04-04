#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use simple_os::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use simple_os::task::{Task, simple_executor::SimpleExecutor};
use simple_os::task::keyboard;

extern crate alloc;


entry_point!(kernel_main);

fn kernel_main (boot_info: &'static BootInfo) -> ! {
    use simple_os::memory::{self, BootInfoFrameAllocator};
    use simple_os::allocator;
    use x86_64::VirtAddr;
 

    println!("Hello world{}", "!");
    simple_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap_initialization failed");

    /*
    let heap_value = Box::new(41);
    println!("Heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();

    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );

    core::mem::drop(reference_counted);

    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
    */

    #[cfg(test)]
    test_main();

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses())); // new
    executor.run();

    println!("It did not crash!");
    simple_os::hlt_loop(); 
}



async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    simple_os::hlt_loop();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_os::test_panic_handler(info)
}
