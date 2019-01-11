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
    drawables: Vec<Drawable>,
    pub offset: (f64, f64),
    pub scale: f64,
    rotation: usize,
    pub projection: IsometricProjection,
    rivers: Option<Vec<Vec<bool>>>
}

impl Graphics {

    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    pub fn new(opengl: OpenGL, world: Local<World>) -> Graphics {
        Graphics{
            graphics: GlGraphics::new(opengl),
            world,
            drawables: vec![],
            offset: (0.0, 0.0),
            scale: 1.0,
            rotation: 0,
            projection: Graphics::get_projection(0),
            rivers: None
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {

        

        let offset = &self.offset;
        let scale = &self.scale;
        let drawables = &self.drawables;

        let viewport = args.viewport().rect;
        let top_left: (f64, f64) = self.screen_to_canvas(viewport[0] as f64, viewport[1] as f64);
        let bottom_right: (f64, f64) = self.screen_to_canvas(viewport[2] as f64, viewport[3] as f64);

        self.graphics.draw(args.viewport(), |c, gl| {

            clear(Graphics::BLUE, gl);

            let transform = c.transform.trans(offset.0, offset.1).scale(*scale, *scale);
                                       
            for d in drawables {
                if Graphics::on_screen(top_left, bottom_right, d) {
                    match d {
                        Drawable::ColoredPolygon{polygon: p, color: c} => polygon(*c, p, transform, gl),
                        Drawable::ColoredLine{line: l, color: c, width: w} => line(*c, *scale * w, *l, transform, gl)
                    };
                }
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
            z: 20.0
        }
    }

    pub fn update_primitives(&mut self) {
        self.world.update();
        if let Some(w) = &self.world.local {

            match self.rivers {
                None => {
                    self.rivers = Graphics::get_rivers(&w);
                },
                _ => {}
            };
            
            const X_DELTAS: [usize; 4] = [0, 1, 1, 0];
            const Y_DELTAS: [usize; 4] = [0, 0, 1, 1];

            let max_height_over_sea_level: f32 = Heightmap::MAX_HEIGHT as f32 - w.sea_level as f32;

            let width: usize = w.mesh.get_width() as usize;

            self.drawables = vec![];

            for i in 0..width - 1 {

                // Ensures back tiles are drawn first
                let x = if self.projection.s == -1.0 {(width - 2) - i} else {i};

                for j in 0..width - 1 {

                    let y = if self.projection.c == -1.0 {(width- 2) - j} else {j};

                    let mut polygon: Vec<[f64; 2]> = vec![];
                    let mut above_sea: Vec<usize> = vec![];
                    let mut points: [(f64, f64, f64); 4] = [(0.0, 0.0, 0.0); 4];
                    let mut color: f32 = 0.0;
                
                    for d in 0..4 {
                        let x_focus = x + X_DELTAS[d];
                        let y_focus = y + Y_DELTAS[d];
                        let mut z_focus = w.mesh.get_z(x_focus as i32, y_focus as i32);

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

                    // for d in 0..4 {
                    //     let d2 = (d + 1) % 4;
                    //     self.drawables.push(Drawable::ColoredLine{
                    //         line: [polygon[d][0],polygon[d][1], polygon[d2][0], polygon[d2][1]], 
                    //         color: Graphics::BLACK,
                    //         width: 0.005} );
                    // }

                    if above_sea.len() == 1 {
                        polygon.remove((above_sea[0] + 2) % 4);
                    }

                    let river = if let Some(r) = &self.rivers {
                        r[x][y]
                    } else{
                        false
                    };

                    let color = if river {
                        Graphics::BLUE
                    } else {
                        [0.0, color, 0.0, 1.0]
                    };

                    //let river: bool = &self.rivers.unwrap()[x][y];

                    if !above_sea.is_empty() {

                        self.drawables.push(Drawable::ColoredPolygon{polygon, color});
                    }
                }
            }

        }
    }

    fn get_rivers(world: &World) -> Option<Vec<Vec<bool>>> {

        let mut vector = vec![vec![false; world.mesh.get_width() as usize]; world.mesh.get_width() as usize];

        for river in world.rivers.iter() {
            vector[river[0] as usize][river[1] as usize] = true;
        }
        Some(vector)
    }

    pub fn screen_to_canvas(&self, screen_x: f64, screen_y: f64) -> (f64, f64) {
        let canvas_x = (screen_x - &self.offset.0) / &self.scale;
        let canvas_y = (screen_y - &self.offset.1) / &self.scale;

        (canvas_x, canvas_y)
    }

    fn on_screen(top_left: (f64, f64), bottom_right: (f64, f64), drawable: &Drawable) -> bool {

        fn coord_in_bounds(coord: [f64; 2], top_left: (f64, f64), bottom_right: (f64, f64)) -> bool {
            coord[0] >= top_left.0 && coord[0] <= bottom_right.0 &&
            coord[1] >= top_left.1 && coord[1] <= bottom_right.1
        }
        
        match drawable {
            Drawable::ColoredPolygon{polygon, color: _} => {
                    coord_in_bounds(polygon[0], top_left, bottom_right)// &&
                    //coord_in_bounds(polygon[1], top_left, bottom_right) &&
                    //coord_in_bounds(polygon[2], top_left, bottom_right) &&
                    //coord_in_bounds(polygon[3], top_left, bottom_right)
                }
            ,
            Drawable::ColoredLine{line, color: _, width: _} => {
                    coord_in_bounds([line[0], line[1]], top_left, bottom_right)// &&
                    //coord_in_bounds([line[2], line[3]], top_left, bottom_right)
            }
            ,
        }
    }

}

enum Drawable {
    ColoredPolygon {
        polygon: Vec<[f64; 2]>,
        color: [f32; 4],
    },
    ColoredLine {
        line: [f64; 4],
        color: [f32; 4],
        width: f64
    },
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