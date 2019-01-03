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

#[derive(Debug, PartialEq)]
pub struct Downhill {
    width: i32,
    directions: Vec<Vec<[bool; 8]>>,
}

impl Downhill {
    pub fn new(mesh: &Mesh) -> Downhill {
        let mut out = Downhill{
            width: mesh.get_width(),
            directions: vec![vec![[false; 8]; mesh.get_width() as usize]; mesh.get_width() as usize],
        };
        out.compute_all_directions(mesh);
        out
    }

    fn get_directions(&self, x: i32, y: i32) -> [bool; 8] {
        self.directions[x as usize][y as usize]
    }

    fn set_directions(&mut self, x: i32, y: i32, directions: [bool; 8]) {
        self.directions[x as usize][y as usize] = directions;
    }

    fn compute_directions(mesh: &Mesh, x: i32, y: i32) -> [bool; 8] {
        let z = mesh.get_z(x, y);
        let mut out = [false; 8];
        for d in 0..DIRECTIONS.len() {
            let dx = DIRECTIONS[d].0;
            let dy = DIRECTIONS[d].1;
            out[d] = mesh.get_z(x + dx, y + dy) < z;
        }
        out
    }

    fn compute_all_directions(&mut self, mesh: &Mesh) {
        for x in 0..mesh.get_width() {
            for y in 0..mesh.get_width() {
                let directions = Downhill::compute_directions(mesh, x, y);
                self.set_directions(x, y, directions);
            }
        }
    }

    fn cell_has_downhill(&self, x: i32, y: i32) -> bool {
        for downhill in self.get_directions(x, y).iter() {
            if *downhill {
                return true;
            }
        }
        return false;
    }

    pub fn all_cells_have_downhill(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.width {
                if !self.cell_has_downhill(x, y) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compute_directions() {
        let mut mesh = Mesh::new(3, 0.0);
        mesh.set_z_vector(vec![
            vec![0.1, 0.8, 0.2],
            vec![0.3, 0.5, 0.9],
            vec![0.6, 0.4, 0.7],
        ]);

        let expected = [false, true, true, false, true, false, false, true];
        let actual = Downhill::compute_directions(&mesh, 1, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compute_all_directions() {

        let mut mesh = Mesh::new(2, 0.0);
        mesh.set_z_vector(vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4],
        ]);

        let expected = Downhill{
            width: 2,
            directions: vec![
                vec![
                    [true, true, true, true, false, false, false, true],
                    [true, true, true, false, false, true, true, true]
                ],
                vec![
                    [true, true, true, true, true, true, false, true],
                    [true, true, true, true, true, true, true, true]
                ]
            ],
        };

        let actual = Downhill::new(&mesh);

        assert_eq!(actual, expected);
    }

     #[test]
    fn test_all_cells_have_downhill() {
        let mut mesh = Mesh::new(3, 0.0);
        mesh.set_z_vector(vec![
            vec![0.1, 0.8, 0.2],
            vec![0.3, 0.5, 0.9],
            vec![0.6, 0.4, 0.7],
        ]);
        let downhill = Downhill::new(&mesh);

        assert_eq!(downhill.all_cells_have_downhill(), true);
    }

     #[test]
    fn test_not_all_cells_have_downhill() {
        let mut mesh = Mesh::new(3, 0.0);
        mesh.set_z_vector(vec![
            vec![0.5, 0.8, 0.2],
            vec![0.3, 0.1, 0.9],
            vec![0.6, 0.4, 0.7],
        ]);
        let downhill = Downhill::new(&mesh);

        assert_eq!(downhill.all_cells_have_downhill(), false);
    }
}
