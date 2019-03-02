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
    let seed = 3;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..9 {
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.1, 0.9));
        if i < 9 {
            let threshold = i * i;
            mesh = Erosion::erode(mesh, &mut rng, threshold, 16);
        }
        println!("{}-{}", i, mesh.get_width());
    }
    
    let max_z = 100.0;
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, max_z)));
    
    let terrain = mesh.get_z_vector().map(|z| z as f32);

    let mut engine = IsometricEngine::new("Isometric", 1024, 1024, terrain, max_z as f32);
    
    engine.run();
   
}

