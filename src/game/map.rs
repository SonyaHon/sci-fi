use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::{components::RenderData, util::Vec3};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum MapCell {
    Void,
    Floor,

    Rock,

    Grass,
    TallGrass,
    ShortGrass,
    Dirt,

    Water,
}

impl MapCell {
    pub fn get_random_grass_cell() -> MapCell {
        vec![
            MapCell::Grass,
            MapCell::TallGrass,
            MapCell::ShortGrass,
            MapCell::Dirt,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
    }
}

pub struct MapCellData {
    pub render_data: RenderData,
}

struct DataBuilder {
    data: MapCellData,
}
impl DataBuilder {
    pub fn glyph(ch: char) -> Self {
        Self {
            data: MapCellData {
                render_data: RenderData {
                    fg: rltk::RGBA::named(rltk::WHITE),
                    bg: rltk::RGBA::named(rltk::BLACK),
                    glyph: rltk::to_cp437(ch),
                },
            },
        }
    }

    pub fn with_fg(mut self, color: impl Into<rltk::RGBA>) -> Self {
        self.data.render_data.fg = color.into();
        self
    }

    pub fn with_bg(mut self, color: impl Into<rltk::RGBA>) -> Self {
        self.data.render_data.bg = color.into();
        self
    }
}

pub struct MapCellDataStorage {
    data: HashMap<MapCell, MapCellData>,
}

impl MapCellDataStorage {
    pub fn init() -> Self {
        let mut data = HashMap::new();

        data.insert(
            MapCell::Void,
            DataBuilder::glyph(' ')
                .with_fg(rltk::DARK_CYAN)
                .with_bg(rltk::DARK_CYAN)
                .data,
        );

        data.insert(
            MapCell::Floor,
            DataBuilder::glyph('.').with_fg(rltk::LIGHT_SALMON).data,
        );

        data.insert(
            MapCell::Rock,
            DataBuilder::glyph('#')
                .with_fg(rltk::GRAY50)
                .with_bg(rltk::GRAY30)
                .data,
        );

        data.insert(
            MapCell::Grass,
            DataBuilder::glyph('`')
                .with_fg(rltk::GREEN)
                .with_bg(rltk::BLACK)
                .data,
        );
        data.insert(
            MapCell::TallGrass,
            DataBuilder::glyph('\'')
                .with_fg(rltk::GREEN4)
                .with_bg(rltk::BLACK)
                .data,
        );

        data.insert(
            MapCell::ShortGrass,
            DataBuilder::glyph(',')
                .with_fg(rltk::WEB_GREEN)
                .with_bg(rltk::BLACK)
                .data,
        );
        data.insert(
            MapCell::Dirt,
            DataBuilder::glyph('.')
                .with_fg(rltk::BROWN4)
                .with_bg(rltk::BLACK)
                .data,
        );

        data.insert(
            MapCell::Water,
            DataBuilder::glyph('~')
                .with_fg(rltk::WHITE_SMOKE)
                .with_bg(rltk::DARK_BLUE)
                .data,
        );

        Self { data }
    }

    pub fn get_data(&self, cell: &MapCell) -> &MapCellData {
        self.data.get(cell).unwrap()
    }
}

pub struct Map {
    pub size: Vec3,
    pub data: Vec<MapCell>,
}

impl Map {
    pub fn default_simple() -> Self {
        Self {
            size: Vec3::new(10, 10, 1),
            data: vec![MapCell::Floor; 100],
        }
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            size: Vec3::new(x, y, z),
            data: vec![MapCell::Void; (x * y * z) as usize],
        }
    }

    pub fn apply_build(self, mut builder: impl MapBuilder) -> Self {
        builder.build(self)
    }

    fn xyztoi(&self, x: i32, y: i32, z: i32) -> usize {
        ((z * self.size.x * self.size.y) + (y * self.size.x) + x) as usize
    }

    pub fn set_cell(&mut self, x: i32, y: i32, z: i32, cell: MapCell) {
        let i = self.xyztoi(x, y, z);
        self.data[i] = cell;
    }

    pub fn get_cell(&self, x: i32, y: i32, z: i32) -> Option<&MapCell> {
        if x >= 0 && x < self.size.x && y >= 0 && y < self.size.y && z >= 0 && z < self.size.z {
            return self.data.get(self.xyztoi(x, y, z));
        }
        None
    }
}

pub struct CameraOffset {
    pub offset: Vec3,
}

impl CameraOffset {
    pub fn init() -> Self {
        Self {
            offset: Vec3::zero(),
        }
    }

    pub fn set_z(mut self, z: i32) -> Self {
        self.offset.z = z;
        self
    }
}

pub trait MapBuilder {
    fn build(&mut self, map: Map) -> Map;
}
