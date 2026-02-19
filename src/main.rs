// --- 全局属性配置 ---

// 1. 禁用标准库 (std)。因为 std 依赖操作系统（如 Linux），
// 写操作系统只能使用 core 库。
#![no_std]

// 2. 告诉 Rust 编译器不使用常规的 main 入口。
// 正常的 Rust 程序由运行时环境调用 main。
#![no_main]

// panic! 时，获取其中的信息并打印
// #![feature(panic_info_message)]

// --- 模块引用与内嵌汇编 ---
use core::arch::asm;
use core::arch::global_asm;

// #[macro_use] 让 console.rs 里定义的 println! 宏在当前文件（及全局）可用
#[macro_use]
mod console;
mod panic; // 引入我们写的 panic 处理逻辑，后面不再用`core::panic::PanicInfo`
mod sbi;   // 引入 SBI 服务调用
mod interrupt;

// 3. 嵌入之前写好的 entry.asm。
// 这样编译器就会把汇编代码拼接到生成的二进制文件中。
global_asm!(include_str!("entry.asm"));

// --- 核心功能：Panic 处理器 ---

// use core::panic::PanicInfo;

// 4. 当代码发生致命错误（panic）时，Rust 会调用这个函数。
// 因为没有 OS 处理错误，只能让 CPU 进入死循环。
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// --- 底层通信：输出字符 ---

// 在屏幕上输出一个字符
// 这里利用了 RISC-V 的 ecall 指令调用 OpenSBI（类似于调用 BIOS 中断）
pub fn console_putchar(ch: u8) {
    let mut _ret: usize; 
    let arg0: usize = ch as usize; // 参数0：要打印的字符
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;          // SBI 服务编号 1：控制台打印
    unsafe {
        // 使用内联汇编直接操作寄存器
        asm!(
            "ecall",                       // 触发环境调用，跳转到 OpenSBI 执行
            inlateout("x10") arg0 => _ret, // 将 arg0 放入 x10 寄存器，返回结果存入 _ret
            in("x11") arg1,                // x11-x12 填 0
            in("x12") arg2,                
            in("x17") which,               // x17 存放服务编号
            options(nostack)               // 告诉编译器这段汇编不使用栈
        );
    }
}

// --- 内核入口函数 ---
// 5. #[unsafe(no_mangle)]：告诉编译器不要混淆函数名，确保汇编能通过 "rust_main" 找到它。
// 6. extern "C"：使用 C 语言的函数调用约定，确保汇编和 Rust 能正常传递参数。
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    interrupt::init();
    // 调用上面定义的函数，在屏幕上打印 "OK"
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');
    // 通过 console 模块 -> sbi 模块 -> ecall 指令执行
    println!("Hello rCore-Tutorial!");
    unsafe {
        core::arch::asm!("ebreak");
    };
    println!("Waiting for timer ticks... (Ctrl+A then X to exit)");
    // 测试：panic 宏 -> panic_handler -> 红色打印 -> 自动关机
    // panic!("end of rust_main");
    loop {
        // CPU 在这里空转，等待时钟中断强行打断它。
    }
}
