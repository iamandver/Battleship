mod io;
mod ship;
mod sprites;

use crate::io::Out;
use crate::sprites::SpriteColor;
use std::fmt::{Display, Formatter};

const MAP_SIZE: u16 = 32;

// struct Game
// {
//     map_size: u8,
//     sprites: sprites::Sprites,
// }

struct Position
{
    x: u16,
    y: u16,
}

impl Display for Position
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "( {}, {} )", self.x, self.y)
    }
}

// used to convert terminal coordinates to world coordinates
impl From<(u16, u16)> for Position
{
    fn from(value: (u16, u16)) -> Self
    {
        Position {
            x: (value.0 + 1) / 2,
            y: value.1,
        }
    }
}

impl Clone for Position
{
    fn clone(&self) -> Self
    {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

// impl Position
// {
//     fn new(x: u16, y: u16) -> Self
//     {
//         Position { x, y }
//     }
// }

fn draw_map(out: &mut Out)
{
    out.clear_all();

    for _i in 0..MAP_SIZE
    {
        out.draw(sprites::SHORE);
    }

    for i in 1..MAP_SIZE
    {
        let y: u16 = i + 1;
        out.go_to_position(&Position { x: 1, y });

        out.draw(sprites::SHORE);

        for _j in 0..MAP_SIZE - 2
        {
            out.draw(sprites::WATER);
        }

        out.draw(sprites::SHORE);
    }

    out.go_to_position(&Position { x: 1, y: MAP_SIZE });

    for _i in 0..MAP_SIZE
    {
        out.draw(sprites::SHORE);
    }
}

fn main()
{
    let mut out: Out = Out::new();

    let terminal_size = termion::terminal_size().unwrap();
    println!("terminal size: {:?}", terminal_size);

    // let stdin: Stdin = stdin();
    out.flush();

    draw_map(&mut out);

    let ship = ship::Ship::new(ship::ShipSize::Three, ship::Orientation::Horizontal);
    out.set_color(SpriteColor::Red);
    out.render(&ship);

    out.set_color(SpriteColor::Reset);

    out.clean_up();
}
