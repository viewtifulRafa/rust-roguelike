use specs::prelude::*;
use rltk::{field_of_view,Point};
use super::super::{FoV, Position, Map};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem{
    type SystemData = ( ReadExpect<'a, Map>, 
                        WriteStorage<'a, FoV>,
                        WriteStorage<'a, Position>);
    
    fn run (&mut self, data : Self::SystemData) {
        let (map, mut fovs, positions) = data;
        for (fov, pos) in (&mut fovs, &positions).join() {
            fov.visible_tiles.clear();
            fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
            fov.visible_tiles.retain( |p| map.is_inside(p.x,p.y) );
        }
    }
}