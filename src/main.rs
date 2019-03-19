pub mod scale;
pub mod utils;
pub mod mesh;
pub mod downhill_map;
pub mod mesh_splitter;
pub mod single_downhill_map;
pub mod flow_map;
pub mod erosion;
pub mod river_runner;

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
use river_runner::get_junctions_and_rivers;
use isometric::engine::IsometricEngine;
use isometric::engine::TerrainHandler;

fn main() {

    let mut mesh = Mesh::new(1, 0.0);
    mesh.set_z(0, 0, MAX);
    let seed = 11;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..9 {
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.0, 0.75));
        if i < 9 {
            let threshold = i * 2;
            mesh = Erosion::erode(mesh, &mut rng, threshold, 8);
        }
        println!("{}-{}", i, mesh.get_width());
    }

    let sea_level = 1.0;
    let (junctions, rivers) = get_junctions_and_rivers(&mesh, 256, sea_level, (0.01, 0.49), &mut rng);

    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 16.0)));
    let terrain = mesh.get_z_vector().map(|z| z as f32);
    
    let terrain_handler = TerrainHandler::new(terrain, junctions, rivers, sea_level as f32);
    let mut engine = IsometricEngine::new("Isometric", 1024, 1024, 16.0, Box::new(terrain_handler));
    
    engine.run();
   
}
