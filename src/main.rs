mod io;
mod dungeon_floor;

use std::cmp::min;
use dungeon_floor::{DungeonFloor, Room};
use crate::io::{Color, Out, Vector2};

fn draw_room_at( out: &mut Out, position: &Vector2, size: &Vector2, color: Option<Color> )
{
    let mut color_was_modified = false;

    if let Some( color ) = color
    {
        out.set_background_color( color );
        color_was_modified = true;
    }

    out.go_to_position( position );

    for _ in 0..size.x
    {
        out.draw(' ');
    }

    for i in 1..size.y - 1
    {
        out.go_to_position( &Vector2::new( position.x, position.y + i ) );
        out.draw( ' ' );

        out.go_to_position( &Vector2::new( position.x + size.x - 1, position.y + i ) );
        out.draw( ' ' );
    }

    out.go_to_position( &Vector2::new( position.x, position.y + size.y - 1 ) );

    for _ in 0..size.x
    {
        out.draw( ' ' );
    }

    if color_was_modified
    {
        out.set_background_color( Color::Reset )
    }
}

fn draw_overlay( out: &mut Out, map_size: &Vector2 )
{
    out.set_foreground_color( Color::Gray );

    let border_char   = '.';
    let position_char = '.';

    out.clear_all();

    for _ in 0..map_size.x
    {
        out.draw( border_char );
    }

    for i in 1..map_size.y
    {
        let y: u16 = i + 1;
        out.go_to_position( &Vector2::new( 1, y ) );

        out.draw( border_char );

        for _ in 0..map_size.x - 2
        {
            out.draw( position_char );
        }

        out.draw( border_char );
    }

    out.go_to_position( &Vector2::new( 1, map_size.y ) );

    for _ in 0..map_size.x
    {
        out.draw( border_char );
    }

    out.set_foreground_color( Color::Reset );
}

fn convert_room_coordinate_to_screen_position( room_coordinate: &Vector2, room_size: &Vector2 ) -> Vector2
{
    let x = 1 + room_size.x * ( room_coordinate.x - 1 );
    let y = 1 + room_size.y * ( room_coordinate.y - 1 );

    Vector2::new( x, y )
}

fn draw_floor( out: &mut Out, floor: &DungeonFloor )
{
    let room_size = &floor.room_size;

    for room_index in 0..floor.rooms.len()
    {
        let room: &Option<Room> = floor.rooms.get( room_index ).unwrap();

        let coordinate: &Vector2 = match room {
            Some(r) => &r.coordinate,
            None => &floor.convert_index_to_room_coordinate( room_index ).unwrap()
        };

        let color = match room {
            Some(_) => Some( Color::Yellow ),
            None => Some( Color::Gray )
        };

        let position = convert_room_coordinate_to_screen_position(coordinate, room_size );

        draw_room_at( out, &position, room_size, color );
    }
}

fn main()
{
    // let stdin: Stdin = stdin();

    let horizontal_multiplier = 2;
    let bottom_padding        = 2;
    let floor_size            = Vector2::new( 9, 9 );

    let terminal_size         = Vector2::from( termion::terminal_size().unwrap() );
    let map_size              = Vector2::new( terminal_size.x / horizontal_multiplier,
                                              terminal_size.y - bottom_padding);

    let room_size_x           = min( map_size.x / floor_size.x, 7 );
    let room_size_y           = min( map_size.y / floor_size.y, 5 );
    let room_size             = Vector2::new( room_size_x, room_size_y );

    println!( "terminal size: {}", terminal_size );
    println!( "map size: {}", map_size );
    println!( "room size: {}", room_size );

    let mut out = Out::new( horizontal_multiplier );

    out.clear_all();

    draw_overlay(&mut out, &map_size );

    // out.set_background_color(Color::Red);
    // out.draw_at(' ', &Vector2::new(3, 4));
    // out.draw_at(' ', &Vector2::new(12, 6));

    // draw_room_at(&mut out, &Vector2::new(40, 1), &room_size);
    // draw_room_at(&mut out, &Vector2::new(12, 32), &room_size);

    let mut dungeon_floor = DungeonFloor::new( &floor_size, &room_size );

    dungeon_floor.add_room_at( Vector2::new( 1, 1 ) );
    dungeon_floor.add_room_at( Vector2::new( 3, 4 ) );
    dungeon_floor.add_room_at( Vector2::new( 9, 4 ) );
    dungeon_floor.add_room_at( Vector2::new( 9, 8 ) );

    let center = Vector2::new( ( floor_size.x + 1 ) / 2, ( floor_size.x + 1 ) / 2 );

    dungeon_floor.add_room_at( center );

    draw_floor( &mut out , &dungeon_floor );

    // out.set_background_color( Color::Red );
    // out.draw_at( ' ', &Vector2::new( room_size.x, room_size.y ) );

    // let test = String::from("   \n   \n   ");
    // out.set_background_color(Color::Green);
    // out.draw_string_at(&test, &Vector2::new(19, 10));
    // out.set_background_color(Color::Reset);

    out.clean_up();
}
