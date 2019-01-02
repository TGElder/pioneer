use mesh::Mesh;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

pub struct Downhill {
    downhill: Vec<Vec<[bool; 8]>>,
}

impl Downhill {
    fn new(mesh: &Mesh) {}

    fn get_downhill_directions(mesh: &Mesh, x: i32, y: i32) -> [bool; 8] {
        let z = mesh.get_z(x, y);
        let mut out = [false; 8];
        for d in 0..DIRECTIONS.len() {
            let dx = DIRECTIONS[d].0;
            let dy = DIRECTIONS[d].1;
            out[d] = mesh.get_z(x + dx, y + dy) < z;
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_downhill_directions() {
        let mut mesh = Mesh::new(3, 0.0);
        mesh.set_z_vector(vec![
            vec![0.1, 0.8, 0.2],
            vec![0.3, 0.5, 0.9],
            vec![0.6, 0.4, 0.7],
        ]);

        let expected = [false, true, true, false, true, false, false, true];
        let actual = Downhill::get_downhill_directions(&mesh, 1, 1);

        assert_eq!(actual, expected);
    }
}
