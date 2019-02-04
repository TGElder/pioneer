use std::fs::File;
use std::io::prelude::*;
use mesh::Mesh;

#[derive(Clone, Debug)]
pub struct World {
    pub mesh: Mesh,
    pub sea_level: f64,
    pub rivers: Vec<[u32; 5]>
}

impl World {

    pub fn new(mesh: Mesh, sea_level: f64, rivers: Vec<[u32; 5]>) -> World {
        World{mesh, sea_level, rivers}
    }

    pub fn load_rivers_from_file(file: &str) -> Vec<[u32; 5]> {
        let mut f = File::open(file).expect("File not found");
        let mut text = String::new();
        f.read_to_string(&mut text).expect("Failed to read file");

        let mut out: Vec<[u32; 5]> = vec![];

        for row in text.split("\n") {
            if row.len() > 0 {
                let columns: Vec<&str> = row.split(",").collect();
                let x: u32 = columns[0].parse().unwrap();
                let y: u32 = columns[1].parse().unwrap();
                let nx: u32 = columns[2].parse().unwrap();
                let ny: u32 = columns[3].parse().unwrap();
                let flow: u32 = columns[4].parse().unwrap();
                out.push([x, y, nx, ny, flow]);
            }
        }

        out
    }

}

