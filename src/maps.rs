use rltk::{Rltk, RGB };


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

pub struct Map {
    width:  i32,
    height: i32,
    tiles: Vec<TileType>
}

impl Map {

    pub fn new(width: i32, height: i32 ) -> Map {
        Map {
            width: width,
            height: height,
            tiles: vec![FLOOR(); (height * width) as usize],
        }
    }

    pub fn new_random(width: i32, height: i32) -> Map {
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

    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        match self.get_tile_at(x, y) {
            TileType::Wall(_) => true,
            _ => false,
        }
    }

    pub fn draw (&self, ctx: &mut Rltk) {
        let mut x = 0;
        let mut y = 0;
        
        for tile in &self.tiles {
            match tile {
                TileType::Floor(render) => {ctx.set(x,y,render.fg,render.bg,render.glyph)},
                TileType::Wall(render)  => {ctx.set(x,y,render.fg,render.bg,render.glyph)},
            }           
        
            x+=1;
            if x > self.width-1 {
                x = 0;
                y += 1;
            }
        }

    }
}

