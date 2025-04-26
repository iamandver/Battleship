mod io;

use crate::io::{Color, Out, Vector2};

const WATER: char = '~';
// const SHORE: char = '#';
// const SHIP_HORIZONTAL_LEFT: char = '◀';
// const SHIP_HORIZONTAL_RIGHT: char = '▶';
// const SHIP_VERTICAL_TOP: char = '▲';
// const SHIP_VERTICAL_BOTTOM: char = '▼';
// const SHIP_BODY: char = '■';

// const MAP_SIZE: u16 = 32;
const ROOM_WIDTH: u16 = 7;
const ROOM_HEIGHT: u16 = 5;

struct DungeonFloor<'a>
{
    room_size: Vector2,
    rooms: Vec<Option<Room<'a>>>,
}

struct Room<'a>
{
    position: Vector2,
    neighbours: Vec<&'a Room<'a>>,
}

fn draw_room_at(out: &mut Out, position: &Vector2)
{
    out.set_background_color(Color::Yellow);
    out.go_to_position(position);

    for _ in 0..ROOM_WIDTH
    {
        out.draw(' ');
    }

    for i in 1..ROOM_HEIGHT - 1
    {
        out.go_to_position(&Vector2::new(position.x, position.y + i));
        out.draw(' ');

        out.go_to_position(&Vector2::new(position.x + ROOM_WIDTH - 1, position.y + i));
        out.draw(' ');
    }

    out.go_to_position(&Vector2::new(position.x, position.y + ROOM_HEIGHT - 1));

    for _ in 0..ROOM_WIDTH
    {
        out.draw(' ');
    }
}

fn draw_map(out: &mut Out, map_size: &Vector2)
{
    let bottom_padding = 4;
    let border_char = '*';
    let position_char = '.';

    out.clear_all();

    for _ in 0..map_size.x
    {
        out.draw(border_char);
    }

    for i in 1..map_size.y - bottom_padding
    {
        let y: u16 = i + 1;
        out.go_to_position(&Vector2::new(1, y));

        out.draw(border_char);

        out.set_foreground_color(Color::Gray);
        for _ in 0..map_size.x - 2
        {
            out.draw(position_char);
        }
        out.set_foreground_color(Color::Reset);

        out.draw(border_char);
    }

    out.go_to_position(&Vector2::new(1, map_size.y - bottom_padding));

    for _ in 0..map_size.x
    {
        out.draw(border_char);
    }
}

fn main()
{
    let horizontal_multiplier: u16 = 2;
    let mut out: Out = Out::new(horizontal_multiplier);

    let (terminal_size_x, terminal_size_y): (u16, u16) = termion::terminal_size().unwrap();
    let (map_size_x, map_size_y): (u16, u16) = (terminal_size_x / horizontal_multiplier, terminal_size_y);

    println!("terminal size: {:?}", (terminal_size_x, terminal_size_y));
    println!("terminal scaled size x: {:?}", (map_size_x, map_size_y));

    // let stdin: Stdin = stdin();

    out.clear_all();

    draw_map(&mut out, &Vector2::new(map_size_x, map_size_y));

    out.set_background_color(Color::Red);
    out.draw_at(' ', &Vector2::new(3, 4));
    out.draw_at(' ', &Vector2::new(12, 6));

    draw_room_at(&mut out, &Vector2::new(40, 1));
    draw_room_at(&mut out, &Vector2::new(12, 4));

    out.draw_at(' ', &Vector2::new(terminal_size_x / horizontal_multiplier, terminal_size_y));

    out.set_background_color(Color::Green);
    let test = String::from("   \n   \n   ");

    out.draw_string_at(&test, &Vector2::new(19, 10));

    out.set_background_color(Color::Reset);

    out.clean_up();
}
