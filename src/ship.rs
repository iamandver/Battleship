use crate::io::Renderable;
use crate::{sprites, SpriteColor, Position};

pub enum Orientation
{
    Horizontal,
    Vertical,
}

pub enum ShipSize
{
    Two,
    Three,
    Four,
    Five,
}

pub struct Ship
{
    sprite: Vec<ShipElement>,
    color: SpriteColor,
    orientation: Orientation,
    position: Position,
    health: u8,
}

impl Ship
{
    pub fn new(size: ShipSize, orientation: Orientation) -> Ship
    {
        let ship_length: u8 = match size {
            ShipSize::Two => { 2 }
            ShipSize::Three => { 3 }
            ShipSize::Four => { 4 }
            ShipSize::Five => { 5 }
        };

        let mut sprite: Vec<ShipElement> = Vec::with_capacity(size as usize);
        sprite.push(ShipElement { tile: sprites::SHIP_HORIZONTAL_LEFT, is_hit: false });
        sprite.push(ShipElement { tile: sprites::SHIP_BODY, is_hit: false });
        sprite.push(ShipElement { tile: sprites::SHIP_HORIZONTAL_RIGHT, is_hit: false });

        let position = Position { x: 3, y: 3 };
        let health: u8 = ship_length;

        Ship
        {
            sprite,
            color: SpriteColor::Yellow,
            orientation,
            position,
            health,
        }
    }

    fn sprite_tiles(&self) -> Vec<char>
    {
        for tile in &self.sprite.iter()
        {
            let test: ShipElement = tile;
        }

        vec![' ' as char]
        // let test = &self.sprite.iter().for_each(|tile|{}).collect::<Vec<char>>();
    }
}

impl Renderable for Ship
{
    fn get_position(&self) -> &Position {
        &self.position
    }

    fn has_orientation(&self) -> bool {
        true
    }

    fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }

    fn get_sprite(&self) -> &Vec<char> {
        &self.sprite_tiles()
    }
}

struct ShipElement
{
    tile: char,
    is_hit: bool,
}