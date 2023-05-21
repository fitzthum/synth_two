use nih_plug::prelude::*;

#[derive(Enum, Debug, PartialEq)]
pub enum LfoConnection {
    #[id = "none"]
    NoLfo,
    #[id = "LFO1"]
    Lfo1,
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
    #[id = "osc1-wave-index"]
    pub osc1_wave_index: FloatParam,

    #[id = "osc1-wave-warp"]
    pub osc1_wave_warp: FloatParam,

    #[id = "osc1-warp-attack"]
    pub osc1_warp_attack: FloatParam,

    #[id = "osc1-warp-decay"]
    pub osc1_warp_decay: FloatParam,

    #[id = "osc1-warp-sustain"]
    pub osc1_warp_sustain: FloatParam,

    #[id = "osc1-warp-release"]
    pub osc1_warp_release: FloatParam,

    #[id = "osc1-tuning"]
    pub osc1_tuning: FloatParam,

    #[id = "osc1-tuning-fine"]
    pub osc1_tuning_fine: FloatParam,

    // All the parameters for the second OSC
    #[id = "osc2-wave-index"]
    pub osc2_wave_index: FloatParam,

    #[id = "osc2-wave-warp"]
    pub osc2_wave_warp: FloatParam,

    #[id = "osc2-warp-attack"]
    pub osc2_warp_attack: FloatParam,

    #[id = "osc2-warp-decay"]
    pub osc2_warp_decay: FloatParam,

    #[id = "osc2-warp-sustain"]
    pub osc2_warp_sustain: FloatParam,

    #[id = "osc2-warp-release"]
    pub osc2_warp_release: FloatParam,

    #[id = "osc2-tuning"]
    pub osc2_tuning: FloatParam,

    #[id = "osc2-tuning-fine"]
    pub osc2_tuning_fine: FloatParam,

    #[id = "oscillator-balance"]
    pub oscillator_balance: FloatParam,

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
            attack: FloatParam::new("Attack", 0.01, FloatRange::Linear { min: 0.0, max: 5.0 })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" seconds"),

            // Decay
            decay: FloatParam::new("Decay", 0.0, FloatRange::Linear { min: 0.0, max: 5.0 })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" seconds"),

            // Sustain
            sustain: FloatParam::new("Sustain", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" percent"),

            // Release
            release: FloatParam::new("Release", 0.01, FloatRange::Linear { min: 0.0, max: 5.0 })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" seconds"),

            // Wave warp stuff or first oscillator
            osc1_wave_index: FloatParam::new(
                "Oscillator 1 Wave Index",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            // this time we can scale this here rather than arithmetically later
            osc1_wave_warp: FloatParam::new(
                "Oscillator 1 Wave Warp",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            osc1_warp_attack: FloatParam::new(
                "Oscillator 1 Warp Attack",
                0.5,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            osc1_warp_decay: FloatParam::new(
                "Oscillator 1 Warp Decay",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            osc1_warp_sustain: FloatParam::new(
                "Oscillator 1 Warp Sustain",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" percent"),

            osc1_warp_release: FloatParam::new(
                "Oscillator 1 Warp Release",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            osc1_tuning: FloatParam::new(
                "Oscillator 1 Tuning",
                0.0,
                FloatRange::Linear { min: -3.0, max: 3.0 },
            ),

            osc1_tuning_fine: FloatParam::new(
                "Oscillator 1 Fine Tuning",
                0.0,
                FloatRange::Linear { min: -10.0, max: 10.0 },
            ),


            // Wave warp stuff for second oscillator
            osc2_wave_index: FloatParam::new(
                "Oscillator 2 Wave Index",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            osc2_wave_warp: FloatParam::new(
                "Oscillator 2 Wave Warp",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            osc2_warp_attack: FloatParam::new(
                "Oscillator 2 Warp Attack",
                0.2,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            osc2_warp_decay: FloatParam::new(
                "Oscillator 2 Warp Decay",
                0.2,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            osc2_warp_sustain: FloatParam::new(
                "Oscillator 2 Warp Sustain",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" percent"),

            osc2_warp_release: FloatParam::new(
                "Oscillator 2 Warp Release",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // tuning 
            osc2_tuning: FloatParam::new(
                "Oscillator 2 Tuning",
                0.0,
                FloatRange::Linear { min: -3.0, max: 3.0 },
            ),

            osc2_tuning_fine: FloatParam::new(
                "Oscillator 2 Fine Tuning",
                0.0,
                FloatRange::Linear { min: -10.0, max: 10.0 },
            ),

            // Oscillator Balance
            oscillator_balance: FloatParam::new(
                "Oscillator Balance",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),


            // Analog
            analog: FloatParam::new(
                "Analog",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),

            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                10000.0,
                FloatRange::Skewed { min: 40.0, max: 18000.0, factor: FloatRange::skew_factor(-1.0), },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            filter_q: FloatParam::new(
                "Filter Q",
                2.0f32.sqrt(),
                FloatRange::Skewed { min: 2.0f32.sqrt() / 2.0, max: 10.0, factor: FloatRange::skew_factor(-1.0), },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            filter_lfo: EnumParam::new("Filter LFO", LfoConnection::NoLfo),

            filter_lfo_strength: FloatParam::new(
                "Filter LFO Strength",
                0.0,
                FloatRange::Skewed { min: 0.0, max: 7000.0, factor: FloatRange::skew_factor(-1.0), },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0)),

            lfo1_period: FloatParam::new(
                "LFO1 Period",
                1.0,
                FloatRange::Linear { min: 0.1, max: 20.0 },
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
