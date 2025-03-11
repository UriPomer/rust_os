#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    loop {}
}
