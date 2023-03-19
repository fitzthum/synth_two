use nih_plug::prelude::{Editor, GuiContext};
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
    attack_slider_state: nih_widgets::param_slider::State,
    decay_slider_state: nih_widgets::param_slider::State,
    sustain_slider_state: nih_widgets::param_slider::State,
    release_slider_state: nih_widgets::param_slider::State,
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
            attack_slider_state: Default::default(),
            decay_slider_state: Default::default(),
            sustain_slider_state: Default::default(),
            release_slider_state: Default::default(),
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
            Column::new().push(Space::with_height(20.into()))
                .push(Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Gain")
                            .height(27.into())
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
            )
                .push(Space::with_height(10.into()))
                .push(Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Attack")
                            .height(27.into())
                            .width(Length::Shrink)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::with_width(5.into()))
                    .push(
                        nih_widgets::ParamSlider::new(
                            &mut self.attack_slider_state,
                            &self.params.attack,
                        )
                        .map(Message::ParamUpdate),
                    ),
            )
                .push(Space::with_height(10.into()))
                .push(Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Decay")
                            .height(27.into())
                            .width(Length::Shrink)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::with_width(5.into()))
                    .push(
                        nih_widgets::ParamSlider::new(
                            &mut self.decay_slider_state,
                            &self.params.decay,
                        )
                        .map(Message::ParamUpdate),
                    ),
            )
                .push(Space::with_height(10.into()))
                .push(Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Sustain")
                            .height(27.into())
                            .width(Length::Shrink)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::with_width(5.into()))
                    .push(
                        nih_widgets::ParamSlider::new(
                            &mut self.sustain_slider_state,
                            &self.params.sustain,
                        )
                        .map(Message::ParamUpdate),
                    ),
            )
                .push(Space::with_height(10.into()))
                .push(Row::new()
                    .align_items(Alignment::Start)
                    .push(Space::with_width(20.into()))
                    .push(
                        Text::new("Release")
                            .height(27.into())
                            .width(Length::Shrink)
                            .horizontal_alignment(alignment::Horizontal::Left)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(Space::with_width(5.into()))
                    .push(
                        nih_widgets::ParamSlider::new(
                            &mut self.release_slider_state,
                            &self.params.release,
                        )
                        .map(Message::ParamUpdate),
                    ),
            )

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
