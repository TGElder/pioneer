extern crate image;



#[derive(Clone, Debug)]
pub struct World {
    pub heightmap: Heightmap,
    pub sea_level: u32
}

impl World {

    pub fn new(heightmap: Heightmap, sea_level: u32) -> World {
        World{heightmap, sea_level}
    }

}

#[derive(Clone, Debug)]
pub struct Heightmap {
    pub width: u32,
    pub height: u32,
    values: Vec<Vec<u32>>
}

impl Heightmap {

    pub const MAX_HEIGHT: u32 = 256;

    pub fn new(width: u32, height: u32) -> Heightmap {
        let values: Vec<Vec<u32>> = vec![vec![0; height as usize]; width as usize];
        Heightmap{width, height, values}
    }

    pub fn get(&self, x: &u32, y: &u32) -> u32 {
        self.values[*x as usize][*y as usize]
    }

    pub fn set(&mut self, x: &u32, y: &u32, z: u32) {
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
            out.set(&x, &y, pixel.data[0] as u32);
        }

        out
    }

}