use std::f64;
use utils::float_ordering;

use rand::prelude::*;
use scale::Scale;

pub struct Mesh {
    width: i32,
    z: Vec<Vec<f64>>,
    out_of_bounds_z: f64,
}

#[derive(Debug, PartialEq)]
pub struct Split {
    x: i32,
    y: i32,
    z: f64,
}

#[derive(Debug, PartialEq)]
struct SplitRule {
    x: i32,
    y: i32,
    range: (f64, f64),
}

#[derive(Debug, PartialEq)]
struct SplitProcess {
    split_rules: Vec<SplitRule>,
    splits: Vec<Split>,
}

impl SplitRule {
    fn generate_split<R: Rng>(&self, rng: &mut Box<R>, random_range: (f64, f64)) -> Split {
        let r: f64 = rng.gen_range(random_range.0, random_range.1);
        let scale: Scale = Scale::new((0.0, 1.0), self.range);
        Split {
            x: self.x,
            y: self.y,
            z: scale.scale(r),
        }
    }
}

impl SplitProcess {
    fn next<R: Rng>(mut self, rng: &mut Box<R>, random_range: (f64, f64)) -> SplitProcess {
        fn update_rule(rule: SplitRule, split: &Split) -> SplitRule {
            if rule.x == split.x || rule.y == split.y {
                SplitRule {
                    x: rule.x,
                    y: rule.y,
                    range: (split.z.min(rule.range.0), rule.range.1),
                }
            } else {
                rule
            }
        }

        let split = self.split_rules[0].generate_split(rng, random_range);
        self.split_rules.remove(0);
        self.split_rules = self
            .split_rules
            .into_iter()
            .map(|rule| update_rule(rule, &split))
            .collect();
        self.splits.push(split);
        self
    }
}

