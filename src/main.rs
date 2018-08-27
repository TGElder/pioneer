pub mod version;
pub mod world;
pub mod graphics;

extern crate piston;
extern crate piston_window;
extern crate drag_controller;


use self::piston::window::WindowSettings;
use self::piston_window::*;

use world::{World, Heightmap};
use version::{Version, Local};
use graphics::Graphics;
use std::sync::{Arc, RwLock};
use drag_controller::{ DragController, Drag };

const OPENGL: OpenGL = OpenGL::V3_2;
const WINDOW_TITLE: &'static str = "Pioneer";

fn main() {

    let height_map = Heightmap::from_grayscale_image("807.png");
    let world: World = World::new(height_map, 64);
    let world_version: Version<World> = Arc::new(RwLock::new(Some(Arc::new(world))));

    let mut window = create_window();
    let mut graphics = Graphics::new(OPENGL, Local::new(&world_version));

    graphics.update_primitives();

    let mut drag_controller = DragController::new();
    let mut drag_last_pos: [f64; 2] = [0.0, 0.0];

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            graphics.render(&r);
        }

        drag_controller.event(&e, |action| {
            match action {
                Drag::Start(x, y) => {
                    drag_last_pos = [x, y];
                    true
                }
                Drag::Move(x, y) => {
                    graphics.offset.0 += x - drag_last_pos[0];
                    graphics.offset.1 += y - drag_last_pos[1];
                    drag_last_pos = [x, y];
                    true
                }
                Drag::End(_, _) => false,
                Drag::Interrupt => true,
            }
        });
       
        if let Some(s) = e.mouse_scroll_args() {
            if s[1] == 1.0 { // zoom in
                graphics.scale *= 2.0;
            } else if s[1] == -1.0 { // zoom out
                graphics.scale /= 2.0;
            }
        }

    }
}

pub fn create_window() -> PistonWindow {

        WindowSettings::new(
            WINDOW_TITLE,
            [2048, 1024]
        )
        .opengl(OPENGL)
        .vsync(false)
        .exit_on_esc(true)
        .samples(0)
        .build()
        .unwrap()
}
