// A voice roughly corresponds to a note
use std::sync::Arc;

use crate::synth::envelope::{Envelope, ADSR};
use crate::synth::oscillator::{Oscillator, SineOscillator, WaveTableOscillator};
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
    oscillator1: WaveTableOscillator,
    oscillator2: WaveTableOscillator,
    main_envelope: ADSR,
    warp_envelope_1: ADSR,
    warp_envelope_2: ADSR,
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
            oscillator1: WaveTableOscillator::new(frequency, time_per_sample),
            oscillator2: WaveTableOscillator::new(frequency, time_per_sample),
            main_envelope: ADSR::default(),
            warp_envelope_1: ADSR::default(),
            warp_envelope_2: ADSR::default(),
        }
    }

    pub fn voice_off(&mut self) {
        self.time_off = self.time_since_on;
    }

    pub fn process(&mut self) -> f64 {
        let ww1: f64 = self.plugin_params.wave_warp_1.value().into();
        let bwi1: f64 = self.plugin_params.wave_index_1.value().into();
        let wi1 = bwi1 + ww1 * self.warp_envelope_1();

        self.oscillator1.set_wave_index(wi1);
        let o1 = self.oscillator1.process(self.time_since_on);

        let ww2: f64 = self.plugin_params.wave_warp_2.value().into();
        let bwi2: f64 = self.plugin_params.wave_index_2.value().into();
        let wi2 = bwi2 + ww2 * self.warp_envelope_2();

        self.oscillator2.set_wave_index(wi2);
        let o2 = self.oscillator2.process(self.time_since_on);

        let balance: f64 = self.plugin_params.oscillator_balance.smoothed.next().into();
        let ob = (o1 * balance) + (o2 * (1.0 - balance));

        self.time_since_on += self.time_per_sample;
        ob * self.main_envelope() * self.velocity as f64
    }

    fn warp_envelope_1(&mut self) -> f64 {
        self.warp_envelope_1.update(
            self.plugin_params.warp_attack_1.smoothed.next(),
            self.plugin_params.warp_decay_1.smoothed.next(),
            self.plugin_params.warp_sustain_1.smoothed.next(),
            self.plugin_params.warp_release_1.smoothed.next(),
        );

        self.warp_envelope_1
            .process(self.time_since_on, self.time_off)
    }

    fn warp_envelope_2(&mut self) -> f64 {
        self.warp_envelope_2.update(
            self.plugin_params.warp_attack_2.smoothed.next(),
            self.plugin_params.warp_decay_2.smoothed.next(),
            self.plugin_params.warp_sustain_2.smoothed.next(),
            self.plugin_params.warp_release_2.smoothed.next(),
        );

        self.warp_envelope_2
            .process(self.time_since_on, self.time_off)
    }

    fn main_envelope(&mut self) -> f64 {
        self.main_envelope.update(
            self.plugin_params.attack.smoothed.next(),
            self.plugin_params.decay.smoothed.next(),
            self.plugin_params.sustain.smoothed.next(),
            self.plugin_params.release.smoothed.next(),
        );

        let out = self
            .main_envelope
            .process(self.time_since_on, self.time_off);
        self.finished = self.main_envelope.finished;

        out
    }
}
