use crate::util::Vec3;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Location {
    pub coordinates: Vec3,
}

#[derive(Component)]
pub struct RenderData {
    pub fg: rltk::RGBA,
    pub bg: rltk::RGBA,
    pub glyph: rltk::FontCharType,
}

