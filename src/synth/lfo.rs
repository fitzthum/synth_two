use crate::synth::oscillator::{Oscillator, WaveTableOscillator};

pub trait Lfo {
    fn tick(&mut self);
    fn amplitude(&mut self) -> f64;
    fn set_period(&mut self, period: f32);
}

// wrapper around WaveTable turning it into an LFO
pub struct WaveTableLfo {
    sample_count: u64,
    samples_per_cycle: u64,
    sample_rate: f64,
    oscillator: WaveTableOscillator,
    amplitude: Option<f64>,
}

impl WaveTableLfo {
    pub fn new(sample_rate: f64, period: f32) -> Self {
        let time_per_sample = 1.0 / sample_rate;
        let frequency = 1.0 / period;
        let osc = WaveTableOscillator::new(frequency.into(), time_per_sample);

        Self {
            sample_count: 0,
            samples_per_cycle: 0,
            sample_rate,
            oscillator: osc,
            amplitude: None,
        }

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
        self.oscillator.update_frequency(frequency.into());
        
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
    
}