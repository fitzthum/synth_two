use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;

use crate::SynthTwoParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(600, 450)
}

pub(crate) fn create(
    params: Arc<SynthTwoParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<SynthTwoEditor>(editor_state, params)
}

struct SynthTwoEditor {
    params: Arc<SynthTwoParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for SynthTwoEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<SynthTwoParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = SynthTwoEditor {
            params,
            context,

            gain_slider_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Container::new(
            Column::new().push(Space::with_height(20.into())).push(
                Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Gain")
                            .height(26.into())
                            .width(Length::Shrink)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::with_width(5.into()))
                    .push(
                        nih_widgets::ParamSlider::new(
                            &mut self.gain_slider_state,
                            &self.params.gain,
                        )
                        .map(Message::ParamUpdate),
                    ),
            ),
        )
        .height(Length::Fill)
        .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
