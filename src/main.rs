mod io;

use crate::io::{Color, Out, Vector2};

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

fn draw_room_at(out: &mut Out, position: &Vector2, size: &Vector2)
{
    out.go_to_position(position);

    for _ in 0..size.x
    {
        out.draw(' ');
    }

    for i in 1..size.y - 1
    {
        out.go_to_position(&Vector2::new(position.x, position.y + i));
        out.draw(' ');

        out.go_to_position(&Vector2::new(position.x + size.x - 1, position.y + i));
        out.draw(' ');
    }

    out.go_to_position(&Vector2::new(position.x, position.y + size.y - 1));

    for _ in 0..size.x
    {
        out.draw(' ');
    }
}

fn draw_map(out: &mut Out, map_size: &Vector2)
{
    let border_char = '.';
    let position_char = '.';

    out.clear_all();

    for _ in 0..map_size.x
    {
        out.draw(border_char);
    }

    for i in 1..map_size.y
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

    out.go_to_position(&Vector2::new(1, map_size.y));

    for _ in 0..map_size.x
    {
        out.draw(border_char);
    }
}

fn main()
{
    // let stdin: Stdin = stdin();

    let horizontal_multiplier              = 2;
    let bottom_padding                     = 2;

    let (terminal_size_x, terminal_size_y) = termion::terminal_size().unwrap();
    let map_size                           = Vector2::new(terminal_size_x / horizontal_multiplier, terminal_size_y - bottom_padding);
    let rooms_layout                       = Vector2::new(9, 9);
    let room_size                          = Vector2::new(map_size.x / rooms_layout.x, map_size.y / rooms_layout.y );


    println!("terminal size: {:?}", (terminal_size_x, terminal_size_y));
    println!("map size: {}", map_size);
    println!("room size: {}", room_size);

    let mut out = Out::new(horizontal_multiplier);

    out.clear_all();

    draw_map(&mut out, &map_size);

    // out.set_background_color(Color::Red);
    // out.draw_at(' ', &Vector2::new(3, 4));
    // out.draw_at(' ', &Vector2::new(12, 6));

    // draw_room_at(&mut out, &Vector2::new(40, 1), &room_size);
    // draw_room_at(&mut out, &Vector2::new(12, 32), &room_size);


    out.set_background_color(Color::Gray);
    for y in 0..rooms_layout.x
    {
        for x in 0..rooms_layout.y
        {
            draw_room_at(
                &mut out,
                &Vector2::new(1 + room_size.x * x, 1 + room_size.y * y),
                &room_size,
            );
        }
    }
    out.set_background_color(Color::Reset);

    
    out.set_background_color(Color::Red);
    out.draw_at(' ', &Vector2::new(map_size.x, map_size.y));

    // let test = String::from("   \n   \n   ");
    // out.set_background_color(Color::Green);
    // out.draw_string_at(&test, &Vector2::new(19, 10));
    // out.set_background_color(Color::Reset);

    out.clean_up();
}
