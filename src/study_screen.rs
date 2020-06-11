use crate::messages::Message;

use crate::review;

use iced::{Container, Text};

pub struct StudyScreen {
    review_display: Option<review::display::ReviewDisplay>,
}

impl StudyScreen {
    pub fn new(review_display: Option<review::display::ReviewDisplay>) -> Self {
        Self { review_display }
    }

    pub fn view(&mut self) -> Container<Message> {
        match &mut self.review_display {
            Some(review_display) => review_display.view(),
            None => Container::new(Text::new("No available questions.")),
        }
    }

    pub fn update(&mut self, message: review::display::ReviewDisplayMessage) {
        match &mut self.review_display {
            Some(review_display) => review_display.update(message),
            None => {}
        }
    }
}
