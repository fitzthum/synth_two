use crate::SynthTwoParams;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::{Arc, Mutex};

mod knob;
use knob::ParamKnob;

mod envelope;
use envelope::EnvelopeGraph;

mod wave;
use wave::WaveGraph;

mod spectrum;
use spectrum::SpectrumGraph;

#[derive(Lens, Clone)]
pub struct Data {
    pub params: Arc<SynthTwoParams>,
    pub envelope: Arc<Mutex<Vec<f32>>>,
    pub graph_samples: Arc<Mutex<Vec<f32>>>,
    pub spectrum_samples: Arc<Mutex<Vec<f32>>>,
    pub lfo1_samples: Arc<Mutex<Vec<f32>>>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (1000, 710))
}

pub(crate) fn create(data: Data, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_fonts_mem(&[b"Px IBM MDA"]);
        cx.set_default_font(&["Px IBM MDA"]);

        cx.add_theme(include_str!("theme.css"));

        data.clone().build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            general(cx);
            oscillators(cx);
            effects(cx);
            output(cx);
        })
        .child_bottom(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}

// Some overall parameters
fn general(cx: &mut Context) {
    HStack::new(cx, |cx| {
        global_controls(cx);
        envelope(cx);
    })
    .class("top");
}

fn global_controls(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "General").class("section-title");
        // Main controls
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.gain, None);
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.oscillator_balance,
                Some("Balance"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.analog, None);
        })
        .class("row");
    })
    .class("section");
}

fn envelope(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Envelope").class("section-title");
        // ADSR
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.attack, Some("A"));
            ParamKnob::new(cx, Data::params, |params| &params.decay, Some("D"));
            ParamKnob::new(cx, Data::params, |params| &params.sustain, Some("S"));
            ParamKnob::new(cx, Data::params, |params| &params.release, Some("R"));
            // need to make lens for adsr
            VStack::new(cx, |cx| {
                EnvelopeGraph::new(cx, Data::envelope).class("graph");
            })
            .class("graph-wrapper")
            .height(Pixels(40.0))
            .width(Pixels(80.0))
            .top(Pixels(20.0));
        })
        .class("row");

        // ADSR Graph goes here
    })
    .class("section");
}

fn output(cx: &mut Context) {
    HStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Output").class("section-title");
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    WaveGraph::new(cx, Data::graph_samples).class("graph");
                })
                .class("graph-wrapper");
                VStack::new(cx, |cx| {
                    SpectrumGraph::new(cx, Data::spectrum_samples).class("graph");
                })
                .class("graph-wrapper");
            })
            .class("row")
            .row_between(Pixels(20.0));
        })
        .class("section");
    });
}

fn oscillators(cx: &mut Context) {
    HStack::new(cx, |cx| {
        oscillator1(cx);
        oscillator2(cx);
    })
    .id("oscillators");
}

// in theory this should be generic but idk how
// maybe if we had a struct for the state of each oscillator
fn oscillator1(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Oscillator 1").class("section-title");

        // wave controls
        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_wave_index,
                Some("Index"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_wave_warp,
                Some("Warp"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.osc1_tuning, Some("Tune"));
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_tuning_fine,
                Some("Fine Tune"),
            );
        })
        .class("row");

        // warp adsr
        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_warp_attack,
                Some("A"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_warp_decay,
                Some("D"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_warp_sustain,
                Some("S"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc1_warp_release,
                Some("R"),
            );
        })
        .class("row");
    })
    .class("section");
}

fn oscillator2(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Oscillator 2").class("section-title");

        // wave controls
        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_wave_index,
                Some("Index"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_wave_warp,
                Some("Warp"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.osc2_tuning, Some("Tune"));
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_tuning_fine,
                Some("Fine Tune"),
            );
        })
        .class("row");

        // warp adsr
        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_warp_attack,
                Some("A"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_warp_decay,
                Some("D"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_warp_sustain,
                Some("S"),
            );
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.osc2_warp_release,
                Some("R"),
            );
        })
        .class("row");
    })
    .class("section");
}

fn effects(cx: &mut Context) {
    HStack::new(cx, |cx| {
        filter(cx);
        lfo1(cx);
    })
    .id("effects");
}

fn filter(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Filter").class("section-title");

        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.filter_cutoff,
                Some("Cutoff"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.filter_q, Some("Q"));
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.filter_lfo_strength,
                Some("LFO"),
            );
        })
        .class("row");
    })
    .class("section")
    .right(Stretch(1.0));
}

fn lfo1(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "LFO1").class("section-title");

        HStack::new(cx, |cx| {
            ParamKnob::new(
                cx,
                Data::params,
                |params| &params.lfo1_period,
                Some("Period"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.lfo1_index, Some("Index"));
            VStack::new(cx, |cx| {
                WaveGraph::new(cx, Data::lfo1_samples).class("graph");
            })
            .class("graph-wrapper");
        })
        .class("row");
    })
    .class("section")
    .right(Stretch(1.0));
}
