// 引入 riscv 库中封装好的寄存器类型
use riscv::register::sstatus::Sstatus;

// Context：程序的瞬间快照
// 当硬件中断（比如时钟中断或键盘输入）发生时，CPU 会强行停下当前正在运行的代码，转去执行“中断处理程序”。
// 为了让原本的代码在中断处理完后能像“什么都没发生过”一样继续运行，必须在处理前把 CPU 当下的所有状态保存下来，这个保存的状态集就叫 Context（上下文） 。

// #[repr(C)] 是非常关键的属性，保证内存顺序：x[0], x[1]...x[31], sstatus, sepc。
// 它告诉 Rust 编译器：这个结构体在内存中的布局必须和 C 语言标准一致。
// 这样在后续写汇编代码来手动保存/恢复这些寄存器时，能准确知道每个成员的偏移位置。
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    // 32 个通用寄存器 (x0 ~ x31)
    // 无论程序在做什么运算，数据都在这 32 个寄存器里。
    // usize 类型在 64 位 RISC-V 下就是 64 位，刚好对应一个寄存器的大小。
    pub x: [usize; 32],     

    // sstatus (Supervisor Status) 寄存器
    // 记录了 CPU 当前的状态，比如是否开启中断、之前的特权级是什么等。
    // 中断处理可能会改变状态，所以必须保存原样。
    pub sstatus: Sstatus,

    // sepc (Supervisor Exception Program Counter) 寄存器
    // 极其重要！它记录了中断发生那一刻，程序运行到了哪一行指令（地址）。
    // 恢复上下文时，会把这个值放回 PC，程序就能从断点处继续。
    pub sepc: usize
}
