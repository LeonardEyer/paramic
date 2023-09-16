use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;

use crate::OscillatorTestParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(300, 200)
}

pub(crate) fn create(
    params: Arc<OscillatorTestParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<OscillatorTestEditor>(editor_state, params)
}

struct OscillatorTestEditor {
    params: Arc<OscillatorTestParams>,
    context: Arc<dyn GuiContext>,
    gain_slider_state: nih_widgets::param_slider::State,
    frequency_slider_state: nih_widgets::param_slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for OscillatorTestEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<OscillatorTestParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = OscillatorTestEditor {
            params,
            context,
            gain_slider_state: Default::default(),
            frequency_slider_state: Default::default(),
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
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("Oscillator Test GUI")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new("Gain")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            .push(
                Text::new("Frequency")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.frequency_slider_state, &self.params.frequency)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
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