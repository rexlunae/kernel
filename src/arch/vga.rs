// Copyright 2015 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::ptr::Unique;
use core::fmt;
use spin::Mutex;
use term::{Terminal, Result, Attr, color};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    fg: VGAColor::White,
    bg: VGAColor::Black,
    color_code: ColorCode::new(VGAColor::LightGreen, VGAColor::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            $crate::arch::vga::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

pub unsafe fn print_error(fmt: fmt::Arguments) {
    use core::fmt::Write;

    let mut writer = Writer {
        column_position: 0,
		fg: VGAColor::White,
		bg: VGAColor::Black,
        color_code: ColorCode::new(VGAColor::Red, VGAColor::Black),
        buffer: Unique::new(0xb8000 as *mut _),
    };
    writer.new_line();
    writer.write_fmt(fmt);
}



// If only this were simple...
impl VGAColor {

/*
    // This really is only a rough approximation.
    const vga_to_ansi = [
        color::BLACK,
        color::BLUE,
        color::GREEN,
        color::CYAN,
        color::RED,
        color::MAGENTA,
        color::BLACK,       // There is no brown in ANSI
        color::WHITE,       // Instead of gray
        color::BLACK,       // Instead of dark gray
        color::BRIGHT_BLUE,
        color::BRIGHT_GREEN,
        color::BRIGHT_CYAN,
        color::BRIGHT_RED,
        color::BRIGHT_RED,  // Instead of pink
        color::YELLOW,
        color::BRIGHT_WHITE,
    ];

    fn to_ansi(self) -> color::Color {
        vga_to_ansi[self as usize]
    }

    fn from_ansi(color::Color) -> &mut Self {
        for (i,color) in vga_to_ansi.enumerate() {
            if color == color::Color
                return i as Self
        }
    }
*/
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    fg: VGAColor,
    bg: VGAColor,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            let buffer = self.buffer();
            buffer.chars[row] = buffer.chars[row + 1]
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer().chars[row] = [blank; BUFFER_WIDTH];
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}


// Note that color::Color is from the term module.
impl Terminal for Writer {
    type Output= Writer;

    fn fg(&mut self, color: color::Color) -> Result<()> {
		self.fg = VGAColor::White;
        Ok(())
    }

    fn bg(&mut self, color: color::Color) -> Result<()> {
		self.bg = VGAColor::Black;
        Ok(())
    }

    fn attr(&mut self, attr: Attr) -> Result<()> {
        Ok(())
    }

    fn cursor_up(&mut self) -> Result<()> {
        Ok(())
    }

    fn delete_line(&mut self) -> Result<()> {
		self.clear_row(0);
        Ok(())
    }

    fn carriage_return(&mut self) -> Result<()> {
		self.new_line();
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_ref(&self) -> &Self::Output {
		self.get_ref()

    }

    fn get_mut(&mut self) -> &mut Self::Output {
		self.get_mut()
    }

    fn into_inner(self) -> Self::Output where Self: Sized {
		self
    }

    fn supports_attr(&self, attr: Attr) -> bool {
        true
    }

    fn supports_reset(&self) -> bool {
        true
    }

    fn supports_color(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: VGAColor, background: VGAColor) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

