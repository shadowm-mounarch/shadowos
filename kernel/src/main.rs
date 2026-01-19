#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let ptr = framebuffer.addr() as *mut u32;
            let count = (framebuffer.width() * framebuffer.height()) as usize;
            for i in 0..count {
                unsafe {
                    *ptr.add(i) = 0xFF0000FF; // Blue
                }
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
