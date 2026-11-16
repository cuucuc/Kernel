#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod gdt;
mod interrupts;
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
    gdt::init();
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();
    println!("Ca fonctionne");
    loop {}
}
