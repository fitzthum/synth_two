use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod voice;
use voice::Voice;

mod envelope;
mod oscillator;

mod filter;
use filter::{Biquad, BiquadCoefficients};

pub mod spectrum;
use spectrum::SpectrumCalculator;

mod lfo;
use lfo::{Lfo, WaveTableLfo};

use crate::SynthTwoParams;

pub struct Synth {
    sample_rate: f64,
    voices: HashMap<u8, Voice>,
    pub spectrum_calculator: SpectrumCalculator,

    plugin_params: Arc<SynthTwoParams>,

    // these are for the graphs
    envelope: Arc<Mutex<Vec<f32>>>,
    graph_samples: Arc<Mutex<Vec<f32>>>,

    filter: Biquad<f32>,
    lfo1: Option<WaveTableLfo>,
}

impl Synth {
    pub fn default() -> Self {
        Self {
            sample_rate: 1.0,
            voices: HashMap::new(),
            spectrum_calculator: SpectrumCalculator::default(),
            // This seems dumb
            plugin_params: Arc::new(SynthTwoParams::default()),
            envelope: Arc::new(Mutex::new(vec![])),
            graph_samples: Arc::new(Mutex::new(vec![])),

            filter: Biquad::default(),
            lfo1: None,
        }
    }
    pub fn initialize(
        &mut self,
        plugin_params: Arc<SynthTwoParams>,
        sample_rate: f64,
        envelope: Arc<Mutex<Vec<f32>>>,
        graph_samples: Arc<Mutex<Vec<f32>>>,
        spectrum_samples: Arc<Mutex<Vec<f32>>>,
    ) {
        self.sample_rate = sample_rate;
        self.plugin_params = plugin_params;
        self.envelope = envelope;
        self.graph_samples = graph_samples;
        self.spectrum_calculator.set_buffer(spectrum_samples);
        
        // initialize filter from params that we just updated
        self.update_filter();

        let lfo1_period = self.plugin_params.lfo1_period.smoothed.next();
        self.lfo1 = Some(WaveTableLfo::new(self.sample_rate, lfo1_period));
    }

    // we're doing fake stereo at first
    pub fn process_sample(&mut self) -> f32 {
        self.update_components();

        let mut out = 0.0;
        for (_, voice) in self.voices.iter_mut() {
            out += voice.process() as f32;
        }

        self.filter.process(out)
    }

    // any components that need some re-initialization based on param changes
    fn update_components(&mut self) {

        // Filter Parameters
        if self.plugin_params.filter_cutoff.smoothed.is_smoothing() || 
            self.plugin_params.filter_q.smoothed.is_smoothing() {

            self.update_filter();

        }

        // Envelope Parameters
        if self.plugin_params.attack.smoothed.is_smoothing() ||
            self.plugin_params.decay.smoothed.is_smoothing() ||
            self.plugin_params.sustain.smoothed.is_smoothing() ||
            self.plugin_params.release.smoothed.is_smoothing() {

            let mut env = self.envelope.lock().unwrap();
            env[0] = self.plugin_params.attack.smoothed.next();
            env[1] = self.plugin_params.decay.smoothed.next();
            env[2] = self.plugin_params.sustain.smoothed.next();
            env[3] = self.plugin_params.release.smoothed.next();
        }

        // LFO1 Parameters
        if self.plugin_params.lfo1_period.smoothed.is_smoothing() {
            self.lfo1.as_mut().unwrap().set_period(self.plugin_params.lfo1_period.smoothed.next());

        }
    }

    fn update_filter(&mut self) {
        let cutoff = self.plugin_params.filter_cutoff.smoothed.next();
        let q = self.plugin_params.filter_q.smoothed.next();

        let coefficients = BiquadCoefficients::lowpass(self.sample_rate as f32, cutoff, q);
        self.filter.coefficients = coefficients;

    }

    // create a new voice
    pub fn voice_on(&mut self, note: u8, velocity: f32) {
        let time_per_sample = 1.0 / self.sample_rate;
        self.voices.insert(
            note,
            Voice::from_midi(note, velocity, time_per_sample, self.plugin_params.clone()),
        );
    }

    pub fn voice_off(&mut self, note: u8) {
        self.voices
            .get_mut(&note)
            .expect("Turning off non-existent note...")
            .voice_off();
    }

    pub fn reap_voices(&mut self) {
        self.voices.retain(|_, note| !note.finished);
    }
}
