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
    mesh.set_z(0, 0, 1.0);
    let seed = 2;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..9 {
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.05, 0.9));
        if i < 9 {
            let threshold = i * 2;
            mesh = Erosion::erode(mesh, &mut rng, threshold, 8);
        }
        println!("{}-{}", i, mesh.get_width());
    }
    
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 32.0)));
    
    let terrain = mesh.get_z_vector().map(|z| z as f32);

    let mut engine = IsometricEngine::new("Isometric", 512, 256, 32.0, terrain);
    
    engine.run();
   
}

