#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use r_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    r_os::init();

    #[cfg(test)]
    test_main();

    println!("It didn't crash");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    r_os::test_panic_handler(info)
}
