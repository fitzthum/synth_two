use rust_embed::RustEmbed;
use serde::Deserialize;

const WAVE_TABLE_LENGTH: usize = 4096;

lazy_static! {
    pub static ref WAVE_TABLE: WaveTable = WaveTable::new();
}

// Time from note on to sample being processed
pub trait Oscillator {
    // different oscillators will have different init logic
    // so don't define anything here

    // an oscillator is always linked to a voice,
    // so the frequency can't be adjusted between samples
    fn process(&self, time: f64) -> f64;
}

#[derive(RustEmbed)]
#[folder = "waves"]
#[include = "*.json"]
struct WaveFiles;

// Struct to store the waves
pub struct WaveTable {
    waves: Vec<Wave>,
}

impl WaveTable {
    pub fn new() -> Self {
        let mut waves: Vec<Wave> = vec![];

        for path in WaveFiles::iter() {
            let f = WaveFiles::get(&path).unwrap().data;
            waves.push(serde_json::from_str(std::str::from_utf8(&f).unwrap()).unwrap());
        }

        WaveTable { waves }
    }
}

// might be overkill having two different structs
#[derive(Deserialize)]
pub struct Wave {
    samples: Vec<f64>,
}

/*
 * Basic Wave Table Oscillator
 *
 */
pub struct WaveTableOscillator {
    time_per_sample: f64,
    samples_per_cycle: f64,
    scale_factor: f64,
    wave_index: f64,
}

impl WaveTableOscillator {
    pub fn new(frequency: f64, time_per_sample: f64) -> Self {
        let samples_per_cycle = (1.0 / time_per_sample) / frequency;
        let scale_factor = WAVE_TABLE_LENGTH as f64 / samples_per_cycle;

        Self {
            time_per_sample,
            samples_per_cycle,
            scale_factor,
            wave_index: 0.5,
        }
    }

    pub fn set_wave_index(&mut self, wave_index: f64) {
        self.wave_index = wave_index;
    }
}

impl Oscillator for WaveTableOscillator {
    fn process(&self, time: f64) -> f64 {
        let total_sample_offset = time / self.time_per_sample;
        let unscaled_sample_offset = total_sample_offset % self.samples_per_cycle;

        // hopefully the way we do the rounding won't land us out of bounds
        let table_offset = unscaled_sample_offset * self.scale_factor;

        // wave_index is a float between 0.0 and 1.0. We want to use this to
        // switch between N waves
        let mut wave_index_a = 0.0;
        let mut wave_index_b = 1.0;
        let mut scaled_warp = self.wave_index;

        let n_waves = WAVE_TABLE.waves.len();
        let wave_width = 1.0 / (n_waves - 1) as f64;

        // there is probably a way to do this arithmetically
        if n_waves > 2 {
            for i in 0..(n_waves - 1) {
                let maybe_a = i as f64;
                let maybe_b = (i + 1) as f64;

                let maybe_a_threshold = maybe_a * wave_width;
                let maybe_b_threshold = maybe_b * wave_width;

                if self.wave_index <= maybe_b_threshold {
                    wave_index_a = maybe_a;
                    wave_index_b = maybe_b;

                    scaled_warp = (self.wave_index - maybe_a_threshold) / wave_width;
                    break;
                }
            }
        }

        let sample_a = WAVE_TABLE.waves[wave_index_a as usize].samples[table_offset as usize];
        let sample_b = WAVE_TABLE.waves[wave_index_b as usize].samples[table_offset as usize];

        let delta = sample_b - sample_a;
        sample_a + delta * scaled_warp
    }
}
