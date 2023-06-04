use crate::map::{MapBuilder, MapCell};

pub struct TerrainBuilder {}

impl Default for TerrainBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl MapBuilder for TerrainBuilder {
    fn build(&mut self, mut map: crate::map::Map) -> crate::map::Map {
        for x in 0..map.size.x {
            for y in 0..map.size.y {
                for z in 0..map.size.z / 2 {
                    map.set_cell(x, y, z, MapCell::Floor)
                }
            }
        }
        map
    }
}
