// 调用 Machine 层的操作
// 暂时忽略未使用的变量或函数警告
#![allow(unused)]

// --- 模块引用与内嵌汇编 ---
use core::arch::asm;
// SBI 调用核心函数：这是内核请求 OpenSBI 服务的唯一标准入口
// - which: 服务编号（Extension ID），放在 x17 寄存器
// - arg0, arg1, arg2: 传递给服务的参数，分别放在 x10, x11, x12 寄存器
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret; // 用于接收从寄存器返回的结果
    unsafe {
        asm!(
            "ecall",                        // 核心指令：环境调用。触发异常，让 CPU 跳入 M 模式（OpenSBI）
            // 寄存器绑定（RISC-V SBI 标准协议）：
            inlateout("x10") arg0 => ret,   // 输入：arg0 放入 x10；输出：执行后的 x10 存入 ret
            in("x11") arg1,                 // 输入：arg1 放入 x11
            in("x12") arg2,                 // 输入：arg2 放入 x12
            in("x17") which,                // 输入：服务编号放入 x17
        );
    }
    ret // 返回 OpenSBI 给我们的处理结果或错误码
}

// --- SBI 服务编号常量定义 ---
// 这些编号是由 RISC-V SBI 标准协议定义的“服务清单”
const SBI_SET_TIMER: usize = 0;              // 设置定时器
const SBI_CONSOLE_PUTCHAR: usize = 1;        // 输出字符到控制台
const SBI_CONSOLE_GETCHAR: usize = 2;        // 从控制台读取字符
const SBI_CLEAR_IPI: usize = 3;              // 清除核间中断
const SBI_SEND_IPI: usize = 4;               // 发送核间中断
const SBI_REMOTE_FENCE_I: usize = 5;         // 远程指令缓存刷新
const SBI_REMOTE_SFENCE_VMA: usize = 6;      // 远程地址映射刷新
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7; // 远程地址映射刷新（带地址空间 ID）
const SBI_SHUTDOWN: usize = 8;               // 关闭操作系统（关机）

// 向控制台输出一个字符
// 注意：参数 c 使用 usize 而非 char，是因为底层寄存器处理的是字长大小的数据
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

// 从控制台中读取一个字符
// 如果当前缓冲区没有字符，通常返回 -1
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

// 调用 SBI_SHUTDOWN 来关闭操作系统
// -> ! 表示这个函数是“发散”的，即它永远不会返回（因为机器已经关了）
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    // 如果关机指令执行完程序还没停，说明出大问题了，手动标记为不可达
    unreachable!()
}

// 设置下一次时钟中断的时间
pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0);
}
