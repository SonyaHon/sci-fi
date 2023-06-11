use noise::*;
use splines::{Interpolation, Key, Spline};

#[derive(Debug)]
struct State {
    seed: u32,
    scale: f64,
    tx: f64,
    ty: f64,
}

impl State {
    fn new() -> Self {
        Self {
            seed: 32,
            scale: 100.0,
            tx: -0.5,
            ty: -0.5,
        }
    }
}

impl rltk::GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        match ctx.key {
            None => {}
            Some(key) => {
                match key {
                    rltk::VirtualKeyCode::Q => self.scale += 10.0,
                    rltk::VirtualKeyCode::A => self.scale -= 10.0,

                    rltk::VirtualKeyCode::Equals => self.seed += 1,
                    rltk::VirtualKeyCode::Minus => self.seed -= 1,

                    rltk::VirtualKeyCode::Left => self.tx -= 0.5,
                    rltk::VirtualKeyCode::Right => self.tx += 0.5,

                    rltk::VirtualKeyCode::Up => self.ty -= 0.5,
                    rltk::VirtualKeyCode::Down => self.ty += 0.5,

                    _ => {}
                }

                println!("{:?}", self);
            }
        }

        let continentalness_noise = Clamp::new(
            HybridMulti::<OpenSimplex>::new(self.seed)
                .set_octaves(4)
                .set_frequency(1.0)
                .set_persistence(0.5)
                .set_lacunarity(2.2),
        )
        .set_bounds(-0.99, 0.99);

        let erosion_noise = Clamp::new(
            HybridMulti::<OpenSimplex>::new(self.seed)
                .set_octaves(2)
                .set_frequency(3.0)
                .set_persistence(0.5)
                .set_lacunarity(2.2),
        )
        .set_bounds(-0.99, 0.99);

        let continentalness_spline = Spline::from_vec(vec![
            Key::new(-1.0, 5.0, Interpolation::Linear),
            Key::new(-0.6, 10.0, Interpolation::Linear),
            Key::new(-0.5, 30.0, Interpolation::Linear),
            Key::new(0.0, 32.0, Interpolation::Linear),
            Key::new(0.5, 50.0, Interpolation::Linear),
            Key::new(0.8, 60.0, Interpolation::Linear),
            Key::new(1.0, 80.0, Interpolation::default()),
        ]);

        for x in 0..200 {
            for y in 0..100 {
                let continentalness =
                    continentalness_noise.get([x as f64 / 40.0, y as f64 / 40.0]) as f32;

                let erosion = erosion_noise.get([x as f64 / 150.0, y as f64 / 150.0]) as f32;

                let elevation = continentalness_spline.sample(continentalness).unwrap();

                let color = rltk::RGBA::named(rltk::BLACK)
                    .lerp(rltk::RGBA::named(rltk::WHITE), elevation / 100.0);
                ctx.set(x, y, color, color, rltk::to_cp437(' '));
            }
        }
    }
}

pub fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple(200, 100)
        .unwrap()
        .with_title("Demo")
        .with_fitscreen(true)
        .build()?;

    let state = State::new();

    rltk::main_loop(context, state)
}
