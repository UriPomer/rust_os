#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
