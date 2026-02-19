use core::arch::global_asm;
use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::Scause;

// 1. 嵌入汇编代码
// 将 interrupt.asm 里的汇编指令直接拼接到这个模块生成的机器码中。
// 这样才能在下面引用到汇编里定义的 `__interrupt` 符号。
global_asm!(include_str!("./interrupt.asm"));

// 初始化中断处理
// 把中断入口 `__interrupt` 写入 `stvec` 中，告诉 CPU：
// “一旦发生中断，请立刻跳到 __interrupt 那个地方去！”
pub fn init() {
    // 2. 外部符号声明
    // 告诉 Rust 编译器：`__interrupt` 是在别的地方（汇编文件）定义的，
    // 按照 C 语言的调用约定来处理它。
    unsafe {
        unsafe extern "C" {
            // `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 3. 设置中断向量表寄存器 (stvec)
        // stvec::write 负责完成底层的寄存器写入操作。
        // 参数 1: `__interrupt as usize` 
        //         这是汇编入口点的内存地址。
        //
        // 参数 2: `stvec::TrapMode::Direct` 
        //         这是“直接模式”。意味着无论是时钟中断、非法指令还是系统调用，
        //         CPU 统统跳到同一个入口（__interrupt）去处理。
        stvec::write(__interrupt as *const () as usize, stvec::TrapMode::Direct);
    }
}

// 中断的具体处理逻辑
// #[unsafe(no_mangle)]: 防止 Rust 编译器修改函数名，确保汇编里的 `jal handle_interrupt` 能精准找到它。
// 参数说明（这些参数都是由 interrupt.asm 里的汇编代码准备好并传进来的）：
// - context: 指向栈上保存的全部寄存器快照的指针。
// - scause:  原因寄存器。记录了是因为什么中断（比如断点、读写错误、时钟等）。
// - stval:   附加信息。比如地址访问错误时，这里存的是那个错误的内存地址。
#[unsafe(no_mangle)]
pub fn handle_interrupt(_context: &mut Context, scause: Scause, _stval: usize) {
    // 目前阶段，任何中断都会触发 panic 并打印原因
    // scause.cause() 会告诉你具体发生了什么（例如 Exception::Breakpoint）
    panic!("Interrupted: {:?}", scause.cause());
}
