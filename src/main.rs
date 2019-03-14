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
use isometric::graphics::drawing::terrain::River;
use downhill_map::DIRECTIONS;

fn main() {

    let mut mesh = Mesh::new(1, 0.0);
    mesh.set_z(0, 0, MAX);
    let seed = 7;
    let mut rng = Box::new(SmallRng::from_seed([seed; 16]));

    for i in 0..10 {
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
    
    mesh = mesh.rescale(&Scale::new((mesh.get_min_z(), mesh.get_max_z()), (0.0, 32.0)));
    let terrain = mesh.get_z_vector().map(|z| z as f32);
    

    let mut include: na::DMatrix<bool> = na::DMatrix::from_element(mesh.get_width() as usize, mesh.get_width() as usize, false);

    let sea_level = 2.0;
    for x in 0..mesh.get_width() {
        for y in 0..mesh.get_width() {
            let flow = flow_map.get_flow(x, y);
            if flow > 256 && mesh.get_z(x, y) >= sea_level {
                include[(x as usize, y as usize)] = true;
                let direction = random_downhill_map.get_direction(x, y);
                let nx = x + DIRECTIONS[direction].0;
                let ny = y + DIRECTIONS[direction].1;
                if mesh.in_bounds(nx, ny) {
                    include[(nx as usize, ny as usize)] = true;
                }
            }
        }
    }    

    let flow_threshold = 512;
    let mut max_flow = 0;
    let mut tuples = vec![];
    for x in 0..mesh.get_width() {
        for y in 0..mesh.get_width() {
            let flow = flow_map.get_flow(x, y);
            if include[(x as usize, y as usize)] {
                let direction = random_downhill_map.get_direction(x, y);
                let nx = x + DIRECTIONS[direction].0;
                let ny = y + DIRECTIONS[direction].1;
                if mesh.in_bounds(nx, ny) {
                    max_flow = max_flow.max(flow);
                    let neighbour = na::Vector2::new(nx as usize, ny as usize);
                    tuples.push((na::Vector2::new(x as usize, y as usize), neighbour, flow as f64));
                }
            }
        }
    }

    let mut rivers = vec![];
    let flow_scale = Scale::new((flow_threshold as f64, max_flow as f64), (0.1, 0.35));
    for tuple in tuples.iter_mut() {
        rivers.push(River::new(tuple.0, tuple.1, flow_scale.scale(tuple.2) as f32));
    }

    let mut engine = IsometricEngine::new("Isometric", 1024, 1024, 64.0, terrain, rivers, sea_level as f32);
    
    engine.run();
   
}
