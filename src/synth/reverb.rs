// Some kind of reverb thing hopefully
//

use crate::synth::filter::{Biquad, BiquadCoefficients};
use crate::synth::delay::Delay;


pub struct Reverb {
    sample_rate: f32,

    apf1_l: Biquad<f32>,
    apf1_r: Biquad<f32>,
    delay1_l: Delay,
    delay1_r: Delay,

    apf2_l: Biquad<f32>,
    apf2_r: Biquad<f32>,
    delay2_l: Delay,
    delay2_r: Delay,

    apf3_l: Biquad<f32>,
    apf3_r: Biquad<f32>,
    delay3_l: Delay,
    delay3_r: Delay,

}

impl Reverb {
    pub fn new(sample_rate: f32) -> Self {
        
        // maaaagic numbers
        // actually these are just placeholders
        // the real values will be set the first time we call update() :'(
        let frequency = 8000.0;
        let q = 0.5;
        let delay_samples = 2000;
        let feedback_level = 0.4;

        let mut apf1_l = Biquad::default();
        let mut apf1_r = Biquad::default();
        apf1_l.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);
        apf1_r.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay1_l = Delay::new(delay_samples, feedback_level);
        let delay1_r = Delay::new(delay_samples, feedback_level);

        let mut apf2_l = Biquad::default();
        let mut apf2_r = Biquad::default();
        apf2_l.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);
        apf2_r.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay2_l = Delay::new(delay_samples, feedback_level);
        let delay2_r = Delay::new(delay_samples, feedback_level);

        let mut apf3_l = Biquad::default();
        let mut apf3_r = Biquad::default();
        apf3_l.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);
        apf3_r.coefficients = BiquadCoefficients::allpass(sample_rate, frequency, q);

        let delay3_l = Delay::new(delay_samples, feedback_level);
        let delay3_r = Delay::new(delay_samples, feedback_level);


        Self {
            sample_rate,
            apf1_l,
            apf1_r,
            delay1_l,
            delay1_r,
            apf2_l,
            apf2_r,
            delay2_l,
            delay2_r,
            apf3_l,
            apf3_r,
            delay3_l,
            delay3_r,
        }

    }

    pub fn update(&mut self, delay_samples_new: i32, feedback_level_new: f32, frequency_new: f32, q_new: f32) {

        let frequency = frequency_new;
        let q = q_new;
        let delay_samples = delay_samples_new + 4000;
        let feedback_level = feedback_level_new;

        self.apf1_l.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.apf1_r.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay1_l.update(delay_samples, feedback_level);
        self.delay1_r.update(delay_samples, feedback_level);


        // a second loop
        let frequency = frequency_new - 200.0;
        let q = q_new;
        let delay_samples = delay_samples_new;
        let feedback_level = feedback_level_new - 0.2;

        self.apf2_l.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.apf2_r.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay2_l.update(delay_samples, feedback_level);
        self.delay2_r.update(delay_samples, feedback_level);


        // a third loop, wait this is synth two....
        let frequency = frequency_new + 200.0;
        let q = q_new;
        let delay_samples = delay_samples_new - 1600;
        let feedback_level = feedback_level_new + 0.2;

        self.apf3_l.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.apf3_r.coefficients = BiquadCoefficients::allpass(self.sample_rate, frequency, q);
        self.delay3_l.update(delay_samples, feedback_level);
        self.delay3_r.update(delay_samples, feedback_level);

    }

    pub fn process(&mut self, sample: f32) -> (f32, f32) {

        let delayed_l = self.delay1_l.process(sample);
        let delayed_r = self.delay3_r.process(sample);
        let wet_l = 0.5 * self.apf1_l.process(delayed_l) + 0.5 * delayed_l;
        let wet_r = 0.5 * self.apf3_r.process(delayed_r) + 0.5 * delayed_r;

        let delayed_l = self.delay2_l.process(wet_l);
        let delayed_r = self.delay2_r.process(wet_r);
        let wet_l = 0.5 * self.apf2_l.process(delayed_l) + 0.5 * delayed_l;
        let wet_r = 0.5 * self.apf2_r.process(delayed_r) + 0.5 * delayed_r;

        let delayed_l = self.delay3_l.process(wet_l);
        let delayed_r = self.delay1_r.process(wet_r);
        let wet_l = 0.5 * self.apf3_l.process(delayed_l) + 0.5 * delayed_l;
        let wet_r = 0.5 * self.apf1_r.process(delayed_r) + 0.5 * delayed_r;

        (wet_l, wet_r)

    }
}
