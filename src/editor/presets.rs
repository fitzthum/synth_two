// Widget for handling presets.
//
// To start with this will just be a button to load/save
//
// Presets are stored in a JSON dictionary stored locally (where?)
// Save the default ones with include data
// When you first create this thing, check if there is a local file
//
// what to do about the default?
//
// - update the save thing to add a dictinoary around the preset
// - put this into include bytes
// - make a dropdown with one element coming from include bytes
// - add more
// - add local mechanism described above
// - add saving of new presets (could do this first, actually)

use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use nih_plug_vizia::vizia::prelude::*;
use nih_plug::context::gui::GuiContext;
use nih_plug::wrapper::state::PluginState;
use std::collections::HashMap;

// TODO: make this OS independent
const PRESETS_PATH: &str = "/tmp/presets.json";

#[derive(Serialize, Deserialize)]
pub struct Presets {
    presets: HashMap<String, PluginState>,
}

impl Presets {

    // load tries to load from fs but falls back to the builtin
    fn load() -> Self {
        let presets_string = match std::path::Path::new(PRESETS_PATH).exists() {
            true => std::fs::read(PRESETS_PATH).unwrap(),
            false => include_str!("../../presets.json").into(),
        };
        Self {
            presets: serde_json::from_slice(&presets_string).unwrap(),
        }
    }

    // for now this will allow you to overwrite an existing preset
    fn add(&mut self, name: &String, state: PluginState) -> Result<()> {
        self.presets.insert(name.to_string(), state);

        // propogate the changes back to the fs
        self.save();

        Ok(())

    }

    fn get(&self, name: String) -> PluginState {
        self.presets.get(&name).unwrap().clone()
    }

    // save always saves to the fs location
    fn save(&self) {
        let presets_string = serde_json::to_string(&self.presets).unwrap();
        std::fs::write(PRESETS_PATH, presets_string).unwrap();

    }
}

#[derive(Lens)]
pub struct PresetMenu {
    gui_context: Arc<dyn GuiContext>,
    presets: Presets,
    new_preset_name: String,
}

enum PresetMenuEvent {
    SavePreset,
    LoadPreset,
    UpdateNewPresetName(String)
}

impl PresetMenu {
    pub fn new(
        cx: &mut Context,
        gcx: Arc<dyn GuiContext>,
    ) -> Handle<Self>
    {
        Self {
            gui_context: gcx,
            presets: Presets::load(),
            new_preset_name: "".to_string(),
        }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
					Button::new(
						cx,
						|ex| ex.emit(PresetMenuEvent::SavePreset),
						|cx| Label::new(cx, "Save Preset")
					);
                    Textbox::new(cx, PresetMenu::new_preset_name)
                        .on_edit(|cx, text| cx.emit(PresetMenuEvent::UpdateNewPresetName(text)))
                        .width(Pixels(200.0))
                        .on_build(|cx| {
                            cx.emit(TextEvent::StartEdit);
                            cx.emit(TextEvent::SelectAll);
                        });
					Button::new(
						cx,
						|ex| ex.emit(PresetMenuEvent::LoadPreset),
						|cx| Label::new(cx, "Load Preset")
					);

                });
            })
    }
}

impl View for PresetMenu {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|preset_event, _| match preset_event {
            PresetMenuEvent::SavePreset => {
                self.presets.add(&self.new_preset_name, self.gui_context.get_state()).unwrap();

            }
            PresetMenuEvent::LoadPreset => {
                let json_state = std::fs::read(PRESETS_PATH).unwrap();
                self.gui_context.set_state(serde_json::from_slice(&json_state).unwrap());

            }
            PresetMenuEvent::UpdateNewPresetName(name) => {
                self.new_preset_name = name.to_string();
            }
        });
    }
}

