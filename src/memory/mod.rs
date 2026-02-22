// 声明该文件夹下的子模块，并设为公开（pub）供外部使用
pub mod config;
pub mod heap;
pub mod address;

// 内存模块的统一初始化入口
pub fn init() {
    heap::init();
}
