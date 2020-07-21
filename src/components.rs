use specs::prelude::*;
use specs_derive::Component;
use rltk::{RGB, Point, FontCharType};

#[derive(Component)]
pub struct FoV {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct LeftMover {    
}

#[derive(Component, Debug)]
pub struct Player {
}