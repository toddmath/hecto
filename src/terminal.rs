use crate::Position;

use anyhow::Result;
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use std::io::{self, Write};

pub struct Size {
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl From<(u16, u16)> for Size {
    fn from(size: (u16, u16)) -> Self {
        Size {
            width: size.0,
            height: size.1.saturating_sub(2),
        }
    }
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub(crate) fn new() -> Result<Self> {
        let size = Size::from(termion::terminal_size()?);
        Ok(Self {
            size,
            _stdout: io::stdout().into_raw_mode()?,
        })
    }

    pub(crate) const fn size(&self) -> &Size {
        &self.size
    }

    pub(crate) fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub(crate) fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub(crate) fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub(crate) fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub(crate) fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }

    pub(crate) fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
