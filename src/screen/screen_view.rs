use cursive::direction::Direction;
use cursive::direction::Orientation;
use cursive::theme::{BaseColor, Color, ColorStyle, PaletteColor};
use cursive::view::View;
use cursive::Printer;
use cursive::Vec2;
use cursive::XY;

use cursive::event::{Event, EventResult, Key};

pub struct ScreenView {
    pub line_position: Vec2,
}

impl View for ScreenView {
    fn draw(&self, printer: &Printer) {
        printer.with_color(
            ColorStyle::new(
                Color::Light(BaseColor::White),
                Color::Dark(BaseColor::Black),
            ),
            |printer| {
                printer.print_line(Orientation::Horizontal, self.line_position, 1, " ");
            },
        )
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        XY::new(32, 64)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh => EventResult::Consumed(None),
            _ => EventResult::Ignored
        }
    }
}
