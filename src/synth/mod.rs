use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod voice;
use voice::Voice;

mod envelope;
mod oscillator;

use crate::SynthTwoParams;

pub struct Synth {
    sample_rate: f64,
    voices: HashMap<u8, Voice>,
    plugin_params: Arc<SynthTwoParams>,
    envelope: Arc<Mutex<Vec<f32>>>,
}

impl Synth {
    pub fn default() -> Self {
        Self {
            sample_rate: 1.0,
            voices: HashMap::new(),
            // This seems dumb
            plugin_params: Arc::new(SynthTwoParams::default()),
            envelope: Arc::new(Mutex::new(vec![])),
        }
    }
    pub fn initialize(
        &mut self,
        plugin_params: Arc<SynthTwoParams>,
        sample_rate: f64,
        envelope: Arc<Mutex<Vec<f32>>>,
    ) {
        self.sample_rate = sample_rate;
        self.plugin_params = plugin_params;
        self.envelope = envelope;
    }

    // we're doing fake stereo at first
    pub fn process_sample(&mut self) -> f64 {
        let mut out = 0.0;
        for (_, voice) in self.voices.iter_mut() {
            out += voice.process();
        }

        // update graph data.
        // maybe should do this somewhere else?
        let mut env = self.envelope.lock().unwrap();
        env[0] = self.plugin_params.attack.smoothed.next();
        env[1] = self.plugin_params.decay.smoothed.next();
        env[2] = self.plugin_params.sustain.smoothed.next();
        env[3] = self.plugin_params.release.smoothed.next();

        out
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
