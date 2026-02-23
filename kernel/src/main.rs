#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga_buffer;
mod serial;
mod block_device;
mod ramdisk;
mod font;
mod framebuffer;
mod gdt;
mod pic;
mod interrupts;
mod keyboard;
mod shell;

use core::panic::PanicInfo;
use core::fmt::Write;
use block_device::{BlockDevice, BLOCK_SIZE};
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsStartMarker, RequestsEndMarker};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static _REQUEST_START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static _REQUEST_END: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut serial = serial::SERIAL.lock();

    writeln!(serial, "ShadowOS v0.1.0").unwrap();
    writeln!(serial, "================").unwrap();
    writeln!(serial).unwrap();

    // Initialize GDT (must be first — IDT references TSS)
    gdt::init();
    writeln!(serial, "[*] GDT initialized").unwrap();

    // Initialize PIC (remap IRQs to 32-47, all masked)
    pic::init();
    writeln!(serial, "[*] PIC remapped (IRQ 0-15 -> vectors 32-47)").unwrap();

    // Initialize IDT
    interrupts::init();
    writeln!(serial, "[*] IDT loaded").unwrap();

    // Unmask keyboard IRQ (IRQ1)
    pic::unmask_irq(1);
    writeln!(serial, "[*] Keyboard IRQ unmasked").unwrap();

    // Initialize framebuffer
    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = response.framebuffers().next() {
            writeln!(serial, "[*] Framebuffer: {}x{}, {}bpp, pitch={}",
                     fb.width(), fb.height(), fb.bpp(), fb.pitch()).unwrap();

            framebuffer::init(
                fb.addr(),
                fb.width() as usize,
                fb.height() as usize,
                fb.pitch() as usize,
                fb.bpp() as usize,
                fb.red_mask_shift(),
                fb.green_mask_shift(),
                fb.blue_mask_shift(),
            );

            writeln!(serial, "[*] Framebuffer initialized").unwrap();
        } else {
            writeln!(serial, "[!] No framebuffers available").unwrap();
        }
    } else {
        writeln!(serial, "[!] Framebuffer request not answered by bootloader").unwrap();
    }

    // Initialize RAM disk
    writeln!(serial, "[*] Initializing RAM disk...").unwrap();
    ramdisk::init();

    // Test RAM disk
    test_ramdisk(&mut serial);

    writeln!(serial, "\n[*] Kernel initialization complete.").unwrap();
    writeln!(serial, "[*] Enabling interrupts...").unwrap();

    // Drop serial lock before enabling interrupts to prevent deadlock
    drop(serial);

    // Enable interrupts
    x86_64::instructions::interrupts::enable();

    // Hand off to the interactive shell
    shell::run();
}

fn test_ramdisk(serial: &mut serial::SerialPort) {
    let mut ramdisk_guard = ramdisk::RAMDISK.lock();

    if let Some(ref mut ramdisk) = *ramdisk_guard {
        let block_count = ramdisk.block_count();
        writeln!(serial, "    RAM disk has {} blocks ({} KB)",
                 block_count, block_count * BLOCK_SIZE as u64 / 1024).unwrap();

        // Test writing to block 0
        writeln!(serial, "[*] Testing RAM disk I/O...").unwrap();

        let mut write_buffer = [0u8; BLOCK_SIZE];
        let test_data = b"ShadowOS RAM disk test block!";
        write_buffer[..test_data.len()].copy_from_slice(test_data);

        match ramdisk.write_block(0, &write_buffer) {
            Ok(_) => writeln!(serial, "    Write to block 0: OK").unwrap(),
            Err(e) => writeln!(serial, "    Write to block 0: FAILED ({:?})", e).unwrap(),
        }

        // Test reading from block 0
        let mut read_buffer = [0u8; BLOCK_SIZE];
        match ramdisk.read_block(0, &mut read_buffer) {
            Ok(_) => {
                writeln!(serial, "    Read from block 0: OK").unwrap();

                // Verify the data
                if &read_buffer[..test_data.len()] == test_data {
                    writeln!(serial, "    Data verification: PASSED").unwrap();
                } else {
                    writeln!(serial, "    Data verification: FAILED").unwrap();
                }
            },
            Err(e) => writeln!(serial, "    Read from block 0: FAILED ({:?})", e).unwrap(),
        }

        // Test out of bounds access
        match ramdisk.read_block(block_count + 1, &mut read_buffer) {
            Ok(_) => writeln!(serial, "    Out of bounds test: FAILED (should have errored)").unwrap(),
            Err(_) => writeln!(serial, "    Out of bounds test: PASSED").unwrap(),
        }
    } else {
        writeln!(serial, "    ERROR: RAM disk not initialized!").unwrap();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Disable interrupts in panic to prevent re-entrancy
    x86_64::instructions::interrupts::disable();

    let mut serial = serial::SERIAL.lock();
    writeln!(serial, "\nPANIC!").unwrap();
    if let Some(location) = info.location() {
        writeln!(serial, "{}:{}: {}", location.file(), location.line(), info.message()).unwrap();
    } else {
        writeln!(serial, "{}", info.message()).unwrap();
    }
    loop {
        x86_64::instructions::hlt();
    }
}
