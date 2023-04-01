// Can we make a modular knob thing?

use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;
use nih_plug::prelude::Param;

#[derive(Lens)]
pub struct ParamKnob {
    param_base: ParamWidgetBase,

}

enum ParamKnobEvent {
    ValUpdate(f32),
}

impl ParamKnob {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self { 
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params.clone(), params_to_param, move |cx, param_data| {
                VStack::new(cx, |cx| {
                    // Label
                    Label::new(cx, param_data.param().name());
                    
                    let value_lens = param_data.make_lens(|param| param.unmodulated_normalized_value());
                    Knob::new(cx, 0.5, value_lens, false)
                        .on_changing(move |cx, val| {
                            cx.emit(ParamKnobEvent::ValUpdate(val));
                        })
                        .color(Color::red());
                });
            }),
        )
    }

}

impl View for ParamKnob {
    fn element(&self) -> Option<&'static str> {
        Some("param-knob")

    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|knob_event, meta| match knob_event {
            ParamKnobEvent::ValUpdate(val) => {
                self.param_base.begin_set_parameter(cx);
                self.param_base.set_normalized_value(cx, *val);
                self.param_base.end_set_parameter(cx);

                meta.consume();
            },
        });
    }
}
