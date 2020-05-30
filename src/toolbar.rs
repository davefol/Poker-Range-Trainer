use iced::{
    button,
    Button,
    Row,
    Text,
};

use iced_native::{
    Align, Length
};

use crate::messages::Message;

#[derive(Default)]
pub struct ToolBar {
    study_button: button::State,
    ranges_button: button::State,
}

impl ToolBar {
    pub fn view(&mut self) -> Row<Message> {
        Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(24)
            .padding(24)
            .push(
                Button::new(&mut self.study_button, Text::new("Study"))
                    .on_press(Message::ViewStudyScreen),
                )
            .push(
                Button::new(&mut self.ranges_button, Text::new("Ranges"))
                    .on_press(Message::ViewRangesScreen),
                )
    }
}
