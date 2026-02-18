// 代替标准库 (std)，实现内核级的 panic（恐慌）和 abort（中止）功能

use core::panic::PanicInfo;
use crate::sbi::shutdown;

// 当代码发生致命错误（panic）时，Rust 编译器会自动调用这个函数
// ### `#[panic_handler]` 属性
// 告诉编译器：这个函数就是处理所有 Panic 的“终极回调”
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // `\x1b[??m` 是控制终端字符输出格式的指令，在支持的平台上可以改变文字颜色等等，这里使用红色
    // \x1b[1;31m 是 ANSI 转义码
    // 1 表示加粗，31 表示红色。这能让错误信息在终端里非常醒目
    // \x1b[0m 表示重置颜色格式，防止后面的打印也被染成红色
    // 参考：https://misc.flogisoft.com/bash/tip_colors_and_formatting
    //
    // 需要全局开启 feature(panic_info_message) 才可以调用 .message() 函数
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message());
    // 报错打印完后，直接调用我们之前写的 sbi 关机函数，退出模拟器
    // 这样就不会让 CPU 毫无意义地在后台空转
    shutdown()
}

// 终止程序
// 调用 [`panic_handler`]
#[unsafe(no_mangle)] // 保持函数名不被混淆，方便链接器找到 "abort" 符号
extern "C" fn abort() -> ! {
    // 直接触发一个 panic，进而进入上面的 panic_handler
    panic!("abort()")
}
