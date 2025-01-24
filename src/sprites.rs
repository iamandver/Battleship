pub static WATER: char = '~';
pub static SHORE: char = '#';
pub static SHIP_HORIZONTAL_LEFT:  char = '◀';
pub static SHIP_HORIZONTAL_RIGHT: char = '▶';
pub static SHIP_VERTICAL_TOP:     char = '▲';
pub static SHIP_VERTICAL_BOTTOM:  char = '▼';
pub static SHIP_BODY:             char = '■';

pub enum SpriteColor
{
    Reset,

    Green,
    Red,
    Blue,
    Yellow,
}

// pub struct Sprites
// {
//     shore: char,
//     water: char,
//     ship: char,
//     missile_miss: char,
//     missile_strike: char,
//     ships: Vec<ShipSprite>,
// }
//
// impl Sprites
// {
//     pub fn init() -> Sprites
//     {
//         Sprites
//         {
//             shore: '#',
//             water: '~',
//             ship: '■',
//             missile_miss: '○',
//             missile_strike: '✗',
//             ships: vec![
//                 ShipSprite
//                 {
//                     horizontal: vec!['◀', '■', '▶'],
//                     vertical: vec!['▲', '■', '▼']
//                 }],
//         }
//     }
// }