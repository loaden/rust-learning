use iced::mouse;
use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path};
use iced::{Element, Length, Point, Rectangle, Renderer, Sandbox, Settings, Theme};

fn main() -> iced::Result {
    Ewl::run(Settings::default())
}

struct Ewl {
    cache: Cache,
}

impl Sandbox for Ewl {
    type Message = ();

    fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Ewl - Iced")
    }

    fn update(&mut self, _: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl<Message> canvas::Program<Message> for Ewl {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let palette = theme.palette();

            let sun = frame.center();
            let radius = frame.width().min(frame.height()) / 5.0;
            let earth = Point::new(sun.x + radius, sun.y + radius);

            let circles = Path::new(|b| {
                b.circle(sun, radius / 2.0);
                b.move_to(earth);
                b.circle(earth, radius / 10.0);
            });

            frame.fill(&circles, palette.text);
        });

        vec![geometry]
    }
}
