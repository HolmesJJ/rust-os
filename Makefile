# 一键编译、转换格式、运行 QEMU

# 目标平台架构
TARGET      := riscv64imac-unknown-none-elf
# 编译模式（debug 或 release）
MODE        := debug
# 编译生成的 ELF 格式内核文件位置
KERNEL_FILE := target/$(TARGET)/$(MODE)/os
# 转换后的二进制镜像位置
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

# 使用 rust-binutils 提供的工具
# objdump: 用来查看反汇编代码（看机器码长啥样）
# objcopy: 用来剥离文件信息（把 ELF 变成纯净的机器指令集）
OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

# .PHONY 告诉 Makefile 这些是“命令名字”而不是“实际文件名字”，防止文件名冲突
.PHONY: doc kernel build clean qemu run

# 默认目标：执行 build 会生成 .bin 文件
build: $(BIN_FILE) 

# 生成内核文档（方便你以后查看代码架构）
doc:
	@cargo doc --document-private-items

# 编译内核：调用 cargo build 产生 ELF 文件
kernel:
	@cargo build

# 【关键步骤】将 ELF 转换成纯二进制格式 (.bin)
# --strip-all 会删掉调试信息，只留下 CPU 能听懂的指令流
$(BIN_FILE): kernel
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

# 反汇编：如果你想看 Rust 代码变成了什么样的 RISC-V 汇编，用这个命令
asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less

# 清理：删掉 target 目录下所有编译产物
clean:
	@cargo clean

# 运行 QEMU：这是最核心的测试命令
qemu: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios default \
		-kernel $(KERNEL_FILE)

# 一键运行：最常用的命令，先构建再运行 QEMU
run: build
	@make qemu
