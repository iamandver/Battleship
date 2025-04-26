use std::fmt::{Display, Formatter};
use std::io::{stdout, Stdout, Write};
use std::ops::Deref;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};

pub enum Color
{
    Reset,

    Green,
    Red,
    Blue,
    Yellow,
    Gray,
}

impl Into<Box<dyn color::Color>> for Color
{
    fn into(self) -> Box<dyn color::Color>
    {
        match self
        {
            Color::Reset => Box::new(color::Reset),
            Color::Green => Box::new(color::Green),
            Color::Red => Box::new(color::Red),
            Color::Blue => Box::new(color::Blue),
            Color::Yellow => Box::new(color::Yellow),
            Color::Gray => Box::new(color::LightBlack)
        }
    }
}

pub struct Vector2
{
    pub x: u16,
    pub y: u16,
}

impl Display for Vector2
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "( {}, {} )", self.x, self.y)
    }
}

impl Vector2
{
    pub(crate) fn new(x: u16, y: u16) -> Self
    {
        Vector2 { x, y }
    }
}

pub struct Out
{
    stdout: RawTerminal<Stdout>,
    horizontal_multiplier: u16,
}

impl Out
{
    pub fn new(horizontal_multiplier: u16) -> Out
    {
        let mut out = Out {
            stdout: stdout().into_raw_mode().unwrap(),
            horizontal_multiplier,
        };

        out.flush();

        out
    }

    pub fn clear_all(&mut self)
    {
        write!(self.stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();
    }

    pub fn flush(&mut self)
    {
        self.stdout.flush().unwrap();
    }

    pub fn set_foreground_color(&mut self, color: Color)
    {
        let color: Box<dyn color::Color> = color.into();

        write!(self.stdout, "{}", color::Fg(color.deref())).unwrap();
    }

    pub fn set_background_color(&mut self, color: Color)
    {
        let color: Box<dyn color::Color> = color.into();

        write!(self.stdout, "{}", color::Bg(color.deref())).unwrap();
    }

    pub fn go_to_position(&mut self, position: &Vector2)
    {
        write!(
            self.stdout,
            "{}",
            cursor::Goto(1 + ((position.x - 1) * self.horizontal_multiplier), position.y)
        )
        .unwrap();
    }

    pub fn draw(&mut self, sprite: char)
    {
        write!(self.stdout, "{}", sprite).unwrap();

        for _ in 1..self.horizontal_multiplier
        {
            write!(self.stdout, " ").unwrap();
        }
    }

    pub fn draw_at(&mut self, character: char, position: &Vector2)
    {
        self.go_to_position(position);
        self.draw(character);
    }

    pub fn draw_string_at(&mut self, string: &str, position: &Vector2)
    {
        let mut position_y = position.y;
        for line in string.lines()
        {
            let current_position = Vector2::new(position.x, position_y);
            self.go_to_position(&current_position);

            for ch in line.chars()
            {
                self.draw(ch);
            }

            position_y += 1;
        }
    }

    pub fn clean_up(&mut self)
    {
        let (_, terminal_size_y) = termion::terminal_size().unwrap();

        self.flush();
        self.set_foreground_color(Color::Reset);
        self.go_to_position(&Vector2 {
            x: 1,
            y: terminal_size_y,
        });
        write!(self.stdout, "{}", cursor::Show).unwrap();
    }
}
