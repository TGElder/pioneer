pub mod scale;
pub mod utils;
pub mod mesh;
pub mod downhill_map;
pub mod mesh_splitter;
pub mod single_downhill_map;
pub mod flow_map;

pub mod version;
pub mod world;
pub mod graphics;

extern crate piston;
extern crate piston_window;
extern crate drag_controller;
extern crate rand;

use self::piston::window::WindowSettings;
use self::piston_window::*;

use world::World;
use mesh::Mesh;
use mesh_splitter::MeshSplitter;
use downhill_map::DownhillMap;
use single_downhill_map::{SingleDownhillMap, RandomDownhillMap};
use flow_map::FlowMap;
use scale::Scale;
use rand::prelude::*;
use version::{Version, Local};
use graphics::Graphics;
use std::sync::{Arc, RwLock};
use drag_controller::{ DragController, Drag };

const OPENGL: OpenGL = OpenGL::V3_2;
const WINDOW_TITLE: &'static str = "Pioneer";

fn main() {

    let mut mesh = Mesh::new(1, 0.0);
    mesh.set_z(0, 0, 2048.0);
    let seed = 2;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..11 {
        print!("{}", i);
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.1, 0.7));
        let downhill_map = DownhillMap::new(&mesh);
        for _ in 0..8 {
            let random_downhill_map = RandomDownhillMap::new(&downhill_map, &mut rng);
            let random_downhill_map: Box<SingleDownhillMap> = Box::new(random_downhill_map);
            let flow_map = FlowMap::from(&mesh, &random_downhill_map);
            print!(".");
        }
        println!("{}", mesh.get_width());
    }
    
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 2048.0)));
    
    let world: World = World::new(mesh, 256.0, vec![]);
    let world_version: Version<World> = Arc::new(RwLock::new(Some(Arc::new(world))));

    let mut window = create_window();
    let mut graphics = Graphics::new(OPENGL, Local::new(&world_version));

    graphics.update_primitives();
    graphics.offset = (256.0, 0.0);

    let mut drag_controller = DragController::new();
    let mut drag_last_pos: [f64; 2] = [0.0, 0.0];
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

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

        if let Some(s) = e.mouse_cursor_args() {
            mouse_pos = s;
        }
       
        if let Some(s) = e.mouse_scroll_args() {
            let x_centre = (mouse_pos[0] - graphics.offset.0) / graphics.scale;
            let y_centre = (mouse_pos[1] - graphics.offset.1) / graphics.scale;
            if s[1] == 1.0 { // zoom in
                graphics.offset.0 = graphics.offset.0 - (x_centre * graphics.scale);
                graphics.offset.1 = graphics.offset.1 - (y_centre * graphics.scale);
                graphics.scale *= 2.0;
            } else if s[1] == -1.0 { // zoom out
                graphics.offset.0 = graphics.offset.0 + (x_centre * graphics.scale) / 2.0;
                graphics.offset.1 = graphics.offset.1 + (y_centre * graphics.scale) / 2.0;
                graphics.scale /= 2.0;
            }
        }

        if let Some(Button::Keyboard(Key::Space)) = e.press_args() {

            let x_centre = (mouse_pos[0] - graphics.offset.0) / graphics.scale;
            let y_centre = (mouse_pos[1] - graphics.offset.1) / graphics.scale;

            let (iso_x_centre, iso_y_centre) = graphics.projection.to_world(x_centre, y_centre);


            graphics.rotate();

            let (x_centre_new, y_centre_new) = graphics.projection.to_isometric(iso_x_centre, iso_y_centre, 0.0);


            graphics.offset.0 = graphics.scale * (x_centre - x_centre_new) + graphics.offset.0;
            graphics.offset.1 = graphics.scale * (y_centre - y_centre_new) + graphics.offset.1;

            graphics.update_primitives();
        }

    }
}


pub fn create_window() -> PistonWindow {

        WindowSettings::new(
            WINDOW_TITLE,
            [512, 512]
        )
        .opengl(OPENGL)
        .vsync(false)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap()
}
