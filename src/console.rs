// 实现控制台的字符输入和输出
// # 格式化输出
// [`core::fmt::Write`] trait 包含
// - 需要实现的 [`write_str`] 方法
// - 自带实现，但依赖于 [`write_str`] 的 [`write_fmt`] 方法
// 我们声明一个类型，为其实现 [`write_str`] 方法后，就可以使用 [`write_fmt`] 来进行格式化输出
// [`write_str`]: core::fmt::Write::write_str
// [`write_fmt`]: core::fmt::Write::write_fmt

use crate::sbi::*; // 引入之前写的 console_putchar
use core::fmt::{self, Write};

// 声明一个“空结构体”（Zero-Sized Type），实现 [`core::fmt::Write`] trait 来进行格式化输出
// ZST 只可能有一个值（即为空），因此它本身就是一个单件
// 它不占用任何内存空间，仅作为一个载体，用来挂载我们实现的打印方法
struct Stdout;

impl Write for Stdout {
    // 核心方法：打印一个基础字符串
    // 这是整个格式化系统的“地基”
    // [`console_putchar`] sbi 调用每次接受一个 `usize`，但实际上会把它作为 `u8` 来打印字符。
    // 因此，如果字符串中存在非 ASCII 字符，需要在 utf-8 编码下，对于每一个 `u8` 调用一次 [`console_putchar`]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 创建一个 4 字节的缓冲区，因为一个 UTF-8 字符最多占 4 字节
        let mut buffer = [0u8; 4];
        // 遍历字符串中的每一个字符（c 是 char 类型，是 Unicode 标量）
        for c in s.chars() {
            // 将 Unicode 字符编码为 UTF-8 格式的字节序列
            // 比如中文“你”会变成 3 个字节
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                // 逐个字节调用 SBI 服务打印到屏幕
                console_putchar(*code_point as usize);
            }
        }
        Ok(()) // 返回成功
    }
}

// 打印由 [`core::format_args!`] 格式化后的数据
// [`print!`] 和 [`println!`] 宏都将展开成此函数
// [`core::format_args!`]: https://doc.rust-lang.org/nightly/core/macro.format_args.html
// 这个函数是 `print!` 和 `println!` 的真正后台，它接收复杂的 Arguments 对象
pub fn print(args: fmt::Arguments) {
    // 调用 Stdout 的 write_fmt 方法。
    // 注意：write_fmt 是 core::fmt::Write 自动帮我们实现的，
    // 它内部会反复调用我们上面写的 write_str。
    Stdout.write_fmt(args).unwrap();
}

// 实现类似于标准库中的 `print!` 宏
// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! print {
    // 匹配模式：比如 print!("Value: {}", 123)
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 展开为调用上面定义的 print 函数
        // format_args! 是编译器内置宏，负责在编译期解析格式化字符串
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

// 实现类似于标准库中的 `println!` 宏
// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 逻辑与 print! 一致，只是通过 concat! 在末尾自动加了一个换行符 "\n"
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
