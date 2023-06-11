use noise::*;
use splines::{Interpolation, Key, Spline};

use crate::map::{MapBuilder, MapCell};

pub struct TerrainBuilder {
    spline: Spline<f32, f32>,
    noise: Clamp<f64, HybridMulti<OpenSimplex>, 2>,
}

impl Default for TerrainBuilder {
    fn default() -> Self {
        let spline = Spline::from_vec(vec![
            Key::new(-1.0, 60.0, Interpolation::Linear), // Ocean floor
            Key::new(-0.8, 70.0, Interpolation::Linear), // Ocean floor
            Key::new(-0.7, 80.0, Interpolation::Linear), // Ocean floor
            Key::new(-0.6, 90.0, Interpolation::Linear), // Ocean shoore
            Key::new(-0.5, 95.0, Interpolation::Linear), // Ocean shoore
            Key::new(-0.4, 98.0, Interpolation::Linear), // Ocean Land Border
            Key::new(0.0, 100.0, Interpolation::Linear), // Land
            Key::new(1.0, 100.0, Interpolation::Linear), // Land
        ]);

        let noise = Clamp::new(
            HybridMulti::<OpenSimplex>::new(69 + 420)
                .set_octaves(8)
                .set_frequency(2.0)
                .set_persistence(0.5)
                .set_lacunarity(2.2),
        )
        .set_bounds(-0.99, 0.99);

        Self { spline, noise }
    }
}

impl TerrainBuilder {
    fn get_elevation(&mut self, x: i32, y: i32) -> i32 {
        let scale = 2000.0;

        let noise_value = self.noise.get([x as f64 / scale, y as f64 / scale]) as f32;
        let elevation = self.spline.sample(noise_value).unwrap();
        f32::round(elevation) as i32
    }
}

impl MapBuilder for TerrainBuilder {
    fn build(&mut self, mut map: crate::map::Map) -> crate::map::Map {
        let water_level = 95;

        for x in 0..map.size.x {
            for y in 0..map.size.y {
                let max_elevation_at_point = self.get_elevation(x, y);
                for z in 0..map.size.z {
                    let cell = match z {
                        z if z <= water_level && z >= max_elevation_at_point => MapCell::Water,
                        z if z == max_elevation_at_point - 1 => MapCell::get_random_grass_cell(),
                        z if z < max_elevation_at_point => MapCell::Rock,
                        _ => MapCell::Void,
                    };

                    map.set_cell(x, y, z, cell)
                }
            }
        }

        map
    }
}
