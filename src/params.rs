use std::sync::Arc;

use nih_plug::prelude::*;

pub const FILTER_CUTOFF_MIN: f32 = 40.0;
pub const FILTER_CUTOFF_MAX: f32 = 18000.0;

pub const ENVELOPE_TIME_MAX: f32 = 5.0;

pub const LFO_PERIOD_MIN: f32 = 0.03;
pub const LFO_PERIOD_MAX: f32 = 8.0;

#[derive(Enum, Debug, PartialEq)]
pub enum LfoConnection {
    #[id = "none"]
    NoLfo,
    #[id = "LFO1"]
    Lfo1,
}

#[derive(Params)]
pub struct OscillatorParams {
    #[id = "wave-index"]
    pub wave_index: FloatParam,

    #[id = "wave-warp"]
    pub wave_warp: FloatParam,

    //TODO: can i use double-nested params for this?
    #[id = "warp-attack"]
    pub warp_attack: FloatParam,

    #[id = "warp-decay"]
    pub warp_decay: FloatParam,

    #[id = "warp-sustain"]
    pub warp_sustain: FloatParam,

    #[id = "warp-release"]
    pub warp_release: FloatParam,

    #[id = "tuning"]
    pub tuning: FloatParam,

    #[id = "tuning-fine"]
    pub tuning_fine: FloatParam,

}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self {
            wave_index: FloatParam::new(
                "Wave Index",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            // this time we can scale this here rather than arithmetically later
            wave_warp: FloatParam::new(
                "Wave Warp",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            warp_attack: FloatParam::new(
                "Warp Attack",
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_decay: FloatParam::new(
                "Warp Decay",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_sustain: FloatParam::new(
                "Warp Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" percent"),

            warp_release: FloatParam::new(
                "Warp Release",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            tuning: FloatParam::new(
                "Tuning",
                0.0,
                FloatRange::Linear {
                    min: -3.0,
                    max: 3.0,
                },
            ),

            tuning_fine: FloatParam::new(
                "Fine Tuning",
                0.0,
                FloatRange::Linear {
                    min: -10.0,
                    max: 10.0,
                },
            ),
        }
    }
}

#[derive(Params)]
pub struct SynthTwoParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "attack"]
    pub attack: FloatParam,

    #[id = "decay"]
    pub decay: FloatParam,

    #[id = "sustain"]
    pub sustain: FloatParam,

    #[id = "release"]
    pub release: FloatParam,

    // All the parameters for the first OSC
    #[nested(id_prefix = "osc1", group = "osc1")]
    pub osc1: Arc<OscillatorParams>,

    // All the parameters for the second OSC
    #[nested(id_prefix = "osc2", group = "osc2")]
    pub osc2: Arc<OscillatorParams>,

    #[id = "oscillator-balance"]
    pub oscillator_balance: FloatParam,

    #[id = "oscillator-balance-lfo-strength"]
    pub oscillator_balance_lfo_strength: FloatParam,

    // Analog/humanization factor
    #[id = "analog"]
    pub analog: FloatParam,

    #[id = "filter-cutoff"]
    pub filter_cutoff: FloatParam,

    #[id = "filter-q"]
    pub filter_q: FloatParam,

    #[id = "filter-lfo"]
    pub filter_lfo: EnumParam<LfoConnection>,

    #[id = "filter-lfo-strength"]
    pub filter_lfo_strength: FloatParam,

    #[id = "lfo1-period"]
    pub lfo1_period: FloatParam,

    #[id = "lfo1-index"]
    pub lfo1_index: FloatParam,
}

impl Default for SynthTwoParams {
    fn default() -> Self {
        Self {
            // Gain
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(-12.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-36.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-36.0, 0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            // Attack
            attack: FloatParam::new(
                "Attack",
                0.01,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // Decay
            decay: FloatParam::new(
                "Decay",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // Sustain
            sustain: FloatParam::new("Sustain", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" percent"),

            // Release
            release: FloatParam::new(
                "Release",
                0.01,
                FloatRange::Linear {
                    min: 0.0,
                    max: ENVELOPE_TIME_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // First oscillator
            osc1: Arc::new(OscillatorParams::default()),

            // Second oscillator
            osc2: Arc::new(OscillatorParams::default()),

            // Oscillator Balance
            oscillator_balance: FloatParam::new(
                "Oscillator Balance",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            oscillator_balance_lfo_strength: FloatParam::new(
                "Oscillator Balance LFO Strength",
                0.0,
                FloatRange::Linear { min: 0.0, max: 0.5 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            // Analog
            analog: FloatParam::new("Analog", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),

            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                10000.0,
                FloatRange::Skewed {
                    min: FILTER_CUTOFF_MIN,
                    max: FILTER_CUTOFF_MAX,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            filter_q: FloatParam::new(
                "Filter Q",
                2.0f32.sqrt(),
                FloatRange::Skewed {
                    min: 2.0f32.sqrt() / 2.0,
                    max: 10.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            filter_lfo: EnumParam::new("Filter LFO", LfoConnection::NoLfo),

            filter_lfo_strength: FloatParam::new(
                "Filter LFO Strength",
                0.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 7000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            lfo1_period: FloatParam::new(
                "LFO1 Period",
                1.0,
                FloatRange::Linear {
                    min: LFO_PERIOD_MIN,
                    max: LFO_PERIOD_MAX,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            lfo1_index: FloatParam::new(
                "LFO1 Index",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),
        }
    }
}
