use spin::Mutex;

pub struct KeyBuffer {
    buf: [u8; 256],
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl KeyBuffer {
    const fn new() -> Self {
        KeyBuffer {
            buf: [0; 256],
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, key: u8) {
        if self.count < 256 {
            self.buf[self.write_pos] = key;
            self.write_pos = (self.write_pos + 1) % 256;
            self.count += 1;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.count == 0 {
            return None;
        }
        let key = self.buf[self.read_pos];
        self.read_pos = (self.read_pos + 1) % 256;
        self.count -= 1;
        Some(key)
    }
}

pub static KEY_BUFFER: Mutex<KeyBuffer> = Mutex::new(KeyBuffer::new());

static mut SHIFT_HELD: bool = false;

// Scancode set 1 -> ASCII (unshifted)
#[rustfmt::skip]
static SCANCODE_UNSHIFTED: [u8; 128] = [
    0,   27,  b'1', b'2', b'3', b'4', b'5', b'6',  // 0x00-0x07
    b'7', b'8', b'9', b'0', b'-', b'=', 8,   b'\t', // 0x08-0x0F
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', // 0x10-0x17
    b'o', b'p', b'[', b']', b'\n', 0,   b'a', b's', // 0x18-0x1F
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', // 0x20-0x27
    b'\'', b'`', 0,   b'\\', b'z', b'x', b'c', b'v', // 0x28-0x2F
    b'b', b'n', b'm', b',', b'.', b'/', 0,   b'*', // 0x30-0x37
    0,   b' ', 0,   0,    0,    0,    0,    0,      // 0x38-0x3F
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x40-0x47
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x48-0x4F
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x50-0x57
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x58-0x5F
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x60-0x67
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x68-0x6F
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x70-0x77
    0,   0,   0,   0,    0,    0,    0,    0,       // 0x78-0x7F
];

// Scancode set 1 -> ASCII (shifted)
#[rustfmt::skip]
static SCANCODE_SHIFTED: [u8; 128] = [
    0,   27,  b'!', b'@', b'#', b'$', b'%', b'^', // 0x00-0x07
    b'&', b'*', b'(', b')', b'_', b'+', 8,   b'\t', // 0x08-0x0F
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', // 0x10-0x17
    b'O', b'P', b'{', b'}', b'\n', 0,   b'A', b'S', // 0x18-0x1F
    b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', // 0x20-0x27
    b'"', b'~', 0,   b'|', b'Z', b'X', b'C', b'V',  // 0x28-0x2F
    b'B', b'N', b'M', b'<', b'>', b'?', 0,   b'*',  // 0x30-0x37
    0,   b' ', 0,   0,    0,    0,    0,    0,       // 0x38-0x3F
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x40-0x47
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x48-0x4F
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x50-0x57
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x58-0x5F
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x60-0x67
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x68-0x6F
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x70-0x77
    0,   0,   0,   0,    0,    0,    0,    0,        // 0x78-0x7F
];

pub fn handle_scancode(scancode: u8) {
    let is_release = scancode & 0x80 != 0;
    let key = scancode & 0x7F;

    // Track shift state
    if key == 0x2A || key == 0x36 {
        unsafe {
            SHIFT_HELD = !is_release;
        }
        return;
    }

    if is_release {
        return;
    }

    let ascii = if unsafe { SHIFT_HELD } {
        SCANCODE_SHIFTED[key as usize]
    } else {
        SCANCODE_UNSHIFTED[key as usize]
    };

    if ascii != 0 {
        KEY_BUFFER.lock().push(ascii);
    }
}
