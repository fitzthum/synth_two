use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod voice;
use voice::Voice;

mod envelope;
mod oscillator;
mod delay;

mod filter;
use filter::{Biquad, BiquadCoefficients};

pub mod spectrum;
use spectrum::SpectrumCalculator;

mod lfo;
use lfo::{Lfo, WaveTableLfo};

mod reverb;
use reverb::Reverb;

mod drive;
use drive::Drive;

use crate::params::{FILTER_CUTOFF_MAX, FILTER_CUTOFF_MIN};
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
    lfo1: Option<Arc<Mutex<WaveTableLfo>>>,
    reverb: Option<Reverb>,
    drive: Option<Drive>,
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
            reverb: None,
            drive: None,
        }
    }
    pub fn initialize(
        &mut self,
        plugin_params: Arc<SynthTwoParams>,
        sample_rate: f64,
        envelope: Arc<Mutex<Vec<f32>>>,
        graph_samples: Arc<Mutex<Vec<f32>>>,
        spectrum_samples: Arc<Mutex<Vec<f32>>>,
        lfo1_samples: Arc<Mutex<Vec<f32>>>,
    ) {
        self.sample_rate = sample_rate;
        self.plugin_params = plugin_params;
        self.envelope = envelope;
        self.graph_samples = graph_samples;
        self.spectrum_calculator.set_buffer(spectrum_samples);

        // initialize filter from params that we just updated
        self.update_filter();

        let lfo1_period = self.plugin_params.lfo1_period.smoothed.next();
        self.lfo1 = Some(Arc::new(Mutex::new(WaveTableLfo::new(
            self.sample_rate,
            lfo1_period,
            lfo1_samples,
        ))));

        self.lfo1.as_mut().unwrap().lock().unwrap().generate_samples();

        self.reverb = Some(Reverb::new(sample_rate as f32));
        self.drive = Some(Drive::new());
    }

    // we're doing fake stereo at first
    pub fn process_sample(&mut self) -> (f32, f32) {
        self.update_components();

        let mut out = 0.0;
        for (_, voice) in self.voices.iter_mut() {
            out += voice.process() as f32;
        }

        let out = self.filter.process(out);

        // drive
        let mut drive_level = self.plugin_params.drive_level.smoothed.next();
        let drive_lfo = self.plugin_params.drive_lfo.smoothed.next();
        if drive_lfo > 0.0 {
            let lfo_value = self.lfo1.as_mut().unwrap().lock().unwrap().amplitude() as f32;
            drive_level = (drive_level + lfo_value * drive_level).min(1.0).max(0.0);

        }
        let out = out * (1.0 - drive_level) + drive_level * self.drive.as_mut().unwrap().process(out);

        let (reverb_l, reverb_r) = self.reverb.as_mut().unwrap().process(out);

        let out_l = out + reverb_l * self.plugin_params.reverb_volume.smoothed.next();
        let out_r = out + reverb_r * self.plugin_params.reverb_volume.smoothed.next();

        (out_l, out_r)
    }

    // any components that need some re-initialization based on param changes
    fn update_components(&mut self) {
        if let Some(lfo1) = self.lfo1.as_mut() {
            lfo1.lock().unwrap().tick();
        }

        // Filter Parameters
        if self.plugin_params.filter_cutoff.smoothed.is_smoothing()
            || self.plugin_params.filter_q.smoothed.is_smoothing()
            || self.plugin_params.filter_lfo_strength.smoothed.next() > 0.0
        {
            self.update_filter();
        }

        // Envelope Parameters
        if self.plugin_params.attack.smoothed.is_smoothing()
            || self.plugin_params.decay.smoothed.is_smoothing()
            || self.plugin_params.sustain.smoothed.is_smoothing()
            || self.plugin_params.release.smoothed.is_smoothing()
        {
            let mut env = self.envelope.lock().unwrap();
            env[0] = self.plugin_params.attack.smoothed.next();
            env[1] = self.plugin_params.decay.smoothed.next();
            env[2] = self.plugin_params.sustain.smoothed.next();
            env[3] = self.plugin_params.release.smoothed.next();
        }

        // LFO1 Parameters
        if self.plugin_params.lfo1_period.smoothed.is_smoothing() {
            if let Some(lfo1) = self.lfo1.as_mut() {
                let mut lfo = lfo1.lock().unwrap();
                lfo.set_period(self.plugin_params.lfo1_period.smoothed.next());
                lfo.generate_samples();
            }
        }
        if self.plugin_params.lfo1_index.smoothed.is_smoothing() {
            if let Some(lfo1) = self.lfo1.as_mut() {
                let mut lfo = lfo1.lock().unwrap();
                lfo.set_index(self.plugin_params.lfo1_index.smoothed.next().into());
                lfo.generate_samples();
            }
        }

        // reverb
        if self.plugin_params.reverb_delay.smoothed.is_smoothing()
            || self.plugin_params.reverb_volume.smoothed.is_smoothing()
            || self.plugin_params.reverb_feedback.smoothed.is_smoothing()
            || self.plugin_params.reverb_color.smoothed.is_smoothing()
            || self.plugin_params.reverb_q.smoothed.is_smoothing()
            || self.plugin_params.reverb_lfo.smoothed.next() > 0.01
        {
            let mut delay = self.plugin_params.reverb_delay.smoothed.next();
            
            if self.plugin_params.reverb_lfo.smoothed.next() > 0.01 {
                let lfo_value = self.lfo1.as_mut().unwrap().lock().unwrap().amplitude();
                delay = (delay + (lfo_value * 40000.0) as i32).min(40000);
            }

            self.reverb.as_mut().unwrap().update(
                delay,
                self.plugin_params.reverb_feedback.smoothed.next(),
                self.plugin_params.reverb_color.smoothed.next(),
                self.plugin_params.reverb_q.smoothed.next());
        }
    }


    fn update_filter(&mut self) {
        let mut cutoff = self.plugin_params.filter_cutoff.smoothed.next();
        let q = self.plugin_params.filter_q.smoothed.next();

        // handle lfo here
        // will make the case for calling this function more complicated
        // but we don't hvae to pass the lfo to the filter

        let lfo_strength = self.plugin_params.filter_lfo_strength.smoothed.next();
        if let Some(lfo) = self.lfo1.as_mut() {
            cutoff += lfo.lock().unwrap().amplitude() as f32 * lfo_strength;
            cutoff = cutoff.min(FILTER_CUTOFF_MAX).max(FILTER_CUTOFF_MIN);
        }

        let coefficients = BiquadCoefficients::lowpass(self.sample_rate as f32, cutoff, q);
        self.filter.coefficients = coefficients;
    }

    // create a new voice
    pub fn voice_on(&mut self, note: u8, velocity: f32) {
        let time_per_sample = 1.0 / self.sample_rate;
        self.voices.insert(
            note,
            Voice::from_midi(note, velocity, time_per_sample, self.plugin_params.clone(), self.lfo1.as_ref().unwrap().clone()),
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
