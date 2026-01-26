// 全局属性
// 禁用标准库
#![no_std]

// 不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]

// 内嵌汇编
use core::arch::asm;
use core::arch::global_asm;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

// 当 panic 发生时会调用该函数
// 暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 在屏幕上输出一个字符
pub fn console_putchar(ch: u8) {
    let mut _ret: usize; // 变量必须声明为 mut，因为汇编会写入它
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => _ret, // 输入 ch 到 x10，执行后将返回值存入 _ret
            in("x11") arg1,                
            in("x12") arg2,                
            in("x17") which,               
            options(nostack)               
        );
    }
}

// Rust 的入口函数
// 覆盖 crt0 中的 _start 函数
// 暂时将它的实现为一个死循环
// 针对 Rust 1.82+ 版本
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');
    loop {}
}
