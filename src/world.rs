extern crate image;

use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
pub struct World {
    pub heightmap: Heightmap,
    pub sea_level: f64,
    pub rivers: Vec<[u32; 5]>
}

impl World {

    pub fn new(heightmap: Heightmap, sea_level: f64, rivers: Vec<[u32; 5]>) -> World {
        World{heightmap, sea_level, rivers}
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

#[derive(Clone, Debug)]
pub struct Heightmap {
    pub width: u32,
    pub height: u32,
    values: Vec<Vec<f64>>
}

impl Heightmap {

    pub const MAX_HEIGHT: f64 = 2048.0;

    pub fn new(width: u32, height: u32) -> Heightmap {
        let values: Vec<Vec<f64>> = vec![vec![0.0; height as usize]; width as usize];
        Heightmap{width, height, values}
    }

    pub fn get(&self, x: &u32, y: &u32) -> f64 {
        self.values[*x as usize][*y as usize]
    }

    pub fn set(&mut self, x: &u32, y: &u32, z: f64) {
        self.values[*x as usize][*y as usize] = z;
    }

    pub fn from_grayscale_image(file: &str) -> Heightmap {

        use self::image::open;
        use self::image::GenericImage;

        let image = open(file).unwrap();
        let width: u32 = image.dimensions().0;
        let height: u32 = image.dimensions().1;

        let mut out: Heightmap = Heightmap::new(width, height);

        for (x, y, pixel) in image.pixels() {
            out.set(&x, &y, pixel.data[0] as f64);
        }

        out
    }

    pub fn from_csv_file(file: &str) -> Heightmap {
        let mut f = File::open(file).expect("File not found");
        let mut text = String::new();
        f.read_to_string(&mut text).expect("Failed to read file");

        let width = text.split("\n").nth(0).unwrap().split(",").count() as u32;
        let height = text.split("\n").count() as u32;

        let mut out: Heightmap = Heightmap::new(width, height);

        let mut y = 0;
        for row in text.split("\n") {
            if row.len() > 0 {
                let mut x = 0;
                for cell in row.split(",") {
                    let height: f64 = cell.parse().unwrap();
                    out.set(&x, &y, height / 2048.0);
                    x += 1;
                }
                y += 1;
            }
        }

        out
    }

}

