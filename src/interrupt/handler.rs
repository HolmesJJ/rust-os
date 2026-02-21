use core::arch::global_asm;
use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::{Scause, Trap, Exception, Interrupt};

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
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    let cause = scause.cause();
    // 可以通过 Debug 来查看发生了什么中断
    println!("{:x?}", cause);
    match cause {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context, 2),
        // 捕获非法内存访问 (LoadFault)
        // 当程序尝试读取非法地址（如 0x0）时，硬件会触发这个异常
        Trap::Exception(Exception::LoadFault) => {
            // stval 记录了触发异常的那个非法地址
            if stval == 0x0 {
                println!("SUCCESS!");
            }
            breakpoint(context, 4);
        },
        // 时钟中断
        // 通过 super 调用同级目录下的 timer 模块
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(),
        // 其他情况，调用故障处理
        _ => fault(context, scause, stval),
    }
}


// 处理 ebreak 断点
// sepc 记录的是触发中断的指令地址。对于 ebreak，我们手动 +2 字节跳过它。
// 在 RISC-V 中
// ebreak 指令占 2 个字节。当 ebreak 触发中断时，sepc 指向的是 ebreak 本身。
// 如果我们不手动 +2，中断返回后 CPU 又会执行 ebreak，导致陷入死循环。
// main.rs 中的 `ld t0, (x0)` 是一条 4 字节指令，
// 我们必须手动将 sepc 增加 4，才能跳过它执行下一条语句。
fn breakpoint(context: &mut Context, offset: usize) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += offset;
}

// 处理时钟中断
// 目前只会在 [`timer`] 模块中进行计数
fn supervisor_timer() {
    // 调用 timer 模块中的 tick 函数
    super::timer::tick();
}

// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\nContext: {:x?}\nstval: 0x{:x}",
        scause.cause(),
        context,
        stval
    );
}
