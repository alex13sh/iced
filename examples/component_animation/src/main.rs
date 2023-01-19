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
        Self::default()
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
    use iced::widget::{self, button, row, text, text_input};
    use iced::{Element, Length};
    use iced::time::{Instant, Duration};
    use iced_lazy::{self, Component};

    pub struct Timer<Message> {
        seconds: u32,
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
        InputChanged(String),
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
                seconds,
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

        fn request_update(&self, now: &Instant) -> Option<(Instant, fn(Instant) -> Event)> {
            if self.is_started {
                Some((*now + Duration::from_millis(1000), |_now| Event::Decrement))
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
                Event::InputChanged(value) => {
                    if let Some(v) = value.parse().ok() {
                        self.seconds = v;
                    }
                    None
                },
                Event::Start => {self.is_started = true; None}
                Event::Pause => {self.is_started = false; None}
                _ => None,
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
                text_input(
                    "Seconds",
                    &self.seconds
                        .to_string(),
                    Event::InputChanged,
                )
                .padding(10),
                if self.is_started {
                    button("Pause", Event::Pause)
                } else {
                    button("Start", Event::Start)
                },
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
