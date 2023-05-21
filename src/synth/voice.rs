// A voice roughly corresponds to a note
use rand::Rng;
use std::sync::Arc;

use crate::synth::envelope::{Envelope, ADSR};
use crate::synth::oscillator::{Oscillator, WaveTableOscillator};
use crate::SynthTwoParams;

fn midi_note_to_freq(note: u8, tune: f64, tune_fine: f64) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    let pitch_tweak = 12 * tune as i8;

    ((f64::from(note as i8 - A4_PITCH + pitch_tweak) / 12.0).exp2() * A4_FREQ) + tune_fine
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
        let mut rng = rand::thread_rng();
        let analog: f64 = plugin_params.analog.value().into();

        let rand_tweak_1 = (rng.gen_range(0.0..10.0) - 5.0) * analog;
        let frequency1 = midi_note_to_freq(
            note,
            plugin_params.osc1_tuning.value().into(),
            plugin_params.osc1_tuning_fine.value().into(),
        ) + rand_tweak_1;

        let rand_tweak_2 = (rng.gen_range(0.0..10.0) - 5.0) * analog;
        let frequency2 = midi_note_to_freq(
            note,
            plugin_params.osc2_tuning.value().into(),
            plugin_params.osc2_tuning_fine.value().into(),
        ) + rand_tweak_2;

        let rand_tweak_velocity = (rng.gen_range(0.0..1.0) - 0.5) * analog as f32;
        Self {
            velocity: velocity + rand_tweak_velocity,
            time_since_on: 0.0,
            time_off: 0.0,
            finished: false,
            time_per_sample,
            plugin_params,
            oscillator1: WaveTableOscillator::new(frequency1, time_per_sample),
            oscillator2: WaveTableOscillator::new(frequency2, time_per_sample),
            main_envelope: ADSR::default(),
            warp_envelope_1: ADSR::default(),
            warp_envelope_2: ADSR::default(),
        }
    }

    pub fn voice_off(&mut self) {
        self.time_off = self.time_since_on;
    }

    pub fn process(&mut self) -> f64 {
        let ww1: f64 = self.plugin_params.osc1_wave_warp.value().into();
        let bwi1: f64 = self.plugin_params.osc1_wave_index.value().into();
        let wi1 = bwi1 + ww1 * self.warp_envelope_1();

        self.oscillator1.set_wave_index(wi1);
        let o1 = self.oscillator1.process(self.time_since_on);

        let ww2: f64 = self.plugin_params.osc2_wave_warp.value().into();
        let bwi2: f64 = self.plugin_params.osc2_wave_index.value().into();
        let wi2 = bwi2 + ww2 * self.warp_envelope_2();

        self.oscillator2.set_wave_index(wi2);
        let o2 = self.oscillator2.process(self.time_since_on);

        let balance: f64 = self.plugin_params.oscillator_balance.smoothed.next().into();
        let ob = (o2 * balance) + (o1 * (1.0 - balance));

        self.time_since_on += self.time_per_sample;
        ob * self.main_envelope() * self.velocity as f64
    }

    fn warp_envelope_1(&mut self) -> f64 {
        self.warp_envelope_1.update(
            self.plugin_params.osc1_warp_attack.smoothed.next(),
            self.plugin_params.osc1_warp_decay.smoothed.next(),
            self.plugin_params.osc1_warp_sustain.smoothed.next(),
            self.plugin_params.osc1_warp_release.smoothed.next(),
        );

        self.warp_envelope_1
            .process(self.time_since_on, self.time_off)
    }

    fn warp_envelope_2(&mut self) -> f64 {
        self.warp_envelope_2.update(
            self.plugin_params.osc2_warp_attack.smoothed.next(),
            self.plugin_params.osc2_warp_decay.smoothed.next(),
            self.plugin_params.osc2_warp_sustain.smoothed.next(),
            self.plugin_params.osc2_warp_release.smoothed.next(),
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
