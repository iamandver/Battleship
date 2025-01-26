use std::io::{stdout, Stdout, Write};
use std::ops::Deref;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};
use termion::cursor::DetectCursorPos;
use crate::{SpriteColor, Position, MAP_SIZE};
use crate::ship::Orientation;

pub trait Renderable
{
    fn get_position(&self) -> &Position;
    fn has_orientation(&self) -> bool;
    fn get_orientation(&self) -> &Orientation;
    fn get_sprite(&self) -> Vec<char>;
}

pub struct Out
{
    stdout: RawTerminal<Stdout>,
}

impl Out
{
    pub fn new() -> Out
    {
        Out
        {
            stdout: stdout().into_raw_mode().unwrap()
        }
    }

    pub fn clear_all(&mut self)
    {
        write!(self.stdout,
               "{}{}{}",
               clear::All,
               cursor::Goto(1, 1),
               cursor::Hide
        ).unwrap();
    }

    pub fn flush(&mut self)
    {
        self.stdout.flush().unwrap();
    }

    pub fn set_color(&mut self, color: SpriteColor)
    {
        let color: Box<dyn color::Color> = match color {
            SpriteColor::Reset  => {Box::new(color::Reset)}
            SpriteColor::Green  => {Box::new(color::Green)}
            SpriteColor::Red    => {Box::new(color::Red)}
            SpriteColor::Blue   => {Box::new(color::Blue)}
            SpriteColor::Yellow => {Box::new(color::Yellow)}
        };

        write!(self.stdout, "{}", color::Fg(color.deref())).unwrap();
    }

    pub fn go_to_position(&mut self, position: &Position)
    {
        write!(self.stdout, "{}", cursor::Goto((position.x * 2) - 1, position.y)).unwrap();
    }

    fn draw_vec_horizontally(&mut self, sprite_vec: Vec<char>)
    {
        sprite_vec.iter().for_each(|x: &char| { self.draw(*x); });
    }

    fn draw_vec_vertically(&mut self, sprite_vec: Vec<char>)
    {
        let cursor_position = self.stdout.cursor_pos().unwrap();
        let starting_position = Position::from(cursor_position);
        let mut current_position = starting_position.clone();

        sprite_vec.iter().for_each(|x: &char|
            {
                self.go_to_position(&current_position);
                self.draw(*x);
                current_position = Position { x: current_position.x, y: current_position.y + 1 };
            });

        self.go_to_position(&Position { x: starting_position.x + 1, y: starting_position.y });
    }

    pub fn draw(&mut self, sprite: char)
    {
        write!(self.stdout, "{} ", sprite).unwrap();
    }

    // fn draw_at(&mut self, character: char, position: Position)
    // {
    //     self.go_to_position(&position);
    //     self.draw(character);
    // }

    pub fn render(&mut self, sprite: &dyn Renderable)
    {
        self.go_to_position(sprite.get_position());

        // if !sprite.has_orientation()
        // {
        //     self.draw(sprite)
        // }

        match sprite.get_orientation() {
            Orientation::Horizontal => { self.draw_vec_horizontally(sprite.get_sprite()); },
            Orientation::Vertical   => { self.draw_vec_vertically(sprite.get_sprite()); },
        }
    }

    pub fn clean_up(&mut self)
    {
        self.flush();
        self.set_color(SpriteColor::Reset);
        self.go_to_position(&Position { x: MAP_SIZE, y: MAP_SIZE } );
        write!(self.stdout, "{}", cursor::Show).unwrap();
    }
}
