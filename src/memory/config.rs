// 操作系统动态分配内存所用的堆大小（8M）
// 0x80_0000 换算成十进制就是 8,388,608 字节，即 8MB
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

use lazy_static::lazy_static;
use super::address::PhysicalAddress;

lazy_static! {
    // 内核代码结束的地址，即可以用来分配的内存起始地址
    // 这里修复了“函数直接强转 usize”的警告
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as *const () as usize);
}

unsafe extern "C" {
    /// 由 `linker.ld` 指定的内核代码结束位置
    /// 作为变量存在 [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}
