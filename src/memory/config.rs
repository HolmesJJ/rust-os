// 操作系统动态分配内存所用的堆大小（8M）
// 0x80_0000 换算成十进制就是 8,388,608 字节，即 8MB
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
