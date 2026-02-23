use crate::font::{FONT_8X16, FONT_HEIGHT, FONT_WIDTH};
use core::fmt;
use core::ptr;
use spin::Mutex;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

pub struct FramebufferWriter {
    buffer: *mut u8,
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    col: usize,
    row: usize,
    max_cols: usize,
    max_rows: usize,
    fg: Color,
    bg: Color,
}

unsafe impl Send for FramebufferWriter {}

impl FramebufferWriter {
    pub fn new(
        buffer: *mut u8,
        width: usize,
        height: usize,
        pitch: usize,
        bpp: usize,
        red_shift: u8,
        green_shift: u8,
        blue_shift: u8,
    ) -> Self {
        let bytes_per_pixel = bpp / 8;
        let max_cols = width / FONT_WIDTH;
        let max_rows = height / FONT_HEIGHT;

        let mut writer = FramebufferWriter {
            buffer,
            width,
            height,
            pitch,
            bytes_per_pixel,
            red_shift,
            green_shift,
            blue_shift,
            col: 0,
            row: 0,
            max_cols,
            max_rows,
            fg: Color::new(0xCC, 0xCC, 0xCC), // light gray
            bg: Color::new(0x00, 0x00, 0x00), // black
        };
        writer.clear_screen();
        writer
    }

    fn color_to_pixel(&self, color: Color) -> u32 {
        ((color.r as u32) << self.red_shift)
            | ((color.g as u32) << self.green_shift)
            | ((color.b as u32) << self.blue_shift)
    }

    fn put_pixel(&self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bytes_per_pixel;
        let pixel = self.color_to_pixel(color);
        unsafe {
            ptr::write_volatile(self.buffer.add(offset) as *mut u32, pixel);
        }
    }

    fn render_char(&self, c: u8, col: usize, row: usize) {
        let idx = (c as usize) & 0x7F;
        let glyph = &FONT_8X16[idx * FONT_HEIGHT..(idx + 1) * FONT_HEIGHT];

        let x0 = col * FONT_WIDTH;
        let y0 = row * FONT_HEIGHT;

        for (dy, &bits) in glyph.iter().enumerate() {
            for dx in 0..FONT_WIDTH {
                let on = (bits >> (7 - dx)) & 1 != 0;
                let color = if on { self.fg } else { self.bg };
                self.put_pixel(x0 + dx, y0 + dy, color);
            }
        }
    }

    fn scroll_up(&self) {
        let row_bytes = FONT_HEIGHT * self.pitch;
        let total_rows = self.max_rows;

        unsafe {
            // Move all rows up by one character row
            let dst = self.buffer;
            let src = self.buffer.add(row_bytes);
            let count = (total_rows - 1) * row_bytes;
            ptr::copy(src, dst, count);

            // Clear the last row
            let last_row_start = self.buffer.add((total_rows - 1) * row_bytes);
            ptr::write_bytes(last_row_start, 0, row_bytes);
        }
    }

    fn new_line(&mut self) {
        self.col = 0;
        if self.row + 1 < self.max_rows {
            self.row += 1;
        } else {
            self.scroll_up();
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= self.max_cols {
                    self.new_line();
                }
                self.render_char(byte, self.col, self.row);
                self.col += 1;
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.render_char(b' ', self.col, self.row);
        } else if self.row > 0 {
            self.row -= 1;
            self.col = self.max_cols - 1;
            self.render_char(b' ', self.col, self.row);
        }
        // At (0, 0): do nothing
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn max_cols(&self) -> usize {
        self.max_cols
    }

    pub fn max_rows(&self) -> usize {
        self.max_rows
    }

    pub fn clear_screen(&mut self) {
        let total_bytes = self.height * self.pitch;
        unsafe {
            ptr::write_bytes(self.buffer, 0, total_bytes);
        }
        self.col = 0;
        self.row = 0;
    }
}

impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static FRAMEBUFFER: Mutex<Option<FramebufferWriter>> = Mutex::new(None);

pub fn init(
    buffer: *mut u8,
    width: usize,
    height: usize,
    pitch: usize,
    bpp: usize,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
) {
    let writer = FramebufferWriter::new(
        buffer, width, height, pitch, bpp,
        red_shift, green_shift, blue_shift,
    );
    *FRAMEBUFFER.lock() = Some(writer);
}
