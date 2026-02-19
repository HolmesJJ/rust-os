// 让操作系统“动起来”的关键，没有它，内核就是一个静止的程序，而不是一个动态的系统。
// 预约和处理时钟中断

use crate::sbi::set_timer; // 调用 sbi.rs 里的设置定时器功能
use riscv::register::{time, sie, sstatus}; // 引入 RISC-V 核心寄存器操作

// 1. 时钟中断的间隔
// 这里的 100000 是 CPU 的时钟周期数。
// 比如 CPU 频率是 10MHz，那么每秒会发生 100 次中断（10,000,000 / 100,000）。
static INTERVAL: usize = 100000;

// 2. 触发时钟中断计数
// static mut 表示这是一个全局可变的变量，用来记录系统启动以来跳动了多少次。
pub static mut TICKS: usize = 0;

// 初始化时钟中断
// 开启硬件开关
pub fn init() {
    unsafe {
        // 开启 STIE (Supervisor Timer Interrupt Enable)
        // 告诉硬件：我想要接收来自定时器的中断信号。
        sie::set_stimer(); 

        // 开启 SIE (Supervisor Interrupt Enable)
        // 这是一个全局开关，允许内核态的代码被中断打断。
        sstatus::set_sie();
    }
    // 预约第一次时钟中断，否则闹钟永远不会响第一次。
    set_next_timeout();
}

// 设置下一次时钟中断
// 计算公式：下一次响铃时间 = 当前时间 + 固定的间隔。
fn set_next_timeout() {
    // time::read() 读取 RISC-V 硬件寄存器 `time` 的当前值。
    set_timer(time::read() + INTERVAL);
}

// 每一次时钟中断时调用的业务逻辑
// 这个函数通常会被 `handle_interrupt` 调用。
pub fn tick() {
    // 1. 极其重要：必须预约下一次中断，否则闹钟就变成“一次性”的了
    set_next_timeout();
    unsafe {
        TICKS += 1;
        // 2. 计数器自增
        let current_ticks = TICKS;
        
        // 3. 为了不让屏幕被刷屏，我们每隔 100 次打印一次
        if current_ticks % 100 == 0 {
            println!("{} tick", current_ticks);
        }

        // 4. 当心跳达到 500 次（大约 5 秒）时自动关机
        if current_ticks >= 500 {
            println!("Time's up! Shutting down...");
            crate::sbi::shutdown(); // 直接调用 sbi 模块里的关机函数
        }
    }
}
