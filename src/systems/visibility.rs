use specs::prelude::*;
use rltk::{field_of_view,Point};
use super::super::{FoV, Position, Map, Player};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem{
    type SystemData = ( WriteExpect<'a, Map>, 
                        Entities<'a>,
                        WriteStorage<'a, FoV>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>);
    
    fn run (&mut self, data : Self::SystemData) {
        let (mut map, entities, mut fovs, positions, player) = data;
        for (ent, fov, pos) in (&entities, &mut fovs, &positions).join() {
            if fov.dirty {
                fov.dirty = false;
                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_tiles.retain( |p| map.is_inside(p.x,p.y) );

                let _p : Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible.iter_mut() { *t = false };
                    for v_point in fov.visible_tiles.iter() {
                        map.set_revealed(v_point, true);
                        map.set_visible(v_point, true);
                    }
                }
            }
        }
    }
}