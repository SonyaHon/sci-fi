use std::collections::HashMap;

use crate::util::Vec3;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum MapCell {
    Void,
    Floor,
}

pub struct MapCellData {
    pub fg: rltk::RGBA,
    pub bg: rltk::RGBA,
    pub glyph: rltk::FontCharType,
}

struct DataBuilder {
    data: MapCellData,
}
impl DataBuilder {
    pub fn glyph(ch: char) -> Self {
        Self {
            data: MapCellData {
                fg: rltk::RGBA::named(rltk::WHITE),
                bg: rltk::RGBA::named(rltk::BLACK),
                glyph: rltk::to_cp437(ch),
            },
        }
    }

    pub fn with_fg(mut self, color: impl Into<rltk::RGBA>) -> Self {
        self.data.fg = color.into();
        self
    }

    pub fn with_bg(mut self, color: impl Into<rltk::RGBA>) -> Self {
        self.data.bg = color.into();
        self
    }
}

pub struct MapCellDataStorage {
    data: HashMap<MapCell, MapCellData>,
}

impl MapCellDataStorage {
    pub fn init() -> Self {
        let mut data = HashMap::new();

        data.insert(MapCell::Void, DataBuilder::glyph(' ').data);

        data.insert(
            MapCell::Floor,
            DataBuilder::glyph('.').with_fg(rltk::LIGHT_SALMON).data,
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

    fn itoxyz(&self, i: usize) -> Vec3 {
        let mut idx = i as i32;
        let z = idx / (self.size.x * self.size.y);
        idx -= z * self.size.x * self.size.y;
        let y = idx / self.size.x;
        let x = idx % self.size.x;
        Vec3::new(x, y, z)
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
}

pub trait MapBuilder {
    fn build(&mut self, map: Map) -> Map;
}
