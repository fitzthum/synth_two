use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

mod synth;
use synth::Synth;

mod editor;

struct SynthTwo {
    params: Arc<SynthTwoParams>,

    // data that we use to draw graphs in the editor
    envelope: Arc<Mutex<Vec<f32>>>,
    graph_samples: Arc<Mutex<Vec<f32>>>,

    // sample code says to put this in the params
    // so that the gui state can be restored automatically
    // but I don't really want to do that
    editor_state: Arc<ViziaState>,
    synth: Synth,
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
}

impl Default for SynthTwo {
    fn default() -> Self {
        let params = Arc::new(SynthTwoParams::default());
        let envelope = Arc::new(Mutex::new(vec![
            params.attack.default_plain_value(),
            params.decay.default_plain_value(),
            params.sustain.default_plain_value(),
            params.release.default_plain_value(),
        ]));
        Self {
            params,
            envelope,
            graph_samples: Arc::new(Mutex::new(vec![])),
            editor_state: editor::default_state(),
            synth: Synth::default(),
        }
    }
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
        }
    }
}

impl Plugin for SynthTwo {
    const NAME: &'static str = "Synth Two";
    const VENDOR: &'static str = "Tobin Fitzthum";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "tobinf@protonmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(0),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let data = editor::Data {
            params: self.params.clone(),
            envelope: self.envelope.clone(),
            graph_samples: self.graph_samples.clone(),
        };
        editor::create(data, self.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.synth.initialize(
            self.params.clone(),
            buffer_config.sample_rate.into(),
            self.envelope.clone(),
            self.graph_samples.clone(),
        );

        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Nothing yet
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        const GRAPH_SAMPLE_RATIO: usize = 4;
        let mut graph_samples = vec![]; 
        for (n, channel_samples) in buffer.iter_samples().enumerate() {
            // process midi events
            while let Some(event) = next_event {
                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.synth.voice_on(note, velocity);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.synth.voice_off(note);
                    }
                    _ => (),
                }
                next_event = context.next_event();
            }

            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();

            let output_sample = self.synth.process_sample() as f32;

            for sample in channel_samples {
                *sample = output_sample * gain; 
            }
            
            if n % GRAPH_SAMPLE_RATIO == 0 {
                graph_samples.push(output_sample);
            }

            // clear out unused voices
            self.synth.reap_voices();
        }

        // push the samples to the mutex
        *self.graph_samples.lock().unwrap() = graph_samples;


        ProcessStatus::Normal
    }
}

impl Vst3Plugin for SynthTwo {
    const VST3_CLASS_ID: [u8; 16] = *b"Synth22222222222";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_vst3!(SynthTwo);
