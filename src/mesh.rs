use std::f64;
use utils::float_ordering;

use rand::prelude::*;
use ::scale::Scale;

const MAX_VALUE: f64 = f64::MAX;
const MIN_VALUE: f64 = f64::MIN;
const dx: [i8; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const dy: [i8; 8] = [0, -1, -1, -1, 0, 1, 1, 1];

pub struct Mesh {
    width: i32,
    z: Vec<Vec<f64>>,
}

#[derive(Debug, PartialEq)]
pub struct Split {
    offset_x: i8,
    offset_y: i8,
    z: f64
}

struct SplitRule {
    offset_x: i8,
    offset_y: i8,
    range: (f64, f64)
}

impl SplitRule {

    fn generate_split<R: Rng> (&self, rng: &mut Box<R>, random_range: (f64, f64)) -> Split
    {
        let r: f64 = rng.gen_range(random_range.0, random_range.1);
        let scale: Scale = Scale::new((0.0, 1.0), self.range);
        Split {
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            z: scale.scale(r)
        }
    }
}

impl Mesh {

    pub fn new(width: i32) -> Mesh {
        Mesh{
            width,
            z: vec![vec![0.0; width as usize]; width as usize]
        }
    }

    pub fn get_z(&self, x: i32, y: i32) -> f64 {
        self.z[x as usize][y as usize]
    }

    pub fn get_z_vector(&self) -> &Vec<Vec<f64>> {
        &self.z
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.width
    }

    pub fn get_z_or_default(&self, x: i32, y: i32, default: f64) -> f64 {
        if self.in_bounds(x, y) {
            self.get_z(x, y)
        } else {
            default
        }
    }

    pub fn set_z(&mut self, x: i32, y: i32, z: f64) {
        self.z[x as usize][y as usize] = z;
    }

    pub fn set_z_vector(&mut self, z: Vec<Vec<f64>>) {
        self.z = z;
    }


    pub fn get_min_z(&self) -> f64 {
        *self.z.iter()
            .map(|column| column.iter()
                .min_by(float_ordering).unwrap())
            .min_by(float_ordering).unwrap()
    }

    pub fn get_max_z(&self) -> f64 {
        *self.z.iter()
            .map(|column| column.iter()
                .max_by(float_ordering).unwrap())
            .max_by(float_ordering).unwrap()
    }

  
}


#[cfg(test)]
mod tests {

    use std::u64;
    use ::mesh::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_generate_split() {
        let rng = StepRng::new(u64::MAX / 2 + 1, 0);
        let rule = SplitRule{
            offset_x: 1,
            offset_y: -1,
            range: (0.12, 0.1986)
        };
        let expected = Split{
            offset_x: 1,
            offset_y: -1,
            z: 0.15537
        };
        assert_eq!(rule.generate_split(&mut Box::new(rng), (0.1, 0.8)),expected);

    }

    #[test]
    fn test_get_min_z() {
        let mut mesh = Mesh::new(3);

        let z = vec![
            vec![0.8, 0.1, 0.3],
            vec![0.9, 0.7, 0.4],
            vec![0.2, 0.5, 0.6]
        ];

        mesh.set_z_vector(z);

        assert_eq!(mesh.get_min_z(), 0.1);
    }

    #[test]
    fn test_get_max_z() {
        let mut mesh = Mesh::new(3);

        let z = vec![
            vec![0.8, 0.1, 0.3],
            vec![0.9, 0.7, 0.4],
            vec![0.2, 0.5, 0.6]
        ];

        mesh.set_z_vector(z);

        assert_eq!(mesh.get_max_z(), 0.9);
    }

}

// class Mesh {

//   static final double MAX_VALUE = Double.MAX_VALUE;
//   static final double MIN_VALUE = Double.MIN_VALUE;
    
//   static final short [] dx8 = {-1, -1, 0, 1, 1, 1, 0, -1};
//   static final short [] dy8 = {0, -1, -1, -1, 0, 1, 1, 1};

//   //static final short [] dx4 = {-1, 0, 1, 0};
//   //static final short [] dy4 = {0, -1, 0, 1};
  
//   @Getter
//   private final int width;
//   private double[][] z;

//   Mesh(int width) {
//     this.width = width;
//     z = new double[width][width];
//   }

//   final double getZ(int x, int y) {
//     return z[x][y];
//   }
  
//   boolean inBounds(int x, int y){
//     return x >= 0 && y >= 0 && x < width && y < width;
//   }
  
//   double[][] getZ() {
//     return z;
//   }
  
//   double getZ(int x, int y, double outOfBoundsValue) {
//     if (inBounds(x, y)) {
//       return getZ(x, y);
//     }
//     else {
//       return outOfBoundsValue;
//     }
//   }
  
//   final void setZ(int x, int y, double value) {
//     z[x][y] = value;
//   }
  
//   void setZ(double[][] values) {
//     z = values;
//   }
  
//   void setZ(double value) {
//     iterate((x, y) -> setZ(x, y, value));
//   }
 
//   double getMinZ() {
//     double out = Mesh.MAX_VALUE;
//     for (int x=0; x<width; x++) {
//       for (int y=0; y<width; y++) {
//         out = Math.min(out, getZ(x, y));
//       }
//     }
//     return out;
//   }
  
//   double getMaxZ() {
//     double out = Mesh.MIN_VALUE;
//     for (int x=0; x<width; x++) {
//       for (int y=0; y<width; y++) {
//         out = Math.max(out, getZ(x, y));
//       }
//     }
//     return out;
//   }
  
//   void iterate(MeshOperation operation) {
//     for (int y = 0; y < width; y++) {
//       for (int x = 0; x < width; x++) {
//         operation.operate(x, y);
//       }
//     }
//   }
  
//   <T extends Throwable> void iterateWithThrows(MeshOperationWithThrows<T> operation) throws T {
//     for (int y = 0; y < width; y++) {
//       for (int x = 0; x < width; x++) {
//         operation.operate(x, y);
//       }
//     }
//   }

//   List<Split> splitCell(int x, int y, RNG rng, Scale scale) {

//     List<SplitRule> splitRules = new ArrayList<>();

//     for (int offsetX = 0; offsetX < 2; offsetX++) {
//       for (int offsetY = 0; offsetY < 2; offsetY++) {
//         int xNeighbour = (offsetX * 2) - 1;
//         int yNeighbour = (offsetY * 2) - 1;
//         double xNeighbourZ = getZ(x + xNeighbour, y, Mesh.MIN_VALUE);
//         double yNeighbourZ = getZ(x, y + yNeighbour, Mesh.MIN_VALUE);
//         double dNeighbourZ = getZ(x + xNeighbour, y + yNeighbour, Mesh.MIN_VALUE);
//         double z = getZ(x, y);

//         double minZ = Stream.of(xNeighbourZ, yNeighbourZ, dNeighbourZ, z).min(Double::compareTo).get();

//         splitRules.add(new SplitRule(offsetX, offsetY, minZ, z));

//       }
//     }

//     List<SplitRule> sorted = splitRules.stream()
//         .sorted(Comparator.comparingDouble(sr -> sr.minZ))
//         .collect(Collectors.toList());

//     List<Split> out = new ArrayList<>();

// //    System.out.println("Starting rules = " + sorted);

//     for (SplitRule rule : sorted) {

//       //System.out.println(rule.minZ);

//       Split split = rule.generateSplit(rng, scale);


//       out.add(split);

//       for (SplitRule other : sorted) {
//         if (other != rule) {
//           if (other.offsetX == rule.offsetX || other.offsetY == rule.offsetY) {
//             other.minZ = Math.min(other.minZ, split.z);

//           }
//         }
//       }
//     }

// //    System.out.println("Final rules = " + sorted);
// //    System.out.println("Final splits = " + out);

//     return out;

//   }

//   @AllArgsConstructor
//   @Data
//   private static class SplitRule {
//     final int offsetX;
//     final int offsetY;
//     double minZ;
//     final double maxZ;

//     private Split generateSplit(RNG rng, Scale scale) {
//       double r = rng.getNext();
//       double range = (maxZ - minZ);
//       double z = minZ + range * scale.scale(r);

//       return new Split(offsetX, offsetY, z);
//     }
//   }

//   @AllArgsConstructor
//   @Data
//   static class Split {
//     final int offsetX;
//     final int offsetY;
//     final double z;
//   }
  
