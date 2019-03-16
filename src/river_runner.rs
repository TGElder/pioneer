use mesh::Mesh;
use mesh_splitter::MeshSplitter;
use erosion::Erosion;
use downhill_map::DownhillMap;
use single_downhill_map::{SingleDownhillMap, RandomDownhillMap};
use flow_map::FlowMap;
use isometric::graphics::drawing::terrain::River;
use scale::Scale;
use downhill_map::DIRECTIONS;
use rand::prelude::*;

pub fn get_rivers <R: Rng> (mesh: &Mesh, threshold: usize, sea_level: f64, rng: &mut Box<R>) -> Vec<River> {
    let downhill_map = DownhillMap::new(&mesh);
    let random_downhill_map: Box<SingleDownhillMap> = Box::new(RandomDownhillMap::new(&downhill_map, &mut rng));

    get_rivers_from_downhill_map(mesh, &random_downhill_map)
}

fn get_rivers_from_downhill_map(mesh: &Mesh, downhill_map: &Box<SingleDownhillMap>) -> Vec<River> {
   let flow_map = FlowMap::from(&mesh, &downhill_map);
   get_rivers_from_flow_map(&mesh, &downhill_map, &flow_map)
}

fn get_rivers_from_flow_map(mesh: &Mesh, downhill_map: &Box<SingleDownhillMap>, flow_map: &FlowMap) -> Vec<River> {

    let mut include: na::DMatrix<bool> = na::DMatrix::from_element(mesh.get_width() as usize, mesh.get_width() as usize, false);

    let sea_level = 2.0;
    for x in 0..mesh.get_width() {
        for y in 0..mesh.get_width() {
            let flow = flow_map.get_flow(x, y);
            if flow > 256 && mesh.get_z(x, y) >= sea_level {
                include[(x as usize, y as usize)] = true;
                let direction = downhill_map.get_direction(x, y);
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
                let direction = downhill_map.get_direction(x, y);
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

    rivers
}

#[cfg(test)]
mod tests {

    use super::*;
    use single_downhill_map::MockDownhillMap;

    #[test]
    pub fn test_get_rivers_from_flow_map() {
        let mesh = Mesh::new(4, 0.0);
        let z = na::DMatrix::from_row_slice(4, 4, &[
            0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
        ]);
        mesh.set_z_vector(z);

        let downhill_map = vec![
            vec![3, 3, 3, 3],
            vec![3, 3, 3, 3],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0]
        ];
        let downhill_map = MockDownhillMap::new(downhill_map);
        let downhill_map: Box<SingleDownhillMap> = Box::new(downhill_map);

        let flow_map = FlowMap{
            width: 4,
            flow: na::DMatrix::from_row_slice(4, 4, &[
                1, 2, 3, 4,
                3, 6, 9, 12,
                2, 2, 2, 2,
                1, 1, 1, 1
            ]),
        };

        let rivers = get_rivers_from_flow_map(&mesh, &downhill_map, &flow_map);

        assert!(rivers.contains(&River::new(na::Vector2::new(3, 0), na::Vector2::new(2, 0), 4.0)));
        assert!(rivers.contains(&River::new(na::Vector2::new(2, 0), na::Vector2::new(1, 0), 3.0)));
        assert!(rivers.contains(&River::new(na::Vector2::new(3, 1), na::Vector2::new(2, 1), 12.0)));
        assert!(rivers.contains(&River::new(na::Vector2::new(2, 1), na::Vector2::new(1, 1), 9.0)));
    }
}