use builder::TerrainBuilder;
use map::{CameraOffset, Map, MapCellDataStorage};
use rltk::GameState;
use specs::prelude::*;

pub mod builder;
pub mod components;
pub mod map;
pub mod util;

struct State {
    ecs: World,
}

impl State {
    pub fn new() -> Self {
        Self { ecs: World::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        let mut camera_offset = self.ecs.fetch_mut::<CameraOffset>();

        match ctx.key {
            None => {}
            Some(key) => match key {
                rltk::VirtualKeyCode::Left => {
                    camera_offset.offset.x += 1;
                }
                rltk::VirtualKeyCode::Right => {
                    camera_offset.offset.x -= 1;
                }
                rltk::VirtualKeyCode::Up => {
                    camera_offset.offset.y += 1;
                }
                rltk::VirtualKeyCode::Down => {
                    camera_offset.offset.y -= 1;
                }
                rltk::VirtualKeyCode::Period if ctx.shift => {
                    camera_offset.offset.z -= 1;
                }
                rltk::VirtualKeyCode::Comma if ctx.shift => {
                    camera_offset.offset.z += 1;
                }
                _ => {}
            },
        }

        ctx.cls();
        let (screen_width, screen_height) = ctx.get_char_size();

        for x in 0..screen_width {
            ctx.set(x, 0, rltk::GRAY30, rltk::BLACK, rltk::to_cp437('█'));
            ctx.set(
                x,
                screen_height - 1,
                rltk::GRAY30,
                rltk::BLACK,
                rltk::to_cp437('█'),
            );
        }
        for y in 0..screen_height {
            ctx.set(0, y, rltk::GRAY30, rltk::BLACK, rltk::to_cp437('█'));
            ctx.set(
                screen_width - 1,
                y,
                rltk::GRAY30,
                rltk::BLACK,
                rltk::to_cp437('█'),
            );
        }
        for y in 0..screen_height - 4 {
            ctx.set(
                screen_width - 1,
                screen_height - y - 3,
                if camera_offset.offset.z == y as i32 {
                    rltk::YELLOW
                } else {
                    rltk::BLUE
                },
                rltk::BLACK,
                rltk::to_cp437('█'),
            );
        }

        let map = self.ecs.fetch::<Map>();
        let cell_data = self.ecs.fetch::<MapCellDataStorage>();
        let layer = camera_offset.offset.z;

        for screen_x in 0..screen_width - 2 {
            for screen_y in 0..screen_height - 2 {
                let world_x = screen_x as i32 - camera_offset.offset.x;
                let world_y = screen_y as i32 - camera_offset.offset.y;

                let cell = map.get_cell(world_x, world_y, layer);
                if let Some(cell) = cell {
                    let data = cell_data.get_data(cell);
                    ctx.set(screen_x + 1, screen_y + 1, data.fg, data.bg, data.glyph);
                }
            }
        }
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple(154, 104)
        .unwrap()
        .with_title("SciFi Rust #0.0.0")
        .with_fitscreen(true)
        .build()?;

    let mut gamestate = State::new();

    gamestate.ecs.insert(MapCellDataStorage::init());
    gamestate
        .ecs
        .insert(Map::new(100, 100, 100).apply_build(TerrainBuilder::default()));
    gamestate.ecs.insert(CameraOffset::init());

    rltk::main_loop(context, gamestate)
}
