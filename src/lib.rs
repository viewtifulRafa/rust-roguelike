pub mod maps;
pub mod rect;
pub mod components;
pub mod systems;

use rltk::{Rltk, GameState,RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp;

pub use crate::maps::*;
pub use crate::components::*;
pub use crate::systems::visibility::VisibilitySystem;

//use web_sys::console;


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut fovs = ecs.write_storage::<FoV>();

    let map = ecs.fetch::<Map>();
    for (_player,pos,fov) in (&mut players, &mut positions, &mut fovs).join() {
        let x = cmp::min(79, cmp::max(0, pos.x + delta_x));
        let y = cmp::min(79, cmp::max(0, pos.y + delta_y));
        if ! map.is_solid(x,y) {
            pos.x = x;
            pos.y = y;
            fov.dirty = true;
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs ),
            
            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L=> try_move_player(1, 0, &mut gs.ecs ),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K   => try_move_player(0, -1, &mut gs.ecs ),
            
            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs ),
            
            _ => {}
        },
    }
}

struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos,render) in (&positions, &renderables).join(){
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

pub extern fn run() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };
    
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<FoV>();
    let map = Map::new_random(80,50);
    let (player_x,player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(FoV{visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();
        
    rltk::main_loop(context, gs)
}


// This is like the `main` function, except for JavaScript.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    // Run game
    run();

    Ok(())
}

