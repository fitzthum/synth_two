use nih_plug::prelude::*;
use realfft::num_complex::Complex32;
use realfft::{RealFftPlanner, RealToComplex};
use std::f32;
use std::sync::{Arc, Mutex};

const WINDOW_SIZE: usize = 64;
const FILTER_WINDOW_SIZE: usize = 33;
// since we have no kernel here, there is no point in using this
// not sure if we need overlap-add at all

const FFT_WINDOW_SIZE: usize = WINDOW_SIZE + FILTER_WINDOW_SIZE - 1;
const GAIN_COMPENSATION: f32 = 1.0 / FFT_WINDOW_SIZE as f32;

pub struct SpectrumCalculator {
    stft: util::StftHelper,
    r2c_plan: Arc<dyn RealToComplex<f32>>,

    complex_fft_buffer: Vec<Complex32>,
    spectrum_samples: Arc<Mutex<Vec<f32>>>,
}

impl Default for SpectrumCalculator {
    fn default() -> Self {

        let mut planner = RealFftPlanner::new();
        let r2c_plan = planner.plan_fft_forward(FFT_WINDOW_SIZE);
        let complex_fft_buffer = r2c_plan.make_output_vec();

        Self {
            stft: util::StftHelper::new(2, WINDOW_SIZE, FFT_WINDOW_SIZE - WINDOW_SIZE),
            r2c_plan,
            complex_fft_buffer,
            spectrum_samples: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl SpectrumCalculator {
    pub fn set_buffer(&mut self, spectrum_samples: Arc<Mutex<Vec<f32>>>) {
        self.spectrum_samples = spectrum_samples;

    }

    pub fn process(
        &mut self,
        buffer: &mut Buffer,
    ) {
        self.stft
            .process_overlap_add(buffer, 1, |_channel_idx, real_fft_buffer| {
                self.r2c_plan
                    .process_with_scratch(real_fft_buffer, &mut self.complex_fft_buffer, &mut [])
                    .unwrap();

                let mut spectrum_samples = vec![];
                let mut sample_count = 0;

                for fft_bin in self
                    .complex_fft_buffer
                    .iter_mut()
                {
                    if sample_count < WINDOW_SIZE {
                        spectrum_samples.push(fft_bin.norm() * GAIN_COMPENSATION);

                    }
                    sample_count += 1;

                }
                *self.spectrum_samples.lock().unwrap() = spectrum_samples;
            });

    }
}
