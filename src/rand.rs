use ::scale::Scale;

pub trait Rng {
    fn next(&mut self) -> f64;
    fn gen_range(&mut self, from: f64, to: f64) -> f64;
}

pub struct StepRng {
    current: u64,
    step: u64
}

impl StepRng {

    pub fn new(start: u64, step: u64) -> StepRng {
        StepRng{current: start, step}
    }
}

impl Rng for StepRng {

    fn next(&mut self) -> f64 {
        return 0.5;
    }
    
    fn gen_range(&mut self, from: f64, to: f64) -> f64 {
        let scale = Scale::new((0.0, 1.0), (from, to));
        scale.scale(self.next()) 
    }

}