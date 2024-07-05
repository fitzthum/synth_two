// Modular oscillator view 

use std::collections::HashMap;

use nih_plug::params::Params;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug::params::internals::ParamPtr;

use crate::editor::ParamKnob;

#[derive(Lens)]
pub struct Oscillator { }

impl Oscillator {

pub fn new<L, PsRef, Ps>(cx: &mut Context, params: L) -> Handle<'_, Self>
    where
        L: Lens<Target = PsRef> + Clone,
        PsRef: AsRef<Ps> + 'static,
        Ps: Params + 'static,
    {
        Self { }
        .build(cx, |cx| {
                
			VStack::new(cx, |cx| {
				Label::new(cx, "Oscillator").class("section-title");

                let mut params_map = HashMap::<String,ParamPtr>::new();

                // questionable method name
                let params_list = params.clone().map(|p| p.as_ref().param_map()).get(cx);
                for (name, ptr, _) in params_list {
                    params_map.insert(name, ptr);
                    
                };

                // radio boxes. don't bother making this dynamic yet
                if let ParamPtr::EnumParam(ptr) = params_map.get("bank-id").unwrap().clone() {
                    HStack::new(cx, |cx| {
                        let id = "basic";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });

                        let id = "original";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });

                        let id = "sample1";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });

                        let id = "sample2";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });

                        let id = "wanderer1";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });

                        let id = "wanderer2";
                        RadioButton::new(cx, params.clone().map(move |_| unsafe { (*ptr).unmodulated_plain_id().unwrap() == id }))
                            .on_select(move |_| unsafe { (*ptr).set_from_id(id); });



                    });
                };

                HStack::new(cx, |cx| {
                    if let ParamPtr::FloatParam(ptr) = params_map.get("wave-index-start").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("Start")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("wave-index-end").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("End")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("tuning").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("Tune")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("tuning-fine").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("Fine")) };
                    };
                }).class("row");

                
                HStack::new(cx, |cx| {
                    if let ParamPtr::FloatParam(ptr) = params_map.get("warp-attack").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("A")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("warp-decay").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("D")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("warp-sustain").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("S")) };
                    };

                    if let ParamPtr::FloatParam(ptr) = params_map.get("warp-release").unwrap().clone() {
                        unsafe { ParamKnob::new(cx, params.clone(), move |_| &*ptr, Some("R")) };
                    };

                }).class("row");
            }).class("section");
        })
    }
}

impl View for Oscillator {
    fn element(&self) -> Option<&'static str> {
        Some("Oscillator")
    }

    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {
        //TODO 
    }
}
