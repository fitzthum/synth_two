use std::collections::HashMap;

mod voice; 
use voice::Voice;

mod oscillator;

pub struct Synth {
    sample_rate: f64,
    voices: HashMap<u8, Voice>
}

impl Synth {
    pub fn default() -> Self {
        Self {
            sample_rate: 1.0,
            voices: HashMap::new(),
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    // we're doing fake stereo at first
    pub fn process_sample(&mut self) -> f64 {
        let mut out = 0.0;
        for (_, voice) in self.voices.iter_mut() {
            out += voice.process();
        }
        out
    }

    // create a new voice 
    pub fn voice_on(&mut self, note: u8, velocity: f32) {
        let time_per_sample = 1.0 / self.sample_rate;
        self.voices.insert(note, Voice::from_midi(note, velocity, time_per_sample));

    }

    pub fn voice_off(&mut self, note: u8) {
        self.voices.get_mut(&note).expect("Turning off non-existent note...").voice_off();

    }

    pub fn reap_voices(&mut self) {
        self.voices.retain(|_, note| !note.finished);

    }
    
}
