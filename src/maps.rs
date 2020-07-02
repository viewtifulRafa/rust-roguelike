use specs::prelude::*;
use rltk::{Rltk, RGB, Point, Algorithm2D, BaseMap };
use std::cmp::{max,min};
pub use crate::rect::*;
pub use crate::components::{FoV,Player};

#[derive(PartialEq, Clone, Copy)]
pub struct Tile {
    fg: RGB,
    bg:RGB,
    glyph: rltk::FontCharType
}


#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall(Tile),
    Floor(Tile)
}

fn WALL() -> TileType { 
    TileType::Wall( 
        Tile {
            fg: RGB::from_f32(0.,1.0,0.),
            bg: RGB::from_f32(0.,0.,0.),
            glyph: rltk::to_cp437('#')
        }
    )
}

fn FLOOR() -> TileType {
        TileType::Floor( Tile {
            fg: RGB::from_f32(0.5,0.5,0.5),
            bg: RGB::from_f32(0.,0.,0.),
            glyph: rltk::to_cp437('.')
        }
    )
}

const MIN_SIZE:i32 = 6;       
const MAX_SIZE:i32 = 10;       

pub struct Room {
    pos: Rectangle,
}

impl Room {
 
    pub fn new (x:i32, y:i32, width:i32, height: i32) -> Room {
        Room {
            pos: Rectangle::new(x,y,width,height)
        }
    }
 
    pub fn random_size(max_x:i32, max_y:i32) -> Room {
        let mut rng = rltk::RandomNumberGenerator::new();
        let width = rng.range(MIN_SIZE, MAX_SIZE);
        let height = rng.range(MIN_SIZE, MAX_SIZE);
        Room::new(
            rng.roll_dice(1,max_x-width-1)-1,
            rng.roll_dice(1,max_y-height-1)-1,
            width, 
            height
        )
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.pos.intersects(&other.pos)
    }
    pub fn center(&self) -> (i32,i32) {
        self.pos.center()
    }

}

const MAX_ROOMS:i32 = 30;

pub struct Map {
    pub width:  i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        if let TileType::Wall(_) = self.tiles[idx] {
            return true;
        }
        false
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl Map {

    pub fn new(width: i32, height: i32 ) -> Map {
        Map {
            width: width,
            height: height,
            tiles: vec![WALL(); (height * width) as usize],
            rooms: vec![]
        }
    }

    pub fn new_fully_random(width: i32, height: i32) -> Map {
        let mut map = Map::new(width, height);

        // Build a wall around
        for x in 0..map.width {
            map.set_tile_at(x,0,WALL());
            map.set_tile_at(x,height-1,WALL());
        };

        for y in 0..map.height {
            map.set_tile_at(0,y,WALL());
            map.set_tile_at(width-1,y,WALL());
        };

        // Random content inside
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1,map.width-1);
            let y = rng.roll_dice(1,map.height-1);
            if (x,y) != (map.width/2, map.height/2) {
                map.set_tile_at(x,y,WALL());
            };
        };

        map

    }

    pub fn new_random(width: i32, height: i32) -> Map {
        let mut map = Map::new(width, height);
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let room = Room::random_size(map.width, map.height);

            if map.try_to_add_room(room) {
            
                // Connect with previous room
                let n_rooms = map.rooms.len();
                if n_rooms > 1  {
                    let (x, y) = map.rooms[n_rooms-1].pos.center();
                    let (prev_x, prev_y) = map.rooms[n_rooms-2].pos.center();
                    if rng.range(0,2) == 1 {
                        map.add_horizontal_tunnel(prev_x,x,prev_y);
                        map.add_vertical_tunnel(prev_y,y,x);
                    } else {
                        map.add_vertical_tunnel(prev_y,y,prev_x);
                        map.add_horizontal_tunnel(prev_x,x,y);
                    }
                }
            }
        }

        map
    }
    
    /// Converts Tile coordinates to Tile index
    /// TODO: 80 Should be a parameter
    fn coord_to_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x ) as usize
    }

    pub fn set_tile_at(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.coord_to_idx(x,y);
        self.tiles[idx] = tile;
    }

    pub fn get_tile_at(&self, x: i32, y: i32) -> &TileType {
        let idx = self.coord_to_idx(x,y);
        &self.tiles[idx]
    }

    pub fn try_to_add_room (&mut self, room: Room) -> bool {
        for map_room in self.rooms.iter() {
            if room.intersects(&map_room){
                return false;
            }
        }
        self.add_room(room);
        true
    }

    pub fn add_room (&mut self, room: Room) {
        for y in room.pos.y0..=room.pos.y1 {
            for x in room.pos.x0..=room.pos.x1 {
                self.set_tile_at(x, y, FLOOR())
            }
        }
        self.rooms.push(room)
    }

    pub fn add_horizontal_tunnel(&mut self,x0:i32, x1:i32, y:i32) {
        for x in min(x0,x1)..=max(x0,x1) {
            self.set_tile_at(x, y, FLOOR())
        }
    }

    pub fn add_vertical_tunnel(&mut self,y0:i32, y1:i32, x:i32) {
        for y in min(y0,y1)..=max(y0,y1) {
            self.set_tile_at(x, y, FLOOR())
        }
        
    }

    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        match self.get_tile_at(x, y) {
            TileType::Wall(_) => true,
            _ => false,
        }
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        return x >= 0 && x <= self.width && y >= 0 && y <= self.height
    }

    pub fn draw (&self, ecs: &World, ctx: &mut Rltk) {
        let mut fovs = ecs.write_storage::<FoV>();
        let mut players = ecs.write_storage::<Player>();
        for (_player, fov) in (&mut players, &mut fovs).join() {
            let mut x = 0;
            let mut y = 0;
        
            for tile in &self.tiles {
                let pt = Point::new(x,y);
                if fov.visible_tiles.contains(&pt) {
                    match tile {
                        TileType::Floor(render) => {ctx.set(x,y,render.fg,render.bg,render.glyph)},
                        TileType::Wall(render)  => {ctx.set(x,y,render.fg,render.bg,render.glyph)},
                    }           
                }
                x+=1;
                if x > self.width-1 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}

