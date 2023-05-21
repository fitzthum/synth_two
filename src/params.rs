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

    // All the warp parameters for the first OSC
    #[id = "wave-index-1"]
    pub wave_index_1: FloatParam,

    #[id = "wave-warp-1"]
    pub wave_warp_1: FloatParam,

    #[id = "warp-attack-1"]
    pub warp_attack_1: FloatParam,

    #[id = "warp-decay-1"]
    pub warp_decay_1: FloatParam,

    #[id = "warp-sustain-1"]
    pub warp_sustain_1: FloatParam,

    #[id = "warp-release-1"]
    pub warp_release_1: FloatParam,

    // All the warp parameters for the second OSC
    #[id = "wave-index-2"]
    pub wave_index_2: FloatParam,

    #[id = "wave-warp-2"]
    pub wave_warp_2: FloatParam,

    #[id = "warp-attack-2"]
    pub warp_attack_2: FloatParam,

    #[id = "warp-decay-2"]
    pub warp_decay_2: FloatParam,

    #[id = "warp-sustain-2"]
    pub warp_sustain_2: FloatParam,

    #[id = "warp-release-2"]
    pub warp_release_2: FloatParam,

    #[id = "oscillator-balance"]
    pub oscillator_balance: FloatParam,

    #[id = "tuning-1"]
    pub tuning_1: FloatParam,

    #[id = "tuning-fine-1"]
    pub tuning_fine_1: FloatParam,

    #[id = "tuning-2"]
    pub tuning_2: FloatParam,

    #[id = "tuning-fine-2"]
    pub tuning_fine_2: FloatParam,

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
            wave_index_1: FloatParam::new(
                "Wave Index 1",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            // this time we can scale this here rather than arithmetically later
            wave_warp_1: FloatParam::new(
                "Wave Warp 1",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            warp_attack_1: FloatParam::new(
                "Warp Attack 1",
                0.5,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_decay_1: FloatParam::new(
                "Warp Decay 1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_sustain_1: FloatParam::new(
                "Warp Sustain 1",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" percent"),

            warp_release_1: FloatParam::new(
                "Warp Release 1",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // Wave warp stuff for second oscillator
            wave_index_2: FloatParam::new(
                "Wave Index 2",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            wave_warp_2: FloatParam::new(
                "Wave Warp 2",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            warp_attack_2: FloatParam::new(
                "Warp Attack 2",
                0.2,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_decay_2: FloatParam::new(
                "Warp Decay 2",
                0.2,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            warp_sustain_2: FloatParam::new(
                "Warp Sustain 2",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" percent"),

            warp_release_2: FloatParam::new(
                "Warp Release 2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" seconds"),

            // Oscillator Balance
            oscillator_balance: FloatParam::new(
                "Oscillator Balance",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0)),

            // tuning 
            tuning_1: FloatParam::new(
                "Oscillator 1 Tuning",
                0.0,
                FloatRange::Linear { min: -3.0, max: 3.0 },
            ),

            tuning_fine_1: FloatParam::new(
                "Oscillator 1 Fine Tuning",
                0.0,
                FloatRange::Linear { min: -10.0, max: 10.0 },
            ),

            tuning_2: FloatParam::new(
                "Oscillator 2 Tuning",
                0.0,
                FloatRange::Linear { min: -3.0, max: 3.0 },
            ),

            tuning_fine_2: FloatParam::new(
                "Oscillator 2 Fine Tuning",
                0.0,
                FloatRange::Linear { min: -10.0, max: 10.0 },
            ),

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
