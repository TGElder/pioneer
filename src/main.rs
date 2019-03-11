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
use downhill_map::DownhillMap;
use single_downhill_map::{SingleDownhillMap, RandomDownhillMap};
use flow_map::FlowMap;
use scale::Scale;
use rand::prelude::*;
use std::f64::MAX;
use isometric::engine::IsometricEngine;
use isometric::graphics::drawing::rivers::River;
use downhill_map::DIRECTIONS;

fn main() {

    let mut mesh = Mesh::new(1, 0.0);
    mesh.set_z(0, 0, MAX);
    let seed = 4;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..11 {
        mesh = MeshSplitter::split(&mesh, &mut rng, (0.0, 0.75));
        if i < 9 {
            let threshold = i * 2;
            mesh = Erosion::erode(mesh, &mut rng, threshold, 8);
        }
        println!("{}-{}", i, mesh.get_width());
    }

    let downhill_map = DownhillMap::new(&mesh);
    let random_downhill_map: Box<SingleDownhillMap> = Box::new(RandomDownhillMap::new(&downhill_map, &mut rng));
    let flow_map = FlowMap::from(&mesh, &random_downhill_map);
    
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 64.0)));
    let terrain = mesh.get_z_vector().map(|z| z as f32);

    let mut rivers = vec![];
    for x in 0..mesh.get_width() {
        for y in 0..mesh.get_width() {
            let flow = flow_map.get_flow(x, y);
            if flow > 64 {
                let direction = random_downhill_map.get_direction(x, y);
                let nx = x + DIRECTIONS[direction].0;
                let ny = y + DIRECTIONS[direction].1;
                if mesh.in_bounds(nx, ny) {
                    let neighbour = na::Vector2::new(nx as usize, ny as usize);
                    rivers.push(River::new(na::Vector2::new(x as usize, y as usize), neighbour));
                }
            }
        }
    }

    let mut engine = IsometricEngine::new("Isometric", 1024, 1024, 64.0, terrain, rivers);
    
    engine.run();
   
}
