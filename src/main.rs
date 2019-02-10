pub mod scale;
pub mod utils;
pub mod mesh;
pub mod downhill_map;
pub mod mesh_splitter;
pub mod single_downhill_map;
pub mod flow_map;
pub mod erosion;

pub mod version;
pub mod world;

extern crate rand;
extern crate isometric;
extern crate nalgebra as na;

use mesh::Mesh;
use mesh_splitter::MeshSplitter;
use erosion::Erosion;
use scale::Scale;
use rand::prelude::*;
use std::f64::MAX;
use isometric::engine::IsometricEngine;

fn main() {

    let mut mesh = Mesh::new(1, 0.0);
    mesh.set_z(0, 0, MAX);
    let seed = 2;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..9 {
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.05, 0.5));
        if i < 9 {
            let threshold = i * 2;
            mesh = Erosion::erode(mesh, &mut rng, threshold, 8);
        }
        println!("{}-{}", i, mesh.get_width());
    }
    
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 1.0)));
    
    let mut triangle_vertices: Vec<f32> = Vec::with_capacity((mesh.get_width() * mesh.get_width() * 36) as usize);
    let mut line_vertices: Vec<f32> = Vec::with_capacity((mesh.get_width() * mesh.get_width() * 24) as usize);
    for x in 0..mesh.get_width() {
        for y in 0..mesh.get_width() {
            let a = (x as f32, y as f32, mesh.get_z(x, y) as f32);
            let b = (x as f32 + 1.0, y as f32, mesh.get_z(x + 1, y) as f32);
            let c = (x as f32 + 1.0, y as f32 + 1.0, mesh.get_z(x + 1, y + 1) as f32);
            let d = (x as f32, y as f32 + 1.0, mesh.get_z(x, y + 1) as f32);
            triangle_vertices.extend([
                a.0, a.1, a.2, a.2, a.2, a.2,
                d.0, d.1, d.2, d.2, d.2, d.2,
                c.0, c.1, c.2, c.2, c.2, c.2,
                a.0, a.1, a.2, a.2, a.2, a.2,
                c.0, c.1, c.2, c.2, c.2, c.2,
                b.0, b.1, b.2, b.2, b.2, b.2
            ].iter().cloned());
            line_vertices.extend([
                a.0, a.1, a.2,
                b.0, b.1, b.2,
                b.0, b.1, b.2,
                c.0, c.1, c.2,
                c.0, c.1, c.2,
                d.0, d.1, d.2,
                d.0, d.1, d.2,
                a.0, a.1, a.2   
            ].iter().cloned());
        }
    }

    let mut engine = IsometricEngine::new("Isometric", 1024, 1024, triangle_vertices, line_vertices);
    
    engine.run();
   
}

