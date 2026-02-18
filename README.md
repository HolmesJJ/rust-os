# Rust OS 从零构建操作系统内核（OS Kernel）

[OS Tutorial](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2020)

基于 RISC-V 架构、使用 Rust 语言 编写的简易操作系统。

RISC-V: 一个开源的 CPU 架构标准。

QEMU: 开源的仿真器，它能在 Windows 或 Mac 电脑上通过软件模拟出一整套 RISC-V 硬件环境。

SBI: 硬件固件提供给操作系统的一组"服务密码"。通过特定的寄存器和 `ecall` 指令，可以告诉底层硬件具体的操作，如：`请帮我显示这个字母`或`请帮我断电关机`。

## 项目结构

| 文件 | 详细描述 |
| :--- | :--- |
| **src/entry.asm** | 启动入口，汇编编写，负责设置 CPU 的栈空间（Stack）并跳转到 Rust 代码。 |
| **src/main.rs** | 内核入口，定义了 `rust_main` 函数，是 Rust 代码执行的起点。 |
| **src/sbi.rs** | 封装了 SBI 调用，让内核能命令硬件做打印字符、关机等操作。 |
| **src/console.rs** | 实现了 `print!` 和 `println!` 宏，打印文字。 |
| **src/panic.rs** | 当程序报错（Panic）时，负责打印红色的错误信息并安全关机。 |
| **src/linker.ld** | 告诉编译器代码应该存放在内存的哪个绝对地址（0x80200000）。 |
| **src/context.rs** | 定义了如何保存 CPU 的寄存器状态，为实现"进程切换"做准备。 |
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
