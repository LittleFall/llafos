#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(llafos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use llafos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("llafos 0.0.3\n\n");
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("Some panic message");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    llafos::test_panic_handler(info)
}
