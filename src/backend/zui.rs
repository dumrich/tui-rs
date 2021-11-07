use crate::{
    backend::Backend,
    buffer::Cell,
    layout::Rect,
    style::{Color, Modifier},
};
use libzui::style::{self, set, Style};
use libzui::term;
use libzui::term::clear::TClear;
use libzui::term::cursor::TCursor;
use std::fmt;
use std::io::{self, Write};

pub struct ZuiBackend<'b> {
    zui: term::Terminal<'b, io::Stdout>,
}

impl<'b> ZuiBackend<'b> {
    pub fn new(stdout: &'b mut io::Stdout) -> ZuiBackend<'b> {
        let term = term::Terminal::new(stdout).unwrap();
        ZuiBackend { zui: term }
    }
}

impl<'b> Write for ZuiBackend<'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.zui.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.zui.stdout.flush()
    }
}

impl<'b> Backend for ZuiBackend<'b> {
    fn clear(&mut self) -> io::Result<()> {
        self.zui.clear_screen()
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.zui.hide_cursor()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.zui.show_cursor()
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.zui.get_cursor()
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        self.zui.set_cursor_to(x, y)
    }

    fn size(&self) -> io::Result<Rect> {
        let terminal = self.zui.rel_size;
        Ok(Rect::new(0, 0, terminal.0, terminal.1))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.zui.stdout.flush()
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        use std::fmt::Write;

        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut fg = Color::Reset;
        let mut bg = Color::Reset;
        let mut modifier = Modifier::empty();
        let mut last_pos: Option<(u16, u16)> = None;
        for (x, y, cell) in content {
            // Move the cursor if the previous location was not (x - 1, y)
            if !matches!(last_pos, Some(p) if x == p.0 + 1 && y == p.1) {
                write!(string, "\u{001b}[{};{}H", y + 1, x + 1).unwrap();
            }
            last_pos = Some((x, y));
            if cell.modifier != modifier {
                write!(
                    string,
                    "{}",
                    ModifierDiff {
                        from: modifier,
                        to: cell.modifier
                    }
                )
                .unwrap();
                modifier = cell.modifier;
            }
            if cell.fg != fg {
                write!(string, "{}", Fg(cell.fg)).unwrap();
                fg = cell.fg;
            }
            if cell.bg != bg {
                write!(string, "{}", Bg(cell.bg)).unwrap();
                bg = cell.bg;
            }
            string.push_str(&cell.symbol);
        }
        write!(
            self.zui.stdout,
            "{}{}{}{}",
            string,
            Fg(Color::Reset),
            Bg(Color::Reset),
            libzui::style::set(libzui::style::Style::Reset),
        )
    }
}

struct Fg(Color);

struct Bg(Color);

struct ModifierDiff {
    from: Modifier,
    to: Modifier,
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use libzui::color::fg;
        use libzui::color::Color as ZuiColor;

