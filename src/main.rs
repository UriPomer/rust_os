/*
 * @Author: Urim_XXL 3051068921@qq.com
 * @Date: 2025-03-12 00:30:41
 * @LastEditors: Urim_XXL 3051068921@qq.com
 * @LastEditTime: 2025-03-12 17:00:56
 * @FilePath: /rust_os/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// 当 panic 发生时调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
