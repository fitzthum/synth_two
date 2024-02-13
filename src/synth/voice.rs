// A voice roughly corresponds to a note
use rand::Rng;
use std::sync::{Arc, Mutex};

use crate::synth::envelope::{Envelope, ADSR};
use crate::synth::oscillator::{Oscillator, WaveTableOscillator};
use crate::synth::lfo::{Lfo, WaveTableLfo};
use crate::SynthTwoParams;
use crate::params::OscillatorParams;

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
    lfo1: Arc<Mutex<WaveTableLfo>>,
}

impl Voice {
    pub fn from_midi(
        note: u8,
        velocity: f32,
        time_per_sample: f64,
        plugin_params: Arc<SynthTwoParams>,
        lfo1: Arc<Mutex<WaveTableLfo>>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let analog: f64 = plugin_params.analog.value().into();

        let rand_tweak_1 = (rng.gen_range(0.0..10.0) - 5.0) * analog;
        let frequency1 = midi_note_to_freq(
            note,
            plugin_params.osc1.tuning.value().into(),
            plugin_params.osc1.tuning_fine.value().into(),
        ) + rand_tweak_1;

        let bank_id1 = plugin_params.osc1.bank_id.value();

        let rand_tweak_2 = (rng.gen_range(0.0..10.0) - 5.0) * analog;
        let frequency2 = midi_note_to_freq(
            note,
            plugin_params.osc2.tuning.value().into(),
            plugin_params.osc2.tuning_fine.value().into(),
        ) + rand_tweak_2;

        let bank_id2 = plugin_params.osc2.bank_id.value();

        let rand_tweak_velocity = (rng.gen_range(0.0..1.0) - 0.5) * analog as f32;
        Self {
            velocity: velocity + rand_tweak_velocity,
            time_since_on: 0.0,
            time_off: 0.0,
            finished: false,
            time_per_sample,
            plugin_params,
            oscillator1: WaveTableOscillator::new(frequency1, time_per_sample, bank_id1),
            oscillator2: WaveTableOscillator::new(frequency2, time_per_sample, bank_id2),
            main_envelope: ADSR::default(),
            warp_envelope_1: ADSR::default(),
            warp_envelope_2: ADSR::default(),
            lfo1,
        }
    }

    pub fn voice_off(&mut self) {
        self.time_off = self.time_since_on;
    }

    pub fn process(&mut self) -> f64 {
    
        // generate sample for each oscillator
        let wave_index = Self::wave_index(self.plugin_params.osc1.clone(), &mut self.warp_envelope_1, self.time_since_on, self.time_off);
        
        self.oscillator1.set_wave_index(wave_index);
        let o1 = self.oscillator1.process(self.time_since_on);

        // second oscillator
        let wave_index = Self::wave_index(self.plugin_params.osc2.clone(), &mut self.warp_envelope_2, self.time_since_on, self.time_off);
        
        self.oscillator2.set_wave_index(wave_index);
        let o2 = self.oscillator2.process(self.time_since_on);


        // calculate oscillator balance
        let mut balance: f64 = self.plugin_params.oscillator_balance.smoothed.next().into();
        let balance_lfo_strength: f64 = self.plugin_params.oscillator_balance_lfo_strength.smoothed.next().into();
        
        // LFO for balance
        if balance_lfo_strength > 0.0 {
            // the lfo uses the global clock.
            balance += self.lfo1.lock().unwrap().amplitude() * balance_lfo_strength;
        }
        balance = balance.min(1.0).max(0.0);
        
        // mix oscillators together using balance
        let ob = (o2 * balance) + (o1 * (1.0 - balance));

        // increment note time
        self.time_since_on += self.time_per_sample;

        // apply main envelope
        ob * self.main_envelope() * self.velocity as f64
    }

    // Using the note timing information and the oscillator params,
    // calculate the wave index for a given oscillator and note/voice
    fn wave_index(params: Arc<OscillatorParams>, env: &mut ADSR, time_since_on: f64, time_off: f64) -> f64 {

        let wave_index_start: f64 = params.wave_index_start.value().into();
        let wave_index_end: f64 = params.wave_index_end.value().into();
        let wave_warp = wave_index_end - wave_index_start;

        env.update(
            params.warp_attack.smoothed.next(),
            params.warp_decay.smoothed.next(),
            params.warp_sustain.smoothed.next(),
            params.warp_release.smoothed.next(),
        );
        wave_index_start + wave_warp * env.process(time_since_on, time_off)
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
