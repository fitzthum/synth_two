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

#[derive(Lens, Clone)]
pub struct Data {
    pub params: Arc<SynthTwoParams>,
    pub envelope: Arc<Mutex<Vec<f32>>>,
    pub graph_samples: Arc<Mutex<Vec<f32>>>,
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
        output(cx);
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
            EnvelopeGraph::new(cx, Data::envelope).class("graph");
        })
        .class("row");

        // ADSR Graph goes here
    })
    .class("section");
}

fn output(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Output").class("section-title");
        WaveGraph::new(cx, Data::graph_samples).class("graph");
    })
    .class("section")
    .child_right(Stretch(1.0));

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
                |params| &params.wave_index_1,
                Some("Index"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.wave_warp_1, Some("Warp"));
            ParamKnob::new(cx, Data::params, |params| &params.tuning_1, Some("Tune"));
            ParamKnob::new(cx, Data::params, |params| &params.tuning_fine_1, Some("Fine Tune"));
        })
        .class("row");

        // warp adsr
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.warp_attack_1, Some("A"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_decay_1, Some("D"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_sustain_1, Some("S"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_release_1, Some("R"));
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
                |params| &params.wave_index_2,
                Some("Index"),
            );
            ParamKnob::new(cx, Data::params, |params| &params.wave_warp_2, Some("Warp"));
            ParamKnob::new(cx, Data::params, |params| &params.tuning_2, Some("Tune"));
            ParamKnob::new(cx, Data::params, |params| &params.tuning_fine_2, Some("Fine Tune"));

        })
        .class("row");

        // warp adsr
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.warp_attack_2, Some("A"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_decay_2, Some("D"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_sustain_2, Some("S"));
            ParamKnob::new(cx, Data::params, |params| &params.warp_release_2, Some("R"));
        })
        .class("row");
    })
    .class("section");
}

fn effects(cx: &mut Context) {
    HStack::new(cx, |cx| {
        filter(cx);
    })
    .id("effects");
}

fn filter(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Filter").class("section-title");

        // wave controls
        HStack::new(cx, |cx| { 
            ParamKnob::new(cx, Data::params, |params| &params.filter_cutoff, Some("Cutoff"));
        });
    })
    .class("section")
    .right(Stretch(1.0));
 
}


