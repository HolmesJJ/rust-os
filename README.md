# Rust OS 从零构建操作系统内核（OS Kernel）

[OS Tutorial](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2020)

基于 RISC-V 架构、使用 Rust 语言 编写的简易操作系统。

RISC-V: 一个开源的 CPU 架构标准。

QEMU: 开源的仿真器，它能在 Windows 或 Mac 电脑上通过软件模拟出一整套 RISC-V 硬件环境。

SBI: 硬件固件提供给操作系统的一组"服务密码"。通过特定的寄存器和 `ecall` 指令，可以告诉底层硬件具体的操作，如：`请帮我显示这个字母`或`请帮我断电关机`。

## 项目结构

| 文件 | 详细描述 |
| :--- | :--- |
| **.cargo/config.toml** | 编译地图，配置默认目标架构和链接脚本路径。 |
| **src/entry.asm** | 启动入口，汇编编写，负责设置 CPU 的栈空间（Stack）并跳转到 Rust 代码。 |
| **src/main.rs** | 内核入口，定义了 `rust_main` 函数，是 Rust 代码执行的起点。 |
| **src/sbi.rs** | 封装了 SBI 调用，让内核能命令硬件做打印字符、关机等操作。 |
| **src/console.rs** | 实现了 `print!` 和 `println!` 宏，打印文字。 |
| **src/panic.rs** | 当程序报错（Panic）时，负责打印红色的错误信息并安全关机。 |
| **src/linker.ld** | 告诉编译器代码应该存放在内存的哪个绝对地址（0x80200000）。 |
| **src/context.rs** | 定义了如何保存 CPU 的寄存器状态，为实现"进程切换"做准备。 |
| **src/interrupt/mod.rs** | 中断模块管理，对外暴露 `init()` 接口，统一调度子模块。 |
| **src/interrupt/context.rs** | 程序快照定义，定义了 `Context` 结构体，用于保存 32 个通用寄存器及状态寄存器。 |
| **src/interrupt/handler.rs** | 中断指挥中心，负责将中断入口告知硬件，并编写 Rust 层的异常处理逻辑。 |
| **src/interrupt/timer.rs** | 系统心脏起搏器，负责初始化硬件定时器、预约下一次时钟中断，并维护全局时间计数 TICKS。 |
| **src/interrupt/interrupt.asm** | 现场保存/恢复，汇编编写，负责在中断发生时按下"快门"保存状态。 |
| **src/memory/config.rs** | 内核配置中心，目前定义了 `KERNEL_HEAP_SIZE` 为 8MB，这是内核动态分配内存的上限。 |
| **src/memory/heap.rs** | 内存管理器，通过 `LockedHeap` 划分堆空间，支持内核动态扩容。 |
| **src/memory/mod.rs** | 内存模块入口，负责向外暴露 `init()` 接口，统一初始化底层的堆内存管理器。 |
| **Cargo.toml** | 项目清单，配置依赖和终止策略。 |
| **Makefile** | 一键编译、转换格式、并启动 QEMU 模拟器运行内核。 |

## 写 OS 必须用 Nightly 版本
```bash
rustup toolchain install nightly
rustup override set nightly
rustup target add riscv64imac-unknown-none-elf
rustup component add llvm-tools-preview
```

## 如何运行

```bash
make run
```

1. QEMU 模拟器启动，加载固件（OpenSBI）。
2. 固件跳转在 `linker.ld` 中指定的地址。
3. `entry.asm` 首先接管 CPU，开辟一块 64KB 的内存作为"栈"，然后跳转到 `rust_main`。
4. `main.rs` 开始运行，输出 `Hello rCore-Tutorial!`。
