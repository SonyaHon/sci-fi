use builder::TerrainBuilder;
use components::RenderData;
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

trait Renderer {
    fn render(&mut self, x: impl TryInto<i32>, y: impl TryInto<i32>, data: &RenderData);
}

impl Renderer for rltk::BTerm {
    fn render(&mut self, x: impl TryInto<i32>, y: impl TryInto<i32>, data: &RenderData) {
        self.set(x, y, data.fg, data.bg, data.glyph)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        let mut camera_offset = self.ecs.fetch_mut::<CameraOffset>();

        match ctx.key {
            None => {}
            Some(key) => match key {
                rltk::VirtualKeyCode::Left => {
                    camera_offset.offset.x += 10;
                }
                rltk::VirtualKeyCode::Right => {
                    camera_offset.offset.x -= 10;
                }
                rltk::VirtualKeyCode::Up => {
                    camera_offset.offset.y += 10;
                }
                rltk::VirtualKeyCode::Down => {
                    camera_offset.offset.y -= 10;
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
                let void_color = rltk::DARK_CYAN;

                if let Some(cell) = cell {
                    match cell {
                        map::MapCell::Void => {
                            let mut encoutered = false;
                            for layer_under in 1..6 {
                                let cell_under =
                                    map.get_cell(world_x, world_y, layer - layer_under);
                                if let Some(cell_under) = cell_under {
                                    match cell_under {
                                        map::MapCell::Void => {}
                                        _ => {
                                            let data = cell_data.get_data(cell_under);
                                            encoutered = true;
                                            ctx.set(
                                                screen_x + 1,
                                                screen_y + 1,
                                                data.render_data.fg.lerp(
                                                    rltk::RGBA::named(void_color),
                                                    0.2 * layer_under as f32,
                                                ),
                                                data.render_data.bg.lerp(
                                                    rltk::RGBA::named(void_color),
                                                    0.2 * layer_under as f32,
                                                ),
                                                data.render_data.glyph,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                            if !encoutered {
                                ctx.set(
                                    screen_x + 1,
                                    screen_y + 1,
                                    void_color,
                                    void_color,
                                    rltk::to_cp437(' '),
                                );
                            }
                        }
                        _ => {
                            let data = cell_data.get_data(cell);
                            ctx.render(screen_x + 1, screen_y + 1, &data.render_data);
                        }
                    }
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
        .insert(Map::new(500, 500, 200).apply_build(TerrainBuilder::default()));
    gamestate.ecs.insert(CameraOffset::init().set_z(100));

    rltk::main_loop(context, gamestate)
}
