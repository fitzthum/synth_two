// Some kind of reverb thing hopefully
//

use crate::synth::filter::{Biquad, BiquadCoefficients};

const BUFFER_SIZE: i32 = 100000;

pub struct Reverb {
    apf: Biquad<f32>,
    delay: Delay, 
}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        
        // no idea how to set these
        let frequency = 1000.0;
        let q = 2.0;
        let delay_samples = 6000;
        let feedback_level = 0.3;
        
        let mut apf = Biquad::default();
        apf.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        Self {
            apf,
            delay: Delay::new(delay_samples, feedback_level),
        }

    }

    // only returns the wet signal
    // or do we need to combine this with the original (i think so)
    pub fn process(&mut self, sample: f32) -> f32 {
        //0.5 * sample + 0.5 * self.apf.process(self.delay.process(sample)) 
        let wet = self.delay.process(self.apf.process(sample));
        wet * 0.5 + sample * 0.5
        //self.apf.process(sample) 

    }
}


pub struct Delay {
    buffer: [f32; BUFFER_SIZE as usize],
    buffer_index: i32,
    delay_samples: i32,
    feedback_level: f32,

}

impl Delay {
    pub fn new(delay_samples: i32, feedback_level: f32) -> Self {
        Self {
            // probably makes sense to start with a buffer of 0.0
            buffer: [0.0; BUFFER_SIZE as usize],
            buffer_index: 0,
            delay_samples,
            feedback_level,
        }

    }

    pub fn process(&mut self, sample: f32) -> f32 {
        // get the previous sample
        let out = self.buffer[self.buffer_bounds(self.buffer_index - self.delay_samples)];

        // update the buffer
        self.buffer[self.buffer_bounds(self.buffer_index)] = sample + out * self.feedback_level;
        self.buffer_index = self.buffer_bounds(self.buffer_index + 1) as i32; 
        
        out

    }

    fn buffer_bounds(&self, index: i32) -> usize {
        (((index % BUFFER_SIZE) + BUFFER_SIZE) % BUFFER_SIZE) as usize

    }
}
