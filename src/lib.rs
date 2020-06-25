use rltk::{Rltk, GameState,RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp;
use specs_derive::Component;

use rltk::prelude::{BTerm, InitHints, BACKEND_INTERNAL};

//use web_sys::console;

// Components 

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {    
}

#[derive(Component, Debug)]
struct Player {
}

// System
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run (&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    
    for (_player,pos) in (&mut players, &mut positions).join() {
        pos.x = cmp::min(79, cmp::max(0, pos.x + delta_x));
        pos.y = cmp::min(79, cmp::max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left  => try_move_player(-1, 0, &mut gs.ecs ),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs ),
            VirtualKeyCode::Up    => try_move_player(0, -1, &mut gs.ecs ),
            VirtualKeyCode::Down  => try_move_player(0, 1, &mut gs.ecs ),
            _ => {}
        },
    }
}

struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
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

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
        
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position {x: i * 7, y: 20})
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }

    rltk::main_loop(context, gs)
}


// pub fn init_raw<S: ToString>(
//     width_pixels: u32,
//     height_pixels: u32,
//     _window_title: S,
//     _platform_hints: InitHints,
// ) -> Result<BTerm> {
//     use super::super::*;
//     use super::*;
//     use wasm_bindgen::JsCast;

//     let canvas = web_sys::window()
//         .unwrap()
//         .document()
//         .unwrap()
//         .get_element_by_id("canvas")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .unwrap();
//     canvas.set_width(width_pixels);
//     canvas.set_height(height_pixels);

//     bracket_terminal::wasm::bind_wasm_events(&canvas);

//     let webgl2_context = canvas
//         .get_context("webgl2")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::WebGl2RenderingContext>()
//         .unwrap();
//     webgl2_context
//         .get_extension("EXT_color_buffer_float")
//         .expect("Unable to add extensions");

//     let gl = glow::Context::from_webgl2_context(webgl2_context);

//     // Load our basic shaders
//     let mut shaders: Vec<Shader> = Vec::new();

//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::CONSOLE_WITH_BG_VS,
//         shader_strings::CONSOLE_WITH_BG_FS,
//     ));
//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::CONSOLE_NO_BG_VS,
//         shader_strings::CONSOLE_NO_BG_FS,
//     ));
//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::BACKING_VS,
//         shader_strings::BACKING_FS,
//     ));
//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::SCANLINES_VS,
//         shader_strings::SCANLINES_FS,
//     ));
//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::FANCY_CONSOLE_VS,
//         shader_strings::FANCY_CONSOLE_FS,
//     ));
//     shaders.push(Shader::new(
//         &gl,
//         shader_strings::SPRITE_CONSOLE_VS,
//         shader_strings::SPRITE_CONSOLE_FS,
//     ));

//     let quad_vao = setup_quad(&gl);
//     let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

//     let mut be = BACKEND.lock();
//     be.gl = Some(gl);
//     be.quad_vao = Some(quad_vao);
//     be.backing_buffer = Some(backing_fbo);

//     BACKEND_INTERNAL.lock().shaders = shaders;

//     Ok(BTerm {
//         width_pixels,
//         height_pixels,
//         original_width_pixels: width_pixels,
//         original_height_pixels: height_pixels,
//         fps: 0.0,
//         frame_time_ms: 0.0,
//         active_console: 0,
//         key: None,
//         mouse_pos: (0, 0),
//         left_click: false,
//         shift: false,
//         alt: false,
//         control: false,
//         web_button: None,
//         quitting: false,
//         post_scanlines: false,
//         post_screenburn: false,
//         screen_burn_color: screen_burn_color: bracket_color::prelude::RGB::from_f32(0.0, 1.0, 1.0)
//     })
// }


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

    run();
    
    Ok(())
}

