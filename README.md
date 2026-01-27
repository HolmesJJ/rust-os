# Rust OS

[OS Tutorial](https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code-2020)

## 写 OS 必须用 Nightly 版本
```bash
rustup toolchain install nightly
rustup override set nightly
rustup target add riscv64imac-unknown-none-elf
rustup component add llvm-tools-preview
```
