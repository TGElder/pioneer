extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use world::{World, Heightmap};
use version::Local;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use self::piston::input::*;
use graphics::graphics::{clear, polygon, Transformed};

pub struct Graphics {
    graphics: GlGraphics,
    world: Local<World>,
    polygons: Vec<ColoredPolygon>,
    pub offset: (f64, f64),
    pub scale: f64
}

impl Graphics {

    pub fn new(opengl: OpenGL, world: Local<World>) -> Graphics {
        Graphics{
            graphics: GlGraphics::new(opengl),
            world,
            polygons: vec![],
            offset: (0.0, 0.0),
            scale: 1.0
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let offset = &self.offset;
        let scale = &self.scale;
        let polygons = &self.polygons;

        self.graphics.draw(args.viewport(), |c, gl| {

            clear(BLUE, gl);

            let transform = c.transform.trans(offset.0, offset.1).scale(*scale, *scale);
                                       
            for p in polygons {
                polygon(p.color, &p.polygon, transform, gl);
            }
            
        });

    }

    pub fn update_primitives(&mut self) {
        self.world.update();
        if let Some(w) = &self.world.local {
            
            const X_DELTAS: [usize; 4] = [0, 1, 1, 0];
            const Y_DELTAS: [usize; 4] = [0, 0, 1, 1];

            let max_height_over_sea_level: f32 = Heightmap::MAX_HEIGHT as f32 - w.sea_level as f32;

            let width: usize = w.heightmap.width as usize;
            let height: usize = w.heightmap.height as usize;

            //self.polygons = Vec::with_capacity(width as usize - 1 * height as usize - 1);
            self.polygons = vec![];

            for x in 0..width - 1 {
                for y in 0..height - 1 {
                    let mut polygon: [[f64; 2]; 4] = [[0.0, 0.0]; 4];
                    let mut color: f32 = 0.0;
                
                    for d in 0..4 {
                        let x_focus = x + X_DELTAS[d];
                        let y_focus = y + Y_DELTAS[d];
                        let mut z_focus = w.heightmap.get(&(x_focus as u32), &(y_focus as u32));

                        z_focus = if z_focus <= w.sea_level {
                            0
                        } else {
                            z_focus - w.sea_level
                        };
          
                        let iso = Graphics::to_isometric(x_focus as u32, y_focus as u32, z_focus);
                        //println!("{}, {}, {} -> {}, {}", x_focus, y_focus, z_focus, iso.0, iso.1);
                        polygon[d] = [iso.0, iso.1];

                        color += z_focus as f32 / max_height_over_sea_level;
                    }

                    if color > 0.0 {
                        self.polygons.push(ColoredPolygon{polygon, color: [0.0, color/4.0, 0.0, 1.0]});
                    }
                }
            }

        }
    }

    pub fn to_isometric(x: u32, y: u32, z: u32) -> (f64, f64) {
        let iso_x = x as f64 - y as f64;
        let iso_y = ((x as f64 + y as f64) / 2.0) - (z as f64 / 10.0);
        (iso_x, iso_y)
    }

}

struct ColoredPolygon {
    polygon: [[f64; 2]; 4],
    color: [f32; 4],
}