use iced::widget::container;
use iced::{Element, Length, Sandbox, Settings};

use timer::timer;

pub fn main() -> iced::Result {
    Component::run(Settings::default())
}

#[derive(Default)]
struct Component {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Finished,
}

impl Sandbox for Component {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 10,
        }
    }

    fn title(&self) -> String {
        String::from("Component - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Finished => {
                self.value = self.value + 10;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(timer(self.value, Message::Finished))
            .padding(20)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

mod timer {
    use iced::alignment::{self, Alignment};
    use iced::widget::{self, button, row, column, text, text_input};
    use iced::{Element, Length};
    use iced::time::{Instant, Duration};
    use iced_lazy::{self, Component};

    use super::numeric_input::numeric_input;

    pub struct Timer<Message> {
        seconds: u32,
        seconds_setup: Option<u32>,
        on_finished: Message,
        is_started: bool,
    }

    pub fn timer<Message>(
        seconds: u32,
        on_finished: Message,
    ) -> Timer<Message> {
        Timer::new(seconds, on_finished)
    }

    #[derive(Debug, Clone)]
    pub enum Event {
        SecondsChanged(Option<u32>),
        Start,
        Decrement,
        Pause,
        Stop,
    }

    impl<Message> Timer<Message> {
        pub fn new(
            seconds: u32,
            on_finished: Message,
        ) -> Self {
            Self {
                seconds, seconds_setup: Some(seconds),
                on_finished: on_finished,
                is_started: false,
            }
        }
    }

    impl<Message, Renderer> Component<Message, Renderer> for Timer<Message>
    where
        Renderer: iced_native::text::Renderer + 'static,
        Renderer::Theme: widget::button::StyleSheet
            + widget::text_input::StyleSheet
            + widget::text::StyleSheet,
        Message: Clone
    {
        type State = ();
        type Event = Event;

        fn request_update(&self) -> Option<(Instant, fn(Instant) -> Event)> {
            if self.is_started {
                Some((Instant::now() + Duration::from_millis(500), |_now| Event::Decrement))
            } else {
                None
            }
        }

        fn update(
            &mut self,
            _state: &mut Self::State,
            event: Event,
        ) -> Option<Message> {
            match event {
                Event::Decrement if self.is_started => {
                    self.seconds = self.seconds.saturating_sub(1);
                    if self.seconds == 0 {
                        self.is_started = false;
                        Some(self.on_finished.clone())
                    } else {
                        None
                    }
                }
                Event::SecondsChanged(value) => {
                    self.seconds_setup = value;
                    None
                },
                Event::Start if self.seconds_setup.is_some() => {
                    self.seconds = self.seconds_setup.unwrap();
                    self.is_started = true;
                    None
                }
                Event::Pause => {self.is_started = false; None}
                _ => None,
            }
        }

        fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
            let seconds_input = numeric_input(self.seconds_setup, Event::SecondsChanged);

            let button = |label, on_press| {
                button(
                    text(label)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .width(Length::Units(100))
                .on_press(on_press)
            };

//             column![
                row![
                    if self.is_started {
                        button("Pause", Event::Pause)
                    } else {
                        button("Start", Event::Start)
                    },
                    if self.is_started {
                        Element::from(text(
                            self.seconds.to_string()
                        ))
                    } else {
                        seconds_input.into()
                    },
//                 ],
            ]
            .align_items(Alignment::Fill)
            .spacing(10)
            .into()
        }
    }

    impl<'a, Message, Renderer> From<Timer<Message>>
        for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: 'static + iced_native::text::Renderer,
        Renderer::Theme: widget::button::StyleSheet
            + widget::text_input::StyleSheet
            + widget::text::StyleSheet,
        Message: Clone,
    {
        fn from(numeric_input: Timer<Message>) -> Self {
            iced_lazy::component(numeric_input)
        }
    }
}


mod numeric_input {
    use iced::alignment::{self, Alignment};
    use iced::widget::{self, button, row, text, text_input};
    use iced::{Element, Length};
    use iced_lazy::{self, Component};

    pub struct NumericInput<Message> {
        value: Option<u32>,
        on_change: Box<dyn Fn(Option<u32>) -> Message>,
    }

    pub fn numeric_input<Message>(
        value: Option<u32>,
        on_change: impl Fn(Option<u32>) -> Message + 'static,
    ) -> NumericInput<Message> {
        NumericInput::new(value, on_change)
    }

    #[derive(Debug, Clone)]
    pub enum Event {
        InputChanged(String),
        IncrementPressed,
        DecrementPressed,
    }

    impl<Message> NumericInput<Message> {
        pub fn new(
            value: Option<u32>,
            on_change: impl Fn(Option<u32>) -> Message + 'static,
        ) -> Self {
            Self {
                value,
                on_change: Box::new(on_change),
            }
        }
    }

    impl<Message, Renderer> Component<Message, Renderer> for NumericInput<Message>
    where
        Renderer: iced_native::text::Renderer + 'static,
        Renderer::Theme: widget::button::StyleSheet
            + widget::text_input::StyleSheet
            + widget::text::StyleSheet,
    {
        type State = ();
        type Event = Event;

        fn update(
            &mut self,
            _state: &mut Self::State,
            event: Event,
        ) -> Option<Message> {
            match event {
                Event::IncrementPressed => Some((self.on_change)(Some(
                    self.value.unwrap_or_default().saturating_add(1),
                ))),
                Event::DecrementPressed => Some((self.on_change)(Some(
                    self.value.unwrap_or_default().saturating_sub(1),
                ))),
                Event::InputChanged(value) => {
                    if value.is_empty() {
                        Some((self.on_change)(None))
                    } else {
                        value
                            .parse()
                            .ok()
                            .map(Some)
                            .map(self.on_change.as_ref())
                    }
                }
            }
        }

        fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
            let button = |label, on_press| {
                button(
                    text(label)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .width(Length::Units(50))
                .on_press(on_press)
            };

            row![
                button("-", Event::DecrementPressed),
                text_input(
                    "Type a number",
                    self.value
                        .as_ref()
                        .map(u32::to_string)
                        .as_deref()
                        .unwrap_or(""),
                    Event::InputChanged,
                )
                .padding(10),
                button("+", Event::IncrementPressed),
            ]
            .align_items(Alignment::Fill)
            .spacing(10)
            .into()
        }
    }

    impl<'a, Message, Renderer> From<NumericInput<Message>>
        for Element<'a, Message, Renderer>
    where
        Message: 'a,
        Renderer: 'static + iced_native::text::Renderer,
        Renderer::Theme: widget::button::StyleSheet
            + widget::text_input::StyleSheet
            + widget::text::StyleSheet,
    {
        fn from(numeric_input: NumericInput<Message>) -> Self {
            iced_lazy::component(numeric_input)
        }
    }
}
