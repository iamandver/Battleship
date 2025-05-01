use crate::io::Vector2;

enum Direction
{
    Up,
    Down,
    Left,
    Right
}

pub struct DungeonLevel<'a>
{
    pub floor_size: Vector2,
    pub room_size: Vector2,
    pub rooms: Vec<Option<Room<'a>>>
}

impl<'a> DungeonLevel<'a>
{
    pub fn new( floor_size: &Vector2, room_size: &Vector2 ) -> Self
    {
        let     floor_size               = Vector2::new( floor_size.x, floor_size.y );
        let     room_size                = Vector2::new( room_size.x,  room_size.y  );

        let mut rooms: Vec<Option<Room>> = Vec::new();

        let     rooms_count              = ( floor_size.x * floor_size.y ) as usize;

        rooms.reserve_exact( rooms_count );

        for _ in 0..rooms_count
        {
            rooms.push( None );
        }

        Self { floor_size, room_size, rooms }
    }

    fn convert_room_coordinate_to_index( &self, room_coordinate: &Vector2 ) -> Result<usize, String>
    {
        let valid_coord_range = ( 1..=self.floor_size.x, 1..=self.floor_size.y );

        if !valid_coord_range.0.contains( &room_coordinate.x ) || !valid_coord_range.1.contains( &room_coordinate.x )
        {
             return Err(String::from( "Coordinate out of range." ) );
        }

        Ok( ( room_coordinate.x + ( self.floor_size.x * ( room_coordinate.y - 1 ) ) - 1 ) as usize )
    }

    pub fn convert_index_to_room_coordinate( &self, index: usize ) -> Result<Vector2, String>
    {
        let valid_index_range = 0..self.rooms.len();

        if !valid_index_range.contains( &index )
        {
            return Err( String::from( "Index out of range." ) );
        }
    
        if let Ok( index ) = u16::try_from( index )
        {
            let y = ( index / self.floor_size.y ) + 1;
            let x = ( index % self.floor_size.x ) + 1;
    
            return Ok( Vector2::new( x, y ) );
        }
        
        Err( String::from( "Failed to convert index of type 'usize' to 'u16'." ) )
    }

    pub fn create_room_at( &mut self, room_coordinate: Vector2 )
    {
        let room = Room::new( room_coordinate );

        self.add_room( room );
    }

    pub fn add_room( &mut self, room: Room<'a> )
    {
        let room_index = self.convert_room_coordinate_to_index( &room.coordinate );
        let room_index = match room_index
        {
            Ok( i ) => i,
            Err( e ) => panic!( "{}", e )
        };

        assert!(!self.rooms.get( room_index ).unwrap().is_some(),  "Room at {} is occupied.", room_index );

        self.rooms[ room_index ] = Some( room );
    }

    fn get_index_of_neighbour( &self, index: usize, at: &Direction ) -> Option<usize>
    {
        if let Ok( index_as_u16 ) = u16::try_from( index )
        {
            let result = ( match at
            {
                Direction::Up    => ( index_as_u16 ) - self.floor_size.x,
                Direction::Down  => ( index_as_u16 ) + self.floor_size.x,
                Direction::Left  => ( index_as_u16 ) - 1,
                Direction::Right => ( index_as_u16 ) + 1
            } ) as usize;

            let valid_range = 0..self.rooms.len();

            if valid_range.contains( &result )
            {
                return None;
            }

            return Some( result );
        }
        
        None
    }
}

pub struct Room<'a>
{
    pub coordinate: Vector2,
    neighbours: Vec<&'a Room<'a>>,
}

impl Room<'_> {
    pub fn new( coordinate: Vector2 ) -> Self
    {
        // todo: create function 'look_for_neighbours()'
        let neighbours = Vec::new();

        Self { coordinate, neighbours }
    }
}