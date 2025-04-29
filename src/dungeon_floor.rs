use crate::io::Vector2;

pub struct DungeonFloor<'a>
{
    pub floor_size: Vector2,
    pub room_size: Vector2,
    pub rooms: Vec<Option<Room<'a>>>,
}

impl DungeonFloor<'_>
{
    pub fn new(floor_size: &Vector2, room_size: &Vector2 ) -> Self
    {
        let     floor_size               = Vector2::new( floor_size.x, floor_size.y );
        let     room_size                = Vector2::new( room_size.x,  room_size.y  );

        let mut rooms: Vec<Option<Room>> = Vec::new();

        let     rooms_count              = ( floor_size.x * floor_size.y ) as usize;
        rooms.reserve_exact(rooms_count);

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
            return Err( String::from( "Index our of range." ) );
        }

        let x = ( ( index % self.floor_size.x as usize ) + 1 ) as u16;
        let y = ( ( index / self.floor_size.y as usize ) + 1 ) as u16;

        Ok( Vector2::new( x, y ) )
    }

    pub fn add_room_at(&mut self, room_coordinate: Vector2 )
    {
        let room_index = self.convert_room_coordinate_to_index( &room_coordinate );
        let room_index = match room_index
        {
            Ok( i ) => i,
            Err( e ) => panic!("{}", e)
        };

        if self.rooms.get( room_index ).unwrap().is_some()
        {
            panic!( "Room at {} is occupied.", room_index );
        }

        let new_room = Room::new( room_coordinate );

        self.rooms[ room_index ] = Some( new_room );
    }
}

pub struct Room<'a>
{
    pub coordinate: Vector2,
    neighbours: Vec<&'a Room<'a>>,
}

impl Room<'_> {
    fn new(coordinate: Vector2 ) -> Self
    {
        // todo: create function 'look_for_neighbours()'
        let neighbours = Vec::new();

        Self { coordinate, neighbours }
    }
}