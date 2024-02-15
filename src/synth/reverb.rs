// Some kind of reverb thing hopefully
//

use crate::synth::filter::{Biquad, BiquadCoefficients};

const BUFFER_SIZE: i32 = 100000;

pub struct Reverb {
    sample_rate: f32,

    apf1: Biquad<f32>,
    delay1: Delay,

    apf2: Biquad<f32>,
    delay2: Delay,

    apf3: Biquad<f32>,
    delay3: Delay,

}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        
        // maaaagic numbers
        let frequency = 1000.0;
        let q = 0.05;
        let delay_samples = 6000;
        let feedback_level = 0.4;

        let mut apf1 = Biquad::default();
        apf1.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay1 = Delay::new(delay_samples, feedback_level);


        // a second loop
        let frequency = 700.0;
        let q = 0.05;
        let delay_samples = 2000;
        let feedback_level = 0.2;

        let mut apf2 = Biquad::default();
        apf2.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay2 = Delay::new(delay_samples, feedback_level);


        // a third loop, wait this is synth two....
        let frequency = 1400.0;
        let q = 0.05;
        let delay_samples = 400;
        let feedback_level = 0.6;

        let mut apf3 = Biquad::default();
        apf3.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay3 = Delay::new(delay_samples, feedback_level);


        Self {
            sample_rate,
            apf1,
            delay1,
            apf2,
            delay2,
            apf3,
            delay3,
        }

    }

    pub fn update(&mut self, delay_samples_new: i32, feedback_level_new: f32, frequency_new: f32, q_new: f32) {

        let frequency = frequency_new;
        let q = q_new;
        let delay_samples = delay_samples_new + 4000;
        let feedback_level = feedback_level_new;

        self.apf1.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay1.update(delay_samples, feedback_level);


        // a second loop
        let frequency = frequency_new - 200.0;
        let q = q_new;
        let delay_samples = delay_samples_new;
        let feedback_level = feedback_level_new - 0.2;

        self.apf2.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay2.update(delay_samples, feedback_level);


        // a third loop, wait this is synth two....
        let frequency = frequency_new + 200.0;
        let q = q_new;
        let delay_samples = delay_samples_new - 1600;
        let feedback_level = feedback_level_new + 0.2;

        self.apf3.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay3.update(delay_samples, feedback_level);

    }

    pub fn process(&mut self, sample: f32) -> (f32, f32) {

        let delayed = self.delay1.process(sample);
        let wet = 0.5 * self.apf1.process(delayed) + 0.5 * delayed;

        let delayed = self.delay2.process(wet);
        let wet = 0.5 * self.apf2.process(delayed) + 0.5 * delayed;

        let delayed = self.delay3.process(wet);
        let wet = 0.5 * self.apf3.process(delayed) + 0.5 * delayed;

        (wet, wet)

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

    pub fn update(&mut self, delay_samples: i32, feedback_level: f32) {
        self.delay_samples = delay_samples;
        self.feedback_level = feedback_level;

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
