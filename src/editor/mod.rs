use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::SynthTwoParams;

#[derive(Lens)]
struct Data {
    params: Arc<SynthTwoParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (1200, 1000))
}

pub(crate) fn create(
    params: Arc<SynthTwoParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

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
    });
}

fn top_left(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Gain");
        ParamSlider::new(cx, Data::params, |params| &params.gain);
 
    });
}

fn top_right(cx: &mut Context) {
    VStack::new(cx, |cx| {
        // ADSR
        Label::new(cx, "Attack");
        ParamSlider::new(cx, Data::params, |params| &params.attack);

        Label::new(cx, "Decay");
        ParamSlider::new(cx, Data::params, |params| &params.decay);

        Label::new(cx, "Sustain");
        ParamSlider::new(cx, Data::params, |params| &params.sustain);

        Label::new(cx, "Release");
        ParamSlider::new(cx, Data::params, |params| &params.release);

    });
}

fn oscillators(cx: &mut Context) {
    HStack::new(cx, |cx| {
        oscillator1(cx);
        oscillator2(cx);

    });
}

// in theory this should be generic but idk how
fn oscillator1(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Wave Index");
        ParamSlider::new(cx, Data::params, |params| &params.wave_index_1);

        Label::new(cx, "Wave Warp");
        ParamSlider::new(cx, Data::params, |params| &params.wave_warp_1);

        Label::new(cx, "Warp Attack");
        ParamSlider::new(cx, Data::params, |params| &params.warp_attack_1);

        Label::new(cx, "Warp Decay");
        ParamSlider::new(cx, Data::params, |params| &params.warp_decay_1);

        Label::new(cx, "Warp Sustain");
        ParamSlider::new(cx, Data::params, |params| &params.warp_sustain_1);

        Label::new(cx, "Warp Release");
        ParamSlider::new(cx, Data::params, |params| &params.warp_release_1);

    });
}

fn oscillator2(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Wave Index");
        ParamSlider::new(cx, Data::params, |params| &params.wave_index_2);

        Label::new(cx, "Wave Warp");
        ParamSlider::new(cx, Data::params, |params| &params.wave_warp_2);

        Label::new(cx, "Warp Attack");
        ParamSlider::new(cx, Data::params, |params| &params.warp_attack_2);

        Label::new(cx, "Warp Decay");
        ParamSlider::new(cx, Data::params, |params| &params.warp_decay_2);

        Label::new(cx, "Warp Sustain");
        ParamSlider::new(cx, Data::params, |params| &params.warp_sustain_2);

        Label::new(cx, "Warp Release");
        ParamSlider::new(cx, Data::params, |params| &params.warp_release_2);

    });
}


