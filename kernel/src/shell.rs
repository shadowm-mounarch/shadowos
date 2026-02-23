use core::fmt::Write;
use x86_64::instructions::interrupts::without_interrupts;
use x86_64::instructions::hlt;

use crate::framebuffer;
use crate::serial;
use crate::keyboard;
use crate::ramdisk;
use crate::block_device::{BlockDevice, BLOCK_SIZE};

// --- LineBuffer: stack-allocated input buffer ---

struct LineBuffer {
    buf: [u8; 256],
    len: usize,
}

impl LineBuffer {
    const fn new() -> Self {
        LineBuffer {
            buf: [0; 256],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) -> bool {
        if self.len < self.buf.len() {
            self.buf[self.len] = byte;
            self.len += 1;
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> bool {
        if self.len > 0 {
            self.len -= 1;
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn as_str(&self) -> &str {
        // All bytes pushed are printable ASCII, so this is safe
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

// --- FmtBuf: stack-allocated Write target for formatting numbers ---

struct FmtBuf {
    buf: [u8; 256],
    pos: usize,
}

impl FmtBuf {
    fn new() -> Self {
        FmtBuf {
            buf: [0; 256],
            pos: 0,
        }
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.pos]) }
    }
}

impl Write for FmtBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            if self.pos < self.buf.len() {
                self.buf[self.pos] = b;
                self.pos += 1;
            }
        }
        Ok(())
    }
}

// --- Output helpers ---

fn echo_byte(byte: u8) {
    without_interrupts(|| {
        let mut fb = framebuffer::FRAMEBUFFER.lock();
        if let Some(ref mut writer) = *fb {
            writer.write_byte(byte);
        }
    });
    without_interrupts(|| {
        let mut serial = serial::SERIAL.lock();
        serial.write_byte(byte);
    });
}

fn print_str(s: &str) {
    for &b in s.as_bytes() {
        echo_byte(b);
    }
}

fn print_prompt() {
    print_str("shadow> ");
}

fn do_backspace() {
    // Erase on framebuffer
    without_interrupts(|| {
        let mut fb = framebuffer::FRAMEBUFFER.lock();
        if let Some(ref mut writer) = *fb {
            writer.backspace();
        }
    });
    // Erase on serial: BS, space, BS
    without_interrupts(|| {
        let mut serial = serial::SERIAL.lock();
        serial.write_byte(8);
        serial.write_byte(b' ');
        serial.write_byte(8);
    });
}

// --- Command dispatch ---

fn execute(line: &str) {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return;
    }

    let (cmd, args) = match trimmed.find(' ') {
        Some(pos) => (&trimmed[..pos], trimmed[pos + 1..].trim_start()),
        None => (trimmed, ""),
    };

    match cmd {
        "help" => cmd_help(),
        "clear" => cmd_clear(),
        "echo" => cmd_echo(args),
        "info" => cmd_info(),
        "reboot" => cmd_reboot(),
        _ => {
            print_str("Unknown command: ");
            print_str(cmd);
            print_str("\n");
        }
    }
}

fn cmd_help() {
    print_str("Available commands:\n");
    print_str("  help    - Show this help message\n");
    print_str("  clear   - Clear the screen\n");
    print_str("  echo    - Print text to the screen\n");
    print_str("  info    - Show system information\n");
    print_str("  reboot  - Reboot the system\n");
}

fn cmd_clear() {
    without_interrupts(|| {
        let mut fb = framebuffer::FRAMEBUFFER.lock();
        if let Some(ref mut writer) = *fb {
            writer.clear_screen();
        }
    });
}

fn cmd_echo(args: &str) {
    print_str(args);
    print_str("\n");
}

fn cmd_info() {
    // Collect framebuffer info into a stack buffer (avoids holding lock while printing)
    let mut fbuf = FmtBuf::new();

    without_interrupts(|| {
        let fb = framebuffer::FRAMEBUFFER.lock();
        if let Some(ref writer) = *fb {
            let _ = write!(
                fbuf,
                "Framebuffer: {}x{}\nText grid:   {}x{}\n",
                writer.width(),
                writer.height(),
                writer.max_cols(),
                writer.max_rows()
            );
        }
    });

    // Collect ramdisk info
    let mut rbuf = FmtBuf::new();

    without_interrupts(|| {
        let rd = ramdisk::RAMDISK.lock();
        if let Some(ref ramdisk) = *rd {
            let blocks = ramdisk.block_count();
            let kb = blocks * BLOCK_SIZE as u64 / 1024;
            let _ = write!(rbuf, "RAM disk:    {} blocks ({} KB)\n", blocks, kb);
        }
    });

    print_str("ShadowOS v0.1.0\n");
    print_str(fbuf.as_str());
    print_str(rbuf.as_str());
}

fn cmd_reboot() {
    print_str("Rebooting...\n");
    // Write 0xFE to keyboard controller command port to trigger reset
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x64u16,
            in("al") 0xFEu8,
            options(nomem, nostack)
        );
    }
    // Safety net: halt if reset doesn't happen immediately
    loop {
        hlt();
    }
}

// --- Main shell entry point ---

pub fn run() -> ! {
    print_str("ShadowOS v0.1.0\n");
    print_str("Type 'help' for available commands.\n\n");
    print_prompt();

    let mut line = LineBuffer::new();

    loop {
        let key = without_interrupts(|| {
            keyboard::KEY_BUFFER.lock().pop()
        });

        if let Some(byte) = key {
            match byte {
                b'\n' => {
                    echo_byte(b'\n');
                    execute(line.as_str());
                    line.clear();
                    print_prompt();
                }
                8 => {
                    // Backspace
                    if line.pop() {
                        do_backspace();
                    }
                }
                b'\t' => {
                    // Ignore tabs
                }
                0x20..=0x7E => {
                    // Printable ASCII
                    if line.push(byte) {
                        echo_byte(byte);
                    }
                }
                _ => {
                    // Ignore non-printable
                }
            }
        }

        hlt();
    }
}
