#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;
/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
static HELLO: &[u8] = b"Hello World!";
#[no_mangle]    // 禁用名称重写 编译器将输出_start函数
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