impl Mesh {
    pub fn new(width: i32, out_of_bounds_z: f64) -> Mesh {
        Mesh {
            width,
            z: vec![vec![0.0; width as usize]; width as usize],
            out_of_bounds_z,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_z_in_bounds(&self, x: i32, y: i32) -> f64 {
        self.z[x as usize][y as usize]
    }

    pub fn get_z_vector(&self) -> &Vec<Vec<f64>> {
        &self.z
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.width
    }

    pub fn get_z(&self, x: i32, y: i32) -> f64 {
        if self.in_bounds(x, y) {
            self.get_z_in_bounds(x, y)
        } else {
            self.out_of_bounds_z
        }
    }

    pub fn set_z(&mut self, x: i32, y: i32, z: f64) {
        self.z[x as usize][y as usize] = z;
    }

    pub fn set_z_vector(&mut self, z: Vec<Vec<f64>>) {
        self.z = z;
    }

    pub fn get_min_z(&self) -> f64 {
        *self
            .z
            .iter()
            .map(|column| column.iter().min_by(float_ordering).unwrap())
            .min_by(float_ordering)
            .unwrap()
    }

    pub fn get_max_z(&self) -> f64 {
        *self
            .z
            .iter()
            .map(|column| column.iter().max_by(float_ordering).unwrap())
            .max_by(float_ordering)
            .unwrap()
    }

    fn init_split_process(&self, x: i32, y: i32) -> SplitProcess {
        const OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

        let mut split_rules: Vec<SplitRule> = OFFSETS
            .iter()
            .map(|o| {
                let dx: i32 = (o.0 as i32 * 2) - 1;
                let dy: i32 = (o.1 as i32 * 2) - 1;
                let z = self.get_z(x, y);
                let zs = [
                    self.get_z(x + dx, y),
                    self.get_z(x, y + dy),
                    self.get_z(x + dx, y + dy),
                    z,
                ];
                let min_z = zs.iter().min_by(float_ordering).unwrap();

                SplitRule {
                    x: x * 2 + o.0,
                    y: y * 2 + o.1,
                    range: (*min_z, z),
                }
            })
            .collect();

        split_rules.sort_by(|a, b| a.range.0.partial_cmp(&b.range.0).unwrap());

        SplitProcess {
            split_rules,
            splits: Vec::with_capacity(4),
        }
    }

    fn split<R: Rng>(
        &self,
        x: i32,
        y: i32,
        rng: &mut Box<R>,
        random_range: (f64, f64),
    ) -> Vec<Split> {
        let mut process: SplitProcess = self.init_split_process(x, y);

        while !process.split_rules.is_empty() {
            process = process.next(rng, random_range);
        }

        process.splits
    }

    fn split_all<R: Rng>(&self, rng: &mut Box<R>, random_range: (f64, f64)) -> Vec<Split> {
        let mut out = Vec::with_capacity((self.width * self.width * 4) as usize);
        for x in 0..self.width {
            for y in 0..self.width {
                out.append(&mut self.split(x, y, rng, random_range));
            }
        }
        out
    }

    pub fn next<R: Rng>(&self, rng: &mut Box<R>, random_range: (f64, f64)) -> Mesh {
        let mut out = Mesh::new(self.width * 2, self.out_of_bounds_z);
        for split in self.split_all(rng, random_range) {
            out.set_z(split.x, split.y, split.z);
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::rngs::mock::StepRng;
    use std::u64;

    fn get_rng() -> Box<StepRng> {
        Box::new(StepRng::new(u64::MAX / 2 + 1, 0))
    }

    #[test]
    fn test_generate_split() {
        let rule = SplitRule {
            x: 11,
            y: 12,
            range: (0.12, 0.1986),
        };
        let expected = Split {
            x: 11,
            y: 12,
            z: 0.15537,
        };
        assert_eq!(rule.generate_split(&mut get_rng(), (0.1, 0.8)), expected);
    }

    #[test]
    fn test_get_min_z() {
        let mut mesh = Mesh::new(3, 0.0);

        let z = vec![
            vec![0.8, 0.1, 0.3],
            vec![0.9, 0.7, 0.4],
            vec![0.2, 0.5, 0.6],
        ];

        mesh.set_z_vector(z);

        assert_eq!(mesh.get_min_z(), 0.1);
    }

    #[test]
    fn test_get_max_z() {
        let mut mesh = Mesh::new(3, 0.0);

        let z = vec![
            vec![0.8, 0.1, 0.3],
            vec![0.9, 0.7, 0.4],
            vec![0.2, 0.5, 0.6],
        ];

        mesh.set_z_vector(z);

        assert_eq!(mesh.get_max_z(), 0.9);
    }

    #[test]
    fn test_init_split_process() {
        let mut mesh = Mesh::new(3, 0.0);

        let z = vec![
            vec![0.8, 0.3, 0.2],
            vec![0.9, 0.7, 0.4],
            vec![0.1, 0.5, 0.6],
        ];

        mesh.set_z_vector(z);

        let expected = SplitProcess {
            split_rules: vec![
                SplitRule {
                    x: 3,
                    y: 2,
                    range: (0.1, 0.7),
                },
                SplitRule {
                    x: 2,
                    y: 3,
                    range: (0.2, 0.7),
                },
                SplitRule {
                    x: 2,
                    y: 2,
                    range: (0.3, 0.7),
                },
                SplitRule {
                    x: 3,
                    y: 3,
                    range: (0.4, 0.7),
                },
            ],
            splits: vec![],
        };

        assert_eq!(mesh.init_split_process(1, 1), expected);
    }

    #[test]
    fn test_next_split_process() {
        //TODO random_range has misleading name
        let random_range = (0.0, 1.0);

        let process = SplitProcess {
            split_rules: vec![
                SplitRule {
                    x: 0,
                    y: 0,
                    range: (0.1, 0.7),
                },
                SplitRule {
                    x: 1,
                    y: 0,
                    range: (0.2, 0.7),
                },
                SplitRule {
                    x: 0,
                    y: 1,
                    range: (0.5, 0.7),
                },
                SplitRule {
                    x: 1,
                    y: 1,
                    range: (0.5, 0.7),
                },
            ],
            splits: vec![],
        };

        let actual = process.next(&mut get_rng(), random_range);

        let expected = SplitProcess {
            split_rules: vec![
                SplitRule {
                    x: 1,
                    y: 0,
                    range: (0.2, 0.7),
                },
                SplitRule {
                    x: 0,
                    y: 1,
                    range: (0.4, 0.7),
                },
                SplitRule {
                    x: 1,
                    y: 1,
                    range: (0.5, 0.7),
                },
            ],
            splits: vec![Split { x: 0, y: 0, z: 0.4 }],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_split_cell() {
        let mut mesh = Mesh::new(3, 0.0);

        let z = vec![
            vec![0.8, 0.3, 0.2],
            vec![0.9, 0.7, 0.4],
            vec![0.1, 0.5, 0.6],
        ];

        mesh.set_z_vector(z);

        let split_process = mesh.init_split_process(1, 1);

        let mut rng = get_rng();
        let random_range = (0.1, 0.5);

        let expected = split_process
            .next(&mut rng, random_range)
            .next(&mut rng, random_range)
            .next(&mut rng, random_range)
            .next(&mut rng, random_range)
            .splits;

        assert_eq!(mesh.split(1, 1, &mut rng, random_range), expected);
    }

    #[test]
    fn test_next() {
        let mut mesh = Mesh::new(2, 0.0);

        let z = vec![vec![0.1, 0.2], vec![0.3, 0.4]];

        mesh.set_z_vector(z);

        let mut rng = get_rng();
        let random_range = (0.1, 0.5);

        mesh.split(0, 1, &mut rng, random_range);

        let next = mesh.next(&mut rng, random_range);

        fn check_splits(mesh: &Mesh, splits: Vec<Split>) {
            for split in splits {
                assert_eq!(mesh.get_z(split.x, split.y), split.z);
            }
        }

        check_splits(&next, mesh.split(0, 0, &mut rng, random_range));
        check_splits(&next, mesh.split(0, 1, &mut rng, random_range));
        check_splits(&next, mesh.split(1, 0, &mut rng, random_range));
        check_splits(&next, mesh.split(1, 1, &mut rng, random_range));
    }

    #[test]
    fn next_mesh_should_retain_downhill_property() {}

}
