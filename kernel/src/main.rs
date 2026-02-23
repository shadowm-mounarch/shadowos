#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello, world!").unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();
    writer.write_str("\nPANIC!\n").unwrap();
    if let Some(location) = info.location() {
        write!(writer, "{}:{}: {}\n", location.file(), location.line(), info.message()).unwrap();
    } else {
        write!(writer, "{}\n", info.message()).unwrap();
    }
    loop {}
}
