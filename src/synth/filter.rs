// Adapted from crisp example by robbert-vdh 
//
use nih_plug::debug::*;
use std::f32::consts;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Biquad<T> {
    pub coefficients: BiquadCoefficients<T>,
    s1: T,
    s2: T,
}

#[derive(Clone, Copy, Debug)]
pub struct BiquadCoefficients<T> {
    b0: T,
    b1: T,
    b2: T,
    a1: T,
    a2: T,
}

pub trait SimdType:
    Mul<Output = Self> + Sub<Output = Self> + Add<Output = Self> + Copy + Sized
{
    fn from_f32(value: f32) -> Self;
}

impl<T: SimdType> Default for Biquad<T> {
    /// Before setting constants the filter should just act as an identity function.
    fn default() -> Self {
        Self {
            coefficients: BiquadCoefficients::identity(),
            s1: T::from_f32(0.0),
            s2: T::from_f32(0.0),
        }
    }
}

impl<T: SimdType> Biquad<T> {
    /// Process a single sample.
    pub fn process(&mut self, sample: T) -> T {
        let result = self.coefficients.b0 * sample + self.s1;

        self.s1 = self.coefficients.b1 * sample - self.coefficients.a1 * result + self.s2;
        self.s2 = self.coefficients.b2 * sample - self.coefficients.a2 * result;

        result
    }
}

impl<T: SimdType> BiquadCoefficients<T> {
    /// Convert scalar coefficients into the correct vector type.
    pub fn from_f32s(scalar: BiquadCoefficients<f32>) -> Self {
        Self {
            b0: T::from_f32(scalar.b0),
            b1: T::from_f32(scalar.b1),
            b2: T::from_f32(scalar.b2),
            a1: T::from_f32(scalar.a1),
            a2: T::from_f32(scalar.a2),
        }
    }

    /// Filter coefficients that would cause the sound to be passed through as is.
    pub fn identity() -> Self {
        Self::from_f32s(BiquadCoefficients {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        })
    }

    /// Compute the coefficients for a low-pass filter.
    ///
    /// Based on <http://shepazu.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html>.
    pub fn lowpass(sample_rate: f32, frequency: f32, q: f32) -> Self {
        nih_debug_assert!(sample_rate > 0.0);
        nih_debug_assert!(frequency > 0.0);
        nih_debug_assert!(frequency < sample_rate / 2.0);
        nih_debug_assert!(q > 0.0);

        let omega0 = consts::TAU * (frequency / sample_rate);
        let cos_omega0 = omega0.cos();
        let alpha = omega0.sin() / (2.0 * q);

        // We'll prenormalize everything with a0
        let a0 = 1.0 + alpha;
        let b0 = ((1.0 - cos_omega0) / 2.0) / a0;
        let b1 = (1.0 - cos_omega0) / a0;
        let b2 = ((1.0 - cos_omega0) / 2.0) / a0;
        let a1 = (-2.0 * cos_omega0) / a0;
        let a2 = (1.0 - alpha) / a0;

        Self::from_f32s(BiquadCoefficients { b0, b1, b2, a1, a2 })
    }
}

impl SimdType for f32 {
    #[inline(always)]
    fn from_f32(value: f32) -> Self {
        value
    }
}
