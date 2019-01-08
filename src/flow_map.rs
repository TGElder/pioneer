use mesh::Mesh;
use single_downhill_map::SingleDownhillMap;
use downhill_map::DIRECTIONS;

pub struct FlowMap {
    width: i32,
    flow: Vec<Vec<u32>>,
}

impl FlowMap {

    fn rain_on(&mut self, mesh: &Mesh, downhill_map: Box<SingleDownhillMap>, x: i32, y: i32) {
        let mut focus = (x, y);
        while mesh.in_bounds(focus.0, focus.1) {
            self.flow[x as usize][y as usize] += 1;
            let direction = downhill_map.get_direction(focus.0, focus.1);
            focus = (focus.0 + DIRECTIONS[direction].0, focus.1 + DIRECTIONS[direction].1);
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_rain_on() {
        let mesh = Mesh::new(4, 0.0);

        let directions = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]
    }
}