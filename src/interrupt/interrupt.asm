# -------------------------------------------------------------------------
# 这一部分是汇编工具设置
# -------------------------------------------------------------------------
.altmacro                # 开启宏的替代模式，允许我们在宏里使用循环和变量
.set    REG_SIZE, 8      # 定义寄存器宽度：64 位 RISC-V 每个寄存器占 8 字节
.set    CONTEXT_SIZE, 34 # 定义 Context 结构体成员数：32个通用寄存器 + sstatus + sepc

# 宏 (Macro)：像函数一样的模板，用来减少重复劳动
# 将指定的寄存器存入栈中对应的位置
.macro SAVE reg, offset
    sd  \reg, \offset*8(sp)
.endm

.macro SAVE_N n
    SAVE  x\n, \n
.endm

# 将寄存器从栈中恢复
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm

.macro LOAD_N n
    LOAD  x\n, \n
.endm

    .section .text
    .globl __interrupt
# -------------------------------------------------------------------------
# __interrupt: 保存“案发现场”
# -------------------------------------------------------------------------
__interrupt:
    # 1. 在栈上开辟空间。sp 指针向下移动 34*8 字节，腾出位子放 Context 结构体
    addi    sp, sp, -34*8

    # 2. 开始保存通用寄存器
    SAVE    x1, 1        # x1 是返回地址 (ra)
    
    # 特殊处理 sp (x2)：
    # 我们现在的 sp 已经移动过了，但我们要保存的是“发生中断那一刻”的旧 sp
    addi    x1, sp, 34*8 # 计算旧 sp 的地址
    SAVE    x1, 2        # 将旧 sp 存入偏移量为 2 的位置

    # 3. 循环保存 x3 至 x31
    # .rept 29 表示重复执行 29 次，自动保存剩下所有的寄存器
    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr

    # 4. 保存控制状态寄存器 (CSR)
    # 这些寄存器记录了中断发生时的 CPU 状态（如是否开中断、之前的特权级等）
    csrr    s1, sstatus  # 读取 sstatus 存入临时寄存器 s1
    csrr    s2, sepc     # 读取 sepc（发生中断时代码执行到了哪一行）存入 s2
    SAVE    s1, 32       # 存入 Context
    SAVE    s2, 33       # 存入 Context

    # 5. 为跳转到 Rust 的 handle_interrupt 函数准备参数
    # 根据 RISC-V 调用约定：a0, a1, a2 分别存放前三个参数
    # context: &mut Context
    mv      a0, sp       # 第一个参数：Context 的指针（当前栈顶地址）
    # scause: Scause
    csrr    a1, scause   # 第二个参数：中断原因（为什么中断？）
    # stval: usize
    csrr    a2, stval    # 第三个参数：中断附加信息（比如访存错误的地址）
    
    # 6. 跳转到 Rust 写的业务逻辑处理函数
    jal  handle_interrupt

    .globl __restore
# -------------------------------------------------------------------------
# __restore: 恢复“案发现场”并返回
# -------------------------------------------------------------------------
__restore:
    # 1. 恢复控制状态寄存器 (CSR)
    LOAD    s1, 32
    LOAD    s2, 33
    csrw    sstatus, s1  # 把值写回硬件寄存器
    csrw    sepc, s2     # 这样一会儿 sret 才知道回哪去

    # 2. 恢复通用寄存器 x1 和 x3~x31
    LOAD    x1, 1
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr

    # 3. 最后恢复栈指针 sp (x2)
    # 一旦恢复了 sp，我们就回到了发生中断前的那个栈
    LOAD    x2, 2

    # 4. 关键指令：sret (Supervisor Return)
    # 这条指令会让 CPU 自动完成以下三件事：
    # 1) 把 pc 设为 sepc 的值。
    # 2) 恢复 CPU 的特权等级和中断使能状态。
    # 3) 继续执行原来被中断的代码。
    sret
