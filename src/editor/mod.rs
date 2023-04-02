use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::SynthTwoParams;

mod knob;
use knob::ParamKnob;

#[derive(Lens)]
struct Data {
    params: Arc<SynthTwoParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 510))
}

pub(crate) fn create(
    params: Arc<SynthTwoParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_fonts_mem(&[b"Px IBM MDA"]);
        cx.set_default_font(&["Px IBM MDA"]);

        cx.add_theme(include_str!("theme.css"));

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            top(cx);
            oscillators(cx);

       })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}

// Top half of editor, stores global params
fn top(cx: &mut Context) {
    HStack::new(cx, |cx| {
        top_left(cx);
        top_right(cx);
    }).class("top");
}

fn top_left(cx: &mut Context) {
    VStack::new(cx, |cx| {
        // Main controls
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.gain);
            ParamKnob::new(cx, Data::params, |params| &params.oscillator_balance);
        });
    })
    .class("quarter");
}

fn top_right(cx: &mut Context) {
    VStack::new(cx, |cx| {
        // ADSR
        HStack::new(cx, |cx| {
            ParamKnob::new(cx, Data::params, |params| &params.attack);
            ParamKnob::new(cx, Data::params, |params| &params.decay);
            ParamKnob::new(cx, Data::params, |params| &params.sustain);
            ParamKnob::new(cx, Data::params, |params| &params.release);
        });

        // ADSR Graph goes here

    })
    .class("quarter");
}

fn oscillators(cx: &mut Context) {
    HStack::new(cx, |cx| {
        oscillator1(cx);
        oscillator2(cx);

    })
    .position_type(PositionType::SelfDirected)
    .id("oscillators");
}

// in theory this should be generic but idk how
fn oscillator1(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Oscillator 1").class("osc-title");

        HStack::new(cx, |cx| {
            Label::new(cx, "Wave Index").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.wave_index_1);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Wave Warp").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.wave_warp_1);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Attack").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_attack_1);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Decay").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_decay_1);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Sustain").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_sustain_1);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Release").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_release_1);
        }).class("row");

    })
    .class("quarter");
}

fn oscillator2(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Oscillator 2").class("osc-title");

        HStack::new(cx, |cx| {
            Label::new(cx, "Wave Index").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.wave_index_2);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Wave Warp").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.wave_warp_2);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Attack").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_attack_2);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Decay").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_decay_2);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Sustain").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_sustain_2);
        }).class("row");

        HStack::new(cx, |cx| {
            Label::new(cx, "Warp Release").class("label");
            ParamSlider::new(cx, Data::params, |params| &params.warp_release_2);
        }).class("row");

    })
    .class("quarter");
}
