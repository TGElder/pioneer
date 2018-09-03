extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use world::{World, Heightmap};
use version::Local;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use self::piston::input::*;
use graphics::graphics::{clear, polygon, line, Transformed};
use std::f64;

pub struct Graphics {
    graphics: GlGraphics,
    world: Local<World>,
    polygons: Vec<ColoredPolygon>,
    lines: Vec<ColoredLine>,
    pub offset: (f64, f64),
    pub scale: f64,
    rotation: usize,
    pub projection: IsometricProjection
}

impl Graphics {

    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

    pub fn new(opengl: OpenGL, world: Local<World>) -> Graphics {
        Graphics{
            graphics: GlGraphics::new(opengl),
            world,
            polygons: vec![],
            lines: vec![],
            offset: (0.0, 0.0),
            scale: 1.0,
            rotation: 0,
            projection: Graphics::get_projection(0)
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        let offset = &self.offset;
        let scale = &self.scale;
        let polygons = &self.polygons;
        let lines = &self.lines;

        self.graphics.draw(args.viewport(), |c, gl| {

            clear(Graphics::BLUE, gl);

            let transform = c.transform.trans(offset.0, offset.1).scale(*scale, *scale);
                                       
            for p in polygons {
                polygon(p.color, &p.polygon, transform, gl);
            }

            for l in lines {
                line(l.color, *scale / 10.0, l.line, transform, gl);
            }
            
        });

    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
        self.projection = Graphics::get_projection(self.rotation);
    }

    fn get_projection(rotation: usize) -> IsometricProjection {
        const COEFFS: [(f64, f64); 4] = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
        IsometricProjection{
            c: COEFFS[rotation].0,
            s: COEFFS[rotation].1,
            z: 10.0
        }
    }

    pub fn update_primitives(&mut self) {
        self.world.update();
        if let Some(w) = &self.world.local {
            
            const X_DELTAS: [usize; 4] = [0, 1, 1, 0];
            const Y_DELTAS: [usize; 4] = [0, 0, 1, 1];

            let max_height_over_sea_level: f32 = Heightmap::MAX_HEIGHT as f32 - w.sea_level as f32;

            let width: usize = w.heightmap.width as usize;
            let height: usize = w.heightmap.height as usize;

            self.polygons = Vec::with_capacity((width - 1) as usize * (height - 1) as usize);

            for i in 0..width - 1 {

                // Ensures back tiles are drawn first
                let x = if self.projection.s == -1.0 {(width - 2) - i} else {i};

                for j in 0..height - 1 {

                    let y = if self.projection.c == -1.0 {(height- 2) - j} else {j};

                    let mut polygon: Vec<[f64; 2]> = vec![];
                    let mut above_sea: Vec<usize> = vec![];
                    let mut points: [(f64, f64, f64); 4] = [(0.0, 0.0, 0.0); 4];
                    let mut color: f32 = 0.0;
                
                    for d in 0..4 {
                        let x_focus = x + X_DELTAS[d];
                        let y_focus = y + Y_DELTAS[d];
                        let mut z_focus = w.heightmap.get(&(x_focus as u32), &(y_focus as u32));

                        points[d] = (x_focus as f64, y_focus as f64, z_focus as f64 * 1.0);

                        z_focus = if z_focus <= w.sea_level {
                            0.0
                        } else {
                            above_sea.push(d);
                            z_focus - w.sea_level
                        };
          
                        let iso = self.projection.to_isometric(x_focus as u32, y_focus as u32, z_focus);

                        polygon.push([iso.0, iso.1]);

                        color += z_focus as f32 / max_height_over_sea_level;
                    }

                    let color = (get_color(points) + color / 4.0) / 2.0;

                    if above_sea.len() == 1 {
                        polygon.remove((above_sea[0] + 2) % 4);
                    }

                    if !above_sea.is_empty() {
                        self.polygons.push(ColoredPolygon{polygon, color: [0.0, color, 0.0, 1.0]});
                    }
                }
            }

            self.lines = vec![];

            for river in w.rivers.iter() {
                let x = river[0];
                let y = river[1];
                let mut z = w.heightmap.get(&(x as u32), &(y as u32));
                let iso_from = self.projection.to_isometric(y, x, z);
                let x = river[2];
                let y = river[3];
                let mut z = w.heightmap.get(&(x as u32), &(y as u32));
                let iso_to = self.projection.to_isometric(y, x, z);
                let line = [iso_from.0, iso_from.1, iso_to.0, iso_to.1];
                self.lines.push(ColoredLine{line, color: Graphics::BLUE});
            }

        }
    }

   

}


struct ColoredPolygon {
    polygon: Vec<[f64; 2]>,
    color: [f32; 4],
}

struct ColoredLine {
    line: [f64; 4],
    color: [f32; 4],
}

pub struct IsometricProjection {
    c: f64,
    s: f64,
    z: f64
}

impl IsometricProjection {
    pub fn to_isometric(&self, x: u32, y: u32, z: f64) -> (f64, f64) {
        let iso_x = self.c * x as f64 - self.s * y as f64;
        let iso_y = ((self.s * x as f64 + self.c* y as f64) / 2.0) - (z / self.z);
        (iso_x, iso_y)
    }

    pub fn to_world(&self, iso_x: f64, iso_y: f64) -> (u32, u32) {
        let y = ((2.0 * iso_y * self.c) - (self.s * iso_x)) / 2.0;
        let x = (iso_x + self.s * y) / self.c;
        (x as u32, y as u32)
    }
}

 fn get_color(points: [(f64, f64, f64); 4]) -> f32 {
        let u = sub(points[0], points[2]);
        let v = sub(points[1], points[3]);
        let normal = cross(u, v);
        let light = (1.0, 0.0, 1.0);
        (angle(normal, light) / f64::consts::PI) as f32

}

fn dot(u: (f64, f64, f64), v: (f64, f64, f64)) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

fn sub(u: (f64, f64, f64), v: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        u.0 - v.0,
        u.1 - v.1,
        u.2 - v.2
    )
}

fn mag(u: (f64, f64, f64)) -> f64 {
    (u.0 * u.0 + u.1 * u.1 + u.2 * u.2).sqrt()
}

fn cross(u: (f64, f64, f64), v: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0
    )
}

fn angle(u: (f64, f64, f64), v: (f64, f64, f64)) -> f64 {
    (dot(u, v) / (mag(u) * mag(v))).acos()
}