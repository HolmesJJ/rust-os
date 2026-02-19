// 中断模块。
// 该模块负责处理所有硬件中断和异常。

// 1. 声明子模块
// 这告诉 Rust 编译器去寻找同目录下的 handler.rs 和 context.rs 文件
mod handler;
mod context;

// 初始化中断相关的子模块。
// 这是整个中断模块的对外总入口。
// 内部流程：
// - 调用 [`handler::init`]：设置中断向量表 (stvec)，让 CPU 知道出事了往哪跑。
pub fn init() {
    // 执行 handler 模块里的初始化逻辑。
    handler::init(); 

    // 打印一条调试信息，确认中断模块已经成功挂载。
    // 注意：这里的 println! 是你在 console.rs 里实现的宏。
    println!("mod interrupt initialized");
}
