use iced::widget::{button, checkbox, column, container, row};
use iced::{Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Box::run(Settings::default())
}

struct Box {
    state: State,
    debug: bool,
}

struct State {
    pages: Vec<Page>,
    page: Page,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextPage,
    PrevPage,
    DebugMode(bool),
}

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Welcome,
    CheckBox,
    End,
}

impl State {
    fn new() -> Self {
        Self {
            pages: vec![Page::Welcome, Page::CheckBox, Page::End],
            page: Page::Welcome,
        }
    }

    fn next_page(&mut self) {
        let idx = self.pages.iter().position(|i| *i == self.page);
        if let Some(i) = idx {
            self.page = *self.pages.get(i + 1).unwrap();
        }
    }

    fn prev_page(&mut self) {
        let idx = self.pages.iter().position(|i| *i == self.page);
        if let Some(i) = idx {
            self.page = *self.pages.get(i - 1).unwrap();
        }
    }
}

impl Sandbox for Box {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: State::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        String::from("State - Iced")
    }

    fn update(&mut self, message: Message) {
        let Box { state, debug } = self;
        match message {
            Message::DebugMode(b) => *debug = b,
            Message::NextPage => state.next_page(),
            Message::PrevPage => state.prev_page(),
        }
    }

    fn view(&self) -> Element<Message> {
        let Box { state, .. } = self;
        let mut content = match state.page {
            Page::Welcome => column!["Welcome!"],
            Page::CheckBox => column![checkbox("Debug mode", self.debug, Message::DebugMode),],
            Page::End => column!["End"],
        };

        content = match state.page {
            Page::Welcome => content.push(button("Next").on_press(Message::NextPage)),
            Page::End => content.push(button("Prev").on_press(Message::PrevPage)),
            _ => content.push(
                row![
                    button("Prev").on_press(Message::PrevPage),
                    button("Next").on_press(Message::NextPage),
                ]
                .spacing(10),
            ),
        };

        container(content.spacing(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
