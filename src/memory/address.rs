use core::fmt;

// 物理地址 (Physical Address) 的包装类型
// 使用 Newtype 模式封装 `usize`：
// 1. 类型安全：防止物理地址、虚拟地址和普通整数之间发生非预期的隐式转换。
// 2. 语义化：在函数参数中使用 `PhysicalAddress` 比直接用 `usize` 更能清晰表达意图。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysicalAddress(pub usize);

// 为 PhysicalAddress 实现 `Display` trait，方便在内核调试时直接通过 `println!` 输出
impl fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 以 16 进制格式打印地址
        write!(f, "PhysicalAddress(0x{:x})", self.0)
    }
}

impl PhysicalAddress {
    // 将物理地址转换为原始指针
    // 在内核需要直接访问某个特定的物理内存位置时使用。
    // 返回一个 `*const u8` 类型的裸指针。
    #[allow(dead_code)]
    pub fn as_ptr(&self) -> *const u8 {
        self.0 as *const u8
    }

    // 获取内部的数值
    // 有时需要将地址作为普通数字参与运算（如位运算）时使用。
    #[allow(dead_code)]
    pub fn as_usize(&self) -> usize {
        self.0
    }
}
