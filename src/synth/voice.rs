// A voice roughly corresponds to a note
use std::sync::Arc;

use crate::synth::envelope::{Envelope, ADSR};
use crate::synth::oscillator::{Oscillator, SineOscillator};
use crate::SynthTwoParams;

fn midi_note_to_freq(note: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    ((f64::from(note as i8 - A4_PITCH)) / 12.0).exp2() * A4_FREQ
}

pub struct Voice {
    // this represents the note
    // maybe it should be in a separate struct?
    velocity: f32,
    time_since_on: f64,
    time_off: f64,
    pub finished: bool,

    // some more general params (should they be here?)
    time_per_sample: f64,

    // plugin params for easy access
    plugin_params: Arc<SynthTwoParams>,

    // all the components for this voice
    oscillator: SineOscillator,
}

impl Voice {
    pub fn from_midi(
        note: u8,
        velocity: f32,
        time_per_sample: f64,
        plugin_params: Arc<SynthTwoParams>,
    ) -> Self {
        let frequency = midi_note_to_freq(note);

        Self {
            velocity,
            time_since_on: 0.0,
            time_off: 0.0,
            finished: false,
            time_per_sample,
            plugin_params,
            oscillator: SineOscillator::new(frequency),
        }
    }

    pub fn voice_off(&mut self) {
        self.time_off = self.time_since_on;

        // change this once we have envelopes
        self.finished = true;
    }

    pub fn process(&mut self) -> f64 {
        let out = self.oscillator.process(self.time_since_on);
        self.time_since_on += self.time_per_sample;
        out * self.envelope() * self.velocity_amp()
    }

    fn velocity_amp(&self) -> f64 {
        self.velocity as f64 / 127.0
    }

    fn envelope(&mut self) -> f64 {
        let mut env = ADSR::new(
            self.plugin_params.attack.smoothed.next(),
            self.plugin_params.decay.smoothed.next(),
            self.plugin_params.sustain.smoothed.next(),
            self.plugin_params.release.smoothed.next(),
        );

        let out = env.process(self.time_since_on, self.time_off);
        self.finished = env.finished;

        out
    }
}
