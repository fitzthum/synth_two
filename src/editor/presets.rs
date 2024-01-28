// Widget for handling presets.
//
// To start with this will just be a button to load/save
//
use std::sync::Arc;

use nih_plug_vizia::vizia::prelude::*;
use nih_plug::context::gui::GuiContext;

const PRESET_PATH: &str = "/tmp/preset.json";

pub struct PresetMenu {
    gui_context: Arc<dyn GuiContext>,
}

enum PresetMenuEvent {
    SavePreset,
    // TODO: Load preset
}

impl PresetMenu {
    pub fn new(
        cx: &mut Context,
        gcx: Arc<dyn GuiContext>,
    ) -> Handle<Self>
    {
        Self { gui_context: gcx }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
					Button::new(
						cx, 
						|ex| ex.emit(PresetMenuEvent::SavePreset), 
						|cx| Label::new(cx, "Save Preset")
					);
                });
            })
    }
}

impl View for PresetMenu {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|preset_event, _meta| match preset_event{
            PresetMenuEvent::SavePreset => {
                let json_state = serde_json::to_string(&self.gui_context.get_state()).unwrap();
                std::fs::write(PRESET_PATH, json_state).unwrap();

            }
        });
    }
}

