#![no_std]
#![no_main]

mod vga_buffer;
mod serial;
mod block_device;
mod ramdisk;

use core::panic::PanicInfo;
use core::fmt::Write;
use block_device::{BlockDevice, BLOCK_SIZE};
use limine::BaseRevision;
use limine::request::{RequestsStartMarker, RequestsEndMarker};

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

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

    // Initialize RAM disk
    writeln!(serial, "[*] Initializing RAM disk...").unwrap();
    ramdisk::init();

    // Test RAM disk
    test_ramdisk(&mut serial);

    writeln!(serial, "\n[*] Kernel initialization complete.").unwrap();

    loop {
        core::hint::spin_loop();
    }
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
    let mut serial = serial::SERIAL.lock();
    writeln!(serial, "\nPANIC!").unwrap();
    if let Some(location) = info.location() {
        writeln!(serial, "{}:{}: {}", location.file(), location.line(), info.message()).unwrap();
    } else {
        writeln!(serial, "{}", info.message()).unwrap();
    }
    loop {
        core::hint::spin_loop();
    }
}
