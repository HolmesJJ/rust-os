// 进行动态内存分配所用的堆空间

// 引入同级模块 config 里的堆大小定义
use crate::memory::config::KERNEL_HEAP_SIZE;
// 引入外部分配器库
use buddy_system_allocator::LockedHeap;
// 引入获取原始指针的宏
use core::ptr::addr_of_mut;

// 在内存中预留出 8MB 的连续空间
// 这段空间在编译后会被放在 bss 段，意味着它不占用内核二进制文件的体积，
// 但在程序加载到内存时会被初始化为 0。
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

// 堆，动态内存分配器
// ### `#[global_allocator]`
// “以后凡是代码里用到 Box、Vec 等需要分内存的地方，都来找这个 HEAP 变量”。
// LockedHeap 是一个带有锁的分配器，保证了在多核或中断环境下分配内存是安全的。
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

// 初始化操作系统运行时堆空间
pub fn init() {
    // 告诉分配器：这是管辖的 8MB 区间。
    // 我们把 HEAP_SPACE 的起始地址和大小传给它。
    unsafe {
        // 直接拿到 HEAP_SPACE 的起始内存地址。
        let heap_start = addr_of_mut!(HEAP_SPACE) as usize;
        HEAP.lock().init(
            heap_start, 
            KERNEL_HEAP_SIZE
        )
    }
}

// 空间分配错误的回调
// 如果区间用完了（8MB 耗尽），或者请求分配的内存太大，
// 这个函数会被自动调用。我们选择直接 panic 报错，防止程序带着错误的地址跑下去。
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}
