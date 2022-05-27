#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("llafos 0.0.3\n\n");
    println!("Hello World{}", "!");

    // panic!("Some panic message");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
