mod io;
mod sprites;
mod ship;

use std::fmt::{Display, Formatter};
use crate::sprites::SpriteColor;

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
        Position { x: (value.0 + 1) / 2, y: value.1 }
    }
}

impl Clone for Position
{
    fn clone(&self) -> Self
    {
        Position { x: self.x, y: self.y }
    }
}

// impl Position
// {
//     fn new(x: u16, y: u16) -> Self
//     {
//         Position { x, y }
//     }
// }

fn draw_map(out: &mut io::Out)
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
    // let sprites = sprites::Sprites::init();
    let mut out = io::Out::new();

    // let terminal_size = termion::terminal_size().unwrap();
    // println!("terminal size: {:?}", terminal_size);

    // let stdin: Stdin = stdin();
    out.flush();

    draw_map(&mut out);

    let ship = ship::Ship::new(ship::ShipSize::Three, ship::Orientation::Horizontal);
    out.set_color(SpriteColor::Red);
    out.render(&ship);

    // out.go_to_position(&Position { x: 3, y: 3 });
    // out.draw_vec_horizontally(&sprites::SPRITE_SHIP_SMALL_HORIZONTAL);
    // out.draw_vec_horizontally(&sprites::SPRITE_SHIP_SMALL_HORIZONTAL);
    // out.draw_vec_horizontally(&sprites::SPRITE_SHIP_SMALL_HORIZONTAL);

    // out.go_to_position(&Position { x: 6, y: 6 });
    // out.set_color(GameColor::Yellow);

    // out.draw_vec_vertically(&sprites::SPRITE_SHIP_SMALL_VERTICAL);
    // out.draw_vec_vertically(&sprites::SPRITE_SHIP_SMALL_VERTICAL);
    // out.draw_vec_vertically(&sprites::SPRITE_SHIP_SMALL_VERTICAL);

    out.draw(sprites::SHORE);


    // out.draw_sprite(sprites.ships[0].horizontal[0]);
    // out.draw_sprite(sprites.ships[0].horizontal[1]);
    // out.draw_sprite(sprites.ships[0].horizontal[1]);
    // out.draw_sprite(sprites.ships[0].horizontal[1]);
    // out.draw_sprite(sprites.ships[0].horizontal[2]);

    // for c in stdin.keys(){
    //     write!(
    //         stdout,
    //         "{}{}",
    //         termion::cursor::Goto(1, 1),
    //         termion::clear::CurrentLine
    //     )
    //     .unwrap();
    //
    //     // Print the key we type...
    //     match c.unwrap() {
    //         Key::Char('q') => break,
    //         Key::Char(c) => println!("{}", c),
    //         Key::Alt(c) => println!("Alt-{}", c),
    //         Key::Ctrl(c) => println!("Ctrl-{}", c),
    //         Key::Left => println!("<left>"),
    //         Key::Right => println!("<right>"),
    //         Key::Up => println!("<up>"),
    //         Key::Down => println!("<down>"),
    //         _ => println!("Other"),
    //     }
    //
    //     stdout.flush().unwrap();
    // }

    // Show the cursor again before we exit.

    out.clean_up();
}
