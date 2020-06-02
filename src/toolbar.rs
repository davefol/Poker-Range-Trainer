use iced::{button, Button, Container, Row, Text};

use iced_native::{Align, Length};

use crate::messages::Message;
use crate::styles;

#[derive(Default)]
pub struct ToolBar {
    study_button: button::State,
    ranges_button: button::State,
}

impl ToolBar {
    pub fn view(&mut self) -> Container<Message> {
        Container::new(
            Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .spacing(24)
                .push(
                    Button::new(&mut self.study_button, Text::new("Study"))
                        .on_press(Message::ViewStudyScreen)
                        .style(styles::Button::Toolbar),
                )
                .push(
                    Button::new(&mut self.ranges_button, Text::new("Ranges"))
                        .on_press(Message::ViewRangesScreen)
                        .style(styles::Button::Toolbar),
                ),
        )
        .style(styles::Container::Basic)
        .align_y(Align::Start)
        .width(Length::Fill)
        .padding(4)
    }
}
