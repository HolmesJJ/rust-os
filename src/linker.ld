/* 目标架构：告诉链接器要为 RISC-V 架构生成程序。 */
OUTPUT_ARCH(riscv)

/* 执行入口：指定整个程序的起始执行点是 _start 标签（在 entry.asm 中定义） */
ENTRY(_start)

/* 定义一个常量：数据存放的起始物理地址。在 RISC-V QEMU 中，0x80200000 是内核起始的常规位置。 */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* 当前地址计数器。这里将地址设置为 BASE_ADDRESS，即从 0x80200000 开始排布。 */
    . = BASE_ADDRESS;

    /* 定义一个全局符号 kernel_start，标记内核在内存中的起始位置。 */
    kernel_start = .;

    text_start = .;

    /* .text 段：存放编译后的机器指令代码。 */
    .text : {
        /* 把 .text.entry 段放在最前面！这确保了 _start 是内核的第一条指令。 */
        *(.text.entry)
        /* 将所有其他输入文件的 .text 段和子段集中放在这里。 */
        *(.text .text.*)
    }

    /* 记录代码段结束、只读数据段开始的位置。 */
    rodata_start = .;

    /* .rodata 段：存放只读数据，比如字符串常量。 */
    .rodata : {
        *(.rodata .rodata.*)
    }

    /* 记录只读数据段结束、已初始化数据段开始的位置。 */
    data_start = .;

    /* .data 段：存放已经初始化的全局变量。 */
    .data : {
        *(.data .data.*)
    }

    /* 记录已初始化数据段结束、未初始化数据段开始的位置。 */
    bss_start = .;

    /* .bss 段：存放未初始化的全局变量（比如在 entry.asm 里定义的栈空间）。 */
    .bss : {
        *(.sbss .bss .bss.*)
    }

    /* 记录整个内核结束的地址。 */
    kernel_end = .;
}
