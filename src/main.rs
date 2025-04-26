mod io;

use crate::io::{Out, Color, Vector2};

const WATER: char = '~';
const SHORE: char = '#';
// const SHIP_HORIZONTAL_LEFT: char = '◀';
// const SHIP_HORIZONTAL_RIGHT: char = '▶';
// const SHIP_VERTICAL_TOP: char = '▲';
// const SHIP_VERTICAL_BOTTOM: char = '▼';
const SHIP_BODY: char = '■';

const MAP_SIZE: u16 = 32;
const ROOM_SIZE_X: u16 = 7;
const ROOM_SIZE_Y: u16 = 5;


fn draw_room_at(out: &mut Out, position: &Vector2)
{
    out.set_background_color(Color::Yellow);
    out.go_to_position(position);

    for _ in 0..ROOM_SIZE_X
    {
        out.draw(' ');
    }

    for i in 1..ROOM_SIZE_Y - 1
    {
        out.go_to_position(&Vector2::new(position.x, position.y + i));
        out.draw(' ');

        out.go_to_position(&Vector2::new(position.x + ROOM_SIZE_X - 1, position.y + i));
        out.draw(' ');
    }

    out.go_to_position(&Vector2::new(position.x, position.y + ROOM_SIZE_Y - 1));

    for _ in 0..ROOM_SIZE_X
    {
        out.draw(' ');
    }
}

fn draw_map(out: &mut Out)
{
    out.clear_all();

    out.set_background_color(Color::Yellow);

    for _i in 0..MAP_SIZE
    {
        out.draw(' ');
    }

    for i in 1..MAP_SIZE
    {
        let y: u16 = i + 1;
        out.go_to_position(&Vector2::new(1, y));

        out.draw(' ');

        out.set_background_color(Color::Reset);
        for _j in 0..MAP_SIZE - 2
        {
            out.draw(WATER);
        }

        out.set_background_color(Color::Yellow);
        out.draw(' ');
    }

    out.go_to_position(&Vector2::new(1, MAP_SIZE));

    for _i in 0..MAP_SIZE
    {
        out.draw(' ');
    }

    out.set_background_color(Color::Reset);
}

// fn draw(frame: &mut Frame)
// {
//     let circle = Circle {
//         x: 5.0,
//         y: 5.0,
//         radius: 8.0,
//         color: Color::Yellow,
//     };
//
//     let map = Canvas::default()
//         .block(Block::bordered().title("World"))
//         .paint(|ctx| {
//             ctx.draw(&circle);
//         });
//
//     let text: Text = Text::raw("Hello World!");
//     frame.render_widget(map, frame.area());
// }

fn main()
{
    const HORIZONTAL_MULTIPLIER: u16 = 2;
    let mut out: Out = Out::new(HORIZONTAL_MULTIPLIER);

    let terminal_size = termion::terminal_size().unwrap();

    println!("terminal size: {:?}", terminal_size);
    println!("terminal scaled size x: {:?}", terminal_size.0 / HORIZONTAL_MULTIPLIER);

    // let stdin: Stdin = stdin();

    draw_map(&mut out);

    out.set_background_color(Color::Red);
    out.draw_at(' ', &Vector2::new(3, 4));
    out.draw_at(' ', &Vector2::new(12, 6));

    draw_room_at(&mut out, &Vector2::new(40, 1));
    draw_room_at(&mut out, &Vector2::new(12, 4));

    out.draw_at(' ', &Vector2::new(terminal_size.0 / HORIZONTAL_MULTIPLIER, terminal_size.1));

    out.set_background_color(Color::Green);
    let test = String::from("   \n   \n   ");

    out.draw_string_at(&test, &Vector2::new(19, 10));

    out.set_background_color(Color::Reset);

    out.clean_up();
}
