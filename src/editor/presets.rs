// Widget for handling presets.
//
// To start with this will just be a button to load/save
//

use nih_plug_vizia::vizia::prelude::*;

const PRESET_PATH: &str = "/tmp/preset.json";

pub struct PresetMenu { }

enum PresetMenuEvent {
    SavePreset,
    // TODO: Load preset
}

impl PresetMenu {
    pub fn new(
        cx: &mut Context,
    ) -> Handle<Self>
    {
        Self { }
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
                std::fs::write(PRESET_PATH, "test").unwrap();

            }
        });
    }
}

