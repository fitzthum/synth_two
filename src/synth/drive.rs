use crate::synth::delay::Delay;
//use std::f32::consts;

pub struct Drive {
    delay: Delay,
}

impl Drive {
    pub fn new() -> Self {
        Self {
            delay: Delay::new(200, 0.0),
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        //consts::TAU * (sample * self.steepness).atan()
        (sample * self.delay.process(sample)).tanh()
    }

}
