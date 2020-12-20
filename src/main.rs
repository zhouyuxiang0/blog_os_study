#![no_std]
#![no_main]
use core::panic::PanicInfo;
/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // 禁用名称重写 编译器将输出_start函数
pub extern "C" fn _start() -> ! {
    loop {}
}
