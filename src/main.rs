#![no_std]
#![no_main]

mod uart;
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Test{}", "!");
    panic!("C'est kc");
    loop {}
}
