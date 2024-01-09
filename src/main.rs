#![no_std] // prevent link the standard library
#![no_main] //disable all rust entry points

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // do not mangle function name
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    // panic!("Some panic message");
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.336
    )
    .unwrap();

    loop {}
}
