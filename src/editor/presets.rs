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

const PRESETS_PATH: &str = "/tmp/presets.json";

#[derive(Serialize, Deserialize)]
struct Presets {
    presets: HashMap<String, PluginState>,
}

impl Presets {
    fn new() -> Self {
        Self {
            presets: HashMap::new()
        }
    }

    // load tries to load from fs but falls back to the builtin 
    fn load(&self) -> Self {
        let presets_string = std::fs::read(PRESET_PATH).unwrap();
        serde_json::from_slice(&presets_string).unwrap()
    }

    // for now this will allow you to overwrite an existing preset
    fn add(&mut self, name: String, state: PluginState) -> Result<()> {
        self.presets.insert(name, state); 
        
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
        std::fs::write(PRESET_PATH, presets_string).unwrap();

    }
}

pub struct PresetMenu {
    gui_context: Arc<dyn GuiContext>,
    presets: Presets,
}

enum PresetMenuEvent {
    SavePreset,
    LoadPreset,
}

impl PresetMenu {
    pub fn new(
        cx: &mut Context,
        gcx: Arc<dyn GuiContext>,
    ) -> Handle<Self>
    {
        Self {
            gui_context: gcx,
            // TODO: replace with load() once we have some base presets to load
            presets: Presets::new(),
        }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
					Button::new(
						cx, 
						|ex| ex.emit(PresetMenuEvent::SavePreset), 
						|cx| Label::new(cx, "Save Preset")
					);
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
        event.map(|preset_event, _meta| match preset_event{
            PresetMenuEvent::SavePreset => {
                self.presets.add("first".to_string(), self.gui_context.get_state()).unwrap();

            }
            PresetMenuEvent::LoadPreset => {
                let json_state = std::fs::read(PRESET_PATH).unwrap();
                self.gui_context.set_state(serde_json::from_slice(&json_state).unwrap());

            }
        });
    }
}