        match self.0 {
            Color::Reset => write!(f, "{}", fg(ZuiColor::Reset)),
            Color::Black => write!(f, "{}", fg(ZuiColor::Black)),
            Color::Red => write!(f, "{}", fg(ZuiColor::Red)),
            Color::Green => write!(f, "{}", fg(ZuiColor::Green)),
            Color::Yellow => write!(f, "{}", fg(ZuiColor::Yellow)),
            Color::Blue => write!(f, "{}", fg(ZuiColor::Blue)),
            Color::Magenta => write!(f, "{}", fg(ZuiColor::Purple)),
            Color::Cyan => write!(f, "{}", fg(ZuiColor::Cyan)),
            Color::Gray => write!(f, "{}", fg(ZuiColor::Black)),
            Color::DarkGray => write!(f, "{}", fg(ZuiColor::Black)),
            Color::LightRed => write!(f, "{}", fg(ZuiColor::RedLight)),
            Color::LightGreen => write!(f, "{}", fg(ZuiColor::GreenLight)),
            Color::LightBlue => write!(f, "{}", fg(ZuiColor::BlueLight)),
            Color::LightYellow => write!(f, "{}", fg(ZuiColor::YellowLight)),
            Color::LightMagenta => write!(f, "{}", fg(ZuiColor::PurpleLight)),
            Color::LightCyan => write!(f, "{}", fg(ZuiColor::CyanLight)),
            Color::White => write!(f, "{}", fg(ZuiColor::White)),
            Color::Indexed(i) => write!(f, "{}", fg(ZuiColor::Reset)), //TODO: Fix
            Color::Rgb(r, g, b) => write!(f, "{}", fg(ZuiColor::RGB(r.into(), g.into(), b.into()))),
        }
    }
}
impl std::fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use libzui::color::bg;
        use libzui::color::Color as ZuiColor;

        match self.0 {
            Color::Reset => write!(f, "{}", bg(ZuiColor::Reset)),
            Color::Black => write!(f, "{}", bg(ZuiColor::Black)),
            Color::Red => write!(f, "{}", bg(ZuiColor::Red)),
            Color::Green => write!(f, "{}", bg(ZuiColor::Green)),
            Color::Yellow => write!(f, "{}", bg(ZuiColor::Yellow)),
            Color::Blue => write!(f, "{}", bg(ZuiColor::Blue)),
            Color::Magenta => write!(f, "{}", bg(ZuiColor::Purple)),
            Color::Cyan => write!(f, "{}", bg(ZuiColor::Cyan)),
            Color::Gray => write!(f, "{}", bg(ZuiColor::Black)),
            Color::DarkGray => write!(f, "{}", bg(ZuiColor::Black)),
            Color::LightRed => write!(f, "{}", bg(ZuiColor::RedLight)),
            Color::LightGreen => write!(f, "{}", bg(ZuiColor::GreenLight)),
            Color::LightBlue => write!(f, "{}", bg(ZuiColor::BlueLight)),
            Color::LightYellow => write!(f, "{}", bg(ZuiColor::YellowLight)),
            Color::LightMagenta => write!(f, "{}", bg(ZuiColor::PurpleLight)),
            Color::LightCyan => write!(f, "{}", bg(ZuiColor::CyanLight)),
            Color::White => write!(f, "{}", bg(ZuiColor::White)),
            Color::Indexed(i) => write!(f, "{}", bg(ZuiColor::Reset)), //TODO: Fix
            Color::Rgb(r, g, b) => write!(f, "{}", bg(ZuiColor::RGB(r.into(), g.into(), b.into()))),
        }
    }
}

impl fmt::Display for ModifierDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let remove = self.from - self.to;
        if remove.contains(Modifier::REVERSED) {
            write!(f, "{}", set(Style::NoReverse))?;
        }
        if remove.contains(Modifier::BOLD) {
            write!(f, "{}", set(Style::NoDim))?;

            if self.to.contains(Modifier::DIM) {
                write!(f, "{}", set(Style::Dim))?;
            }
        }
        if remove.contains(Modifier::ITALIC) {
            write!(f, "{}", set(Style::NoItalic))?;
        }
        if remove.contains(Modifier::UNDERLINED) {
            write!(f, "{}", set(Style::NoUnderline))?;
        }
        if remove.contains(Modifier::DIM) {
            write!(f, "{}", set(Style::NoDim))?;

            if self.to.contains(Modifier::BOLD) {
                write!(f, "{}", set(Style::Bold))?;
            }
        }
        if remove.contains(Modifier::CROSSED_OUT) {
            write!(f, "{}", set(Style::NoStrike))?;
        }
        if remove.contains(Modifier::SLOW_BLINK) || remove.contains(Modifier::RAPID_BLINK) {
            write!(f, "{}", set(Style::NoBlinking))?;
        }

        let add = self.to - self.from;
        if add.contains(Modifier::REVERSED) {
            write!(f, "{}", set(Style::Reverse))?;
        }
        if add.contains(Modifier::BOLD) {
            write!(f, "{}", set(Style::Bold))?;
        }
        if add.contains(Modifier::ITALIC) {
            write!(f, "{}", set(Style::Italic))?;
        }
        if add.contains(Modifier::UNDERLINED) {
            write!(f, "{}", set(Style::Underline))?;
        }
        if add.contains(Modifier::DIM) {
            write!(f, "{}", set(Style::Dim))?;
        }
        if add.contains(Modifier::CROSSED_OUT) {
            write!(f, "{}", set(Style::Strike))?;
        }
        if add.contains(Modifier::SLOW_BLINK) || add.contains(Modifier::RAPID_BLINK) {
            write!(f, "{}", set(Style::Blinking))?;
        }

        Ok(())
    }
}
