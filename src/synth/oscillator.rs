
use std::f64::consts::PI;

// Time from note on to sample being processed
pub trait Oscillator {
    // different oscillators will have different init logic
	// so don't define anything here

	// an oscillator is always linked to a voice, 
	// so the frequency can't be adjusted between samples
    fn process(&self, time: f64) -> f64;
}

pub struct SineOscillator {
    frequency: f64,
}

impl SineOscillator {
    pub fn new(frequency: f64) -> Self {
        Self { frequency }
    }
}

impl Oscillator for SineOscillator {
    fn process(&self, time: f64) -> f64 {
        (time * self.frequency * PI * 2.0).sin()
    }
}
