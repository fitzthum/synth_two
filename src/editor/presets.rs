// Widget for handling presets.
//
//  A preset menu
//
//  FIXME: edit event does not trigger on textbox

use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use nih_plug_vizia::vizia::prelude::*;
use nih_plug::context::gui::GuiContext;
use nih_plug::wrapper::state::PluginState;
use std::collections::HashMap;

// TODO: make this OS independent
const PRESETS_PATH: &str = "/tmp/presets.json";

#[derive(Serialize, Deserialize, Clone)]
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
    preset_names: Vec<String>,
    new_preset_name: String,
	selected_preset: String,
}

enum PresetMenuEvent {
    SavePreset,
    LoadPreset,
    UpdateNewPresetName(String),
	UpdatePresetSelection(String),
}

impl PresetMenu {
    pub fn new(
        cx: &mut Context,
        gcx: Arc<dyn GuiContext>,
    ) -> Handle<Self>
    {
        let presets = Presets::load();
        let preset_names = Vec::from_iter(presets.presets.clone().into_keys());

        Self {
            gui_context: gcx,
            presets: presets.clone(),
            preset_names: preset_names.clone(),
            new_preset_name: "".to_string(),
            // hopefully there is no preset with this name
			selected_preset: "Default".to_string(),
        }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Presets");
                // some kind of dropdown thing here
                //


                Dropdown::new(cx,
                    move |cx| {
                        // make a label to fill the top
                        HStack::new(cx, move |cx| {
                            Label::new(cx, PresetMenu::selected_preset);
                            Label::new(cx, "∨");
                        }).class("dd-label")
                    },

                    // make the list of options
                    move |cx| {
                        VStack::new(cx, |cx| {
                            // need some binding here if the stuff is updated
                            for preset_name in preset_names.clone() {

                                // keep us from running into problems with the move closure
                                // below. maybe there is a better way
                                let name = preset_name.clone();

                                Label::new(cx, &preset_name)
                                    .bind(PresetMenu::selected_preset, move |handle, selected| {
                                        if preset_name == selected.get(handle.cx) {
                                            handle.class("selected");
                                        }
                                    })
                                    .on_press(move |cx| {
                                        cx.emit(PresetMenuEvent::UpdatePresetSelection(name.clone()));
                                        cx.emit(PopupEvent::Close);
                                    });
                                }
                        }).class("dd-content");
                    }
                ).class("dropdown");

                Textbox::new(cx, PresetMenu::new_preset_name)
                    .on_edit(|cx, text| cx.emit(PresetMenuEvent::UpdateNewPresetName(text)))
                    .on_build(|cx| {
                        cx.emit(TextEvent::StartEdit);
                        cx.emit(TextEvent::SelectAll);
                    })
                    .id("preset-name-box");

                Button::new(
                    cx,
                    |ex| ex.emit(PresetMenuEvent::SavePreset),
                    |cx| Label::new(cx, "Save")
                );

            })
            .class("row")
            .col_between(Pixels(30.0));
        })
        .class("section")
        .id("preset-browser")
    }
}

impl View for PresetMenu {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|preset_event, _| match preset_event {
            PresetMenuEvent::SavePreset => {
                self.presets.add(&self.new_preset_name, self.gui_context.get_state()).unwrap();

            }
            PresetMenuEvent::LoadPreset => {
                let state = self.presets.get(self.selected_preset.clone());
                self.gui_context.set_state(state);

            }
            PresetMenuEvent::UpdateNewPresetName(name) => {
                self.new_preset_name = name.to_string();
            }
		    PresetMenuEvent::UpdatePresetSelection(name) => {
                self.selected_preset = name.to_string();
            }
        });
    }
}

