use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

mod synth;
use synth::Synth;

mod params;
use params::SynthTwoParams;

mod editor;

struct SynthTwo {
    params: Arc<SynthTwoParams>,

    // data that we use to draw graphs in the editor
    envelope: Arc<Mutex<Vec<f32>>>,

    // shared between the synth and the editor.
    // stored here for convenience.
    graph_samples: Arc<Mutex<Vec<f32>>>,
    spectrum_samples: Arc<Mutex<Vec<f32>>>,

    // sample code says to put this in the params
    // so that the gui state can be restored automatically
    // but I don't really want to do that
    editor_state: Arc<ViziaState>,
    synth: Synth,
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
            spectrum_samples: Arc::new(Mutex::new(vec![])),
            editor_state: editor::default_state(),
            synth: Synth::default(),
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
            spectrum_samples: self.spectrum_samples.clone(),
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
            self.spectrum_samples.clone(),
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

            let output_sample = self.synth.process_sample();

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
        
        self.synth.spectrum_calculator.process(buffer);


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
