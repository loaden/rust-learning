use iced::alignment::Horizontal;
use iced::widget::{button, checkbox, horizontal_space, slider, text, text_input, toggler};
use iced::widget::{column, container, row};
use iced::widget::{Column, Container};
use iced::{Size, window};
use iced::{executor, Color};
use iced::{Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: Size {height: 600.0, width: 800.0},
            ..Default::default()
        },
        ..Default::default()
    };
    App::run(settings)
}

struct App {
    steps: Steps,
    debug: bool,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                steps: Steps::new(),
                debug: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let Self { steps, debug } = self;
        match message {
            Message::PageNext => {
                steps.go_next();
                Command::none()
            }
            Message::PageBack => {
                steps.go_back();
                Command::none()
            }
            Message::StepMessage(msg) => {
                steps.update(msg, debug);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Self { steps, .. } = self;

        let mut controls = row![];
        if steps.can_back() {
            controls = controls.push(button("Back").on_press(Message::PageBack));
        }

        controls = controls.push(horizontal_space());
        if steps.can_next() {
            controls = controls.push(button("Next").on_press(Message::PageNext));
        }

        let content: Element<_> =
            column![steps.view(self.debug).map(Message::StepMessage), controls,]
                .max_width(600)
                .spacing(20)
                .padding(20)
                .into();

        container(if self.debug {
            content.explain(Color::from_rgb8(0, 180, 0))
        } else {
            content
        })
        .width(Length::Fill)
        .center_x()
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PageNext,
    PageBack,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self {
            steps: vec![
                Step::Welcome,
                Step::Debugger,
                Step::Toggler {
                    can_continue: false,
                },
                Step::Slider { value: 36 },
                Step::TextInput {
                    value: String::new(),
                    is_secure: false,
                },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        self.steps[self.current].update(msg, debug);
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
    }

    fn go_next(&mut self) {
        if self.can_next() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.can_back() {
            self.current -= 1;
        }
    }

    fn can_next(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn can_back(&self) -> bool {
        self.current > 0
    }
}

enum Step {
    Welcome,
    Debugger,
    Toggler { can_continue: bool },
    Slider { value: u8 },
    TextInput { value: String, is_secure: bool },
    End,
}

#[derive(Debug, Clone)]
enum StepMessage {
    DebugToggled(bool),
    TogglerChanged(bool),
    SliderChanged(u8),
    InputChanged(String),
    ToggleSecureInput(bool),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        match msg {
            StepMessage::SliderChanged(val) => {
                if let Self::Slider { value } = self {
                    *value = val;
                }
            }
            StepMessage::DebugToggled(dbg) => {
                if let Self::Debugger = self {
                    *debug = dbg;
                }
            }
            StepMessage::TogglerChanged(value) => {
                if let Self::Toggler { can_continue } = self {
                    *can_continue = value;
                }
            }
            StepMessage::InputChanged(str) => {
                if let Self::TextInput { value, .. } = self {
                    *value = str;
                }
            }
            StepMessage::ToggleSecureInput(flag) => {
                if let Self::TextInput { is_secure, .. } = self {
                    *is_secure = flag;
                }
            }
        }
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        match self {
            Self::Welcome => Self::welcome(),
            Self::Debugger => Self::debugger(debug),
            Self::Toggler { can_continue } => Self::toggler(*can_continue),
            Self::Slider { value } => Self::slider(*value),
            Self::TextInput { value, is_secure } => Self::text_input(value, *is_secure),
            Self::End => Self::end(),
        }
        .into()
    }

    fn can_continue(&self) -> bool {
        if let Self::Toggler { can_continue } = self {
            *can_continue
        } else if let Self::TextInput { value, .. } = self {
            !value.is_empty()
        } else {
            true
        }
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        column![text(title).size(50)].spacing(20)
    }

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome!").push(
            "This is a simple tour meant to showcase a bunch of widgets \
                that can be easily implemented on top of Iced.",
        )
    }

    fn debugger(debug: bool) -> Column<'a, StepMessage> {
        Self::container("Debugger")
            .push(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            )
            .push(checkbox("Explain layout", debug))
    }

    fn toggler(can_continue: bool) -> Column<'a, StepMessage> {
        Self::container("Toggler")
            .push("A toggler is mostly used to enable or disable something.")
            .push(
                Container::new(toggler(
                    "Toggle me to continue...".to_owned(),
                    can_continue,
                    StepMessage::TogglerChanged,
                ))
                .padding([0, 40]),
            )
    }

    fn slider(value: u8) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            )
            .push(slider(0..=100, value, StepMessage::SliderChanged))
            .push(
                text(value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            )
    }

    fn text_input(value: &str, is_secure: bool) -> Column<'a, StepMessage> {
        let text_input = text_input("Type something to continue...", value)
            .on_input(StepMessage::InputChanged)
            .padding(10)
            .size(30);

        Self::container("Text input")
            .push("Use a text input to ask for different kinds of information.")
            .push(if is_secure {
                text_input.secure(is_secure)
            } else {
                text_input
            })
            .push(checkbox(
                "Enable password mode",
                is_secure
            ))
            .push(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            )
            .push(
                text(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            )
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }
}
