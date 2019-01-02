extern crate rand;

pub mod downhill;
pub mod mesh;
pub mod scale;
pub mod utils;

fn main() {

    // use rand::rngs::mock::StepRng;
    // use std::u64;

    // fn get_rng() -> Box<StepRng> {
    //     Box::new(StepRng::new(u64::MAX / 2 + 1, 0))
    // }

    // let mut mesh = Mesh::new(2, 0.0);
    // mesh.set_z_vector(vec![
    //     vec![100.0, 200.0],
    //     vec![300.0, 400.0]
    // ]);

    // let mut rng = get_rng();
    // let range = (0.1, 0.9);

    // for i in 0..14 {

    //     mesh = mesh.next(&mut rng, range);
    //     println!("{}", mesh.get_width());
    // }

}
