use crate::synth::oscillator::{Oscillator, WaveTableOscillator};
use std::sync::{Arc, Mutex};

pub trait Lfo {
    fn tick(&mut self);
    fn amplitude(&mut self) -> f64;
    fn set_period(&mut self, period: f32);

    // For populating the graph of the LFO
    fn generate_samples(&self);
}

// wrapper around WaveTable turning it into an LFO
pub struct WaveTableLfo {
    sample_count: u64,
    samples_per_cycle: u64,
    sample_rate: f64,
    oscillator: WaveTableOscillator,
    amplitude: Option<f64>,
    samples: Arc<Mutex<Vec<f32>>>,
}

impl WaveTableLfo {
    pub fn new(sample_rate: f64, period: f32, samples: Arc<Mutex<Vec<f32>>>) -> Self {
        let time_per_sample = 1.0 / sample_rate;
        let frequency = 1.0 / period;
        let osc = WaveTableOscillator::new(frequency.into(), time_per_sample);

        Self {
            sample_count: 0,
            samples_per_cycle: 0,
            sample_rate,
            oscillator: osc,
            amplitude: None,
            samples,
        }
    }

    // This is specific to a WaveTable, so it's not in
    // the LFO trait
    pub fn set_index(&mut self, index: f64) {
        self.oscillator.set_wave_index(index);
    }
}

impl Lfo for WaveTableLfo {
    fn tick(&mut self) {
        self.sample_count += 1;
        if self.sample_count <= self.samples_per_cycle {
            self.sample_count = 0;
        }

        self.amplitude = None;
    }

    fn set_period(&mut self, period: f32) {
        // sets the samples per cycle
        //
        // the frequency is 1/period
        let frequency = 1.0 / period;
        self.oscillator.set_frequency(frequency.into());

        // samples_per_cycle = period * sample_rate
        self.samples_per_cycle = (period as f64 * self.sample_rate) as u64;
    }

    fn amplitude(&mut self) -> f64 {
        if let Some(amplitude) = self.amplitude {
            return amplitude;
        }
        let time_per_sample = 1.0 / self.sample_rate;
        let time = time_per_sample * self.sample_count as f64;
        let amplitude = self.oscillator.process(time.into());

        self.amplitude = Some(amplitude);

        amplitude
    }

    // Generate buffer of samples to show in the editor
    fn generate_samples(&self) {
        let mut graph_samples = vec![];

        // let's try to calculate the minimum to make a decent looking graph
        let num_samples = 512;

        let time_per_sample = 1.0 / 512 as f64;

        for n in 0..num_samples {
            graph_samples.push(self.oscillator.process(n as f64 * time_per_sample) as f32);
        }

        *self.samples.lock().unwrap() = graph_samples;
    }
}
