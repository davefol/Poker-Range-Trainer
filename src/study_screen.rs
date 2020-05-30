use crate::messages::{
    Message, StudyMessage
};
use crate::range::Range;
use crate::hand::Hand;

use iced::{Column, Row, button, Button, Text,};
use iced_native::{
    Length, 
    Element, Align,
};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, Eq)]
enum AnswerState {
    Waiting,
    Correct,
    Wrong
}

struct Question {
    range: Range,
    hand: Hand,
    state: AnswerState,
}

impl Question {
    fn new(ranges: &Vec<Range>) -> Option<Self> {
        let mut rng = rand::thread_rng();
        match ranges.choose(&mut rng) {
            Some(range) => {
                Some(Self{
                    range: range.clone(),
                    hand: rng.gen(),
                    state: AnswerState::Waiting,
                })
            }, 
            None => None
        }
    }
}

pub struct StudyScreen {
    question: Option<Question>,
    yes_button: button::State,
    no_button: button::State,
    next_button: button::State,
}

impl StudyScreen {
    pub fn new(ranges: &Vec<Range>) -> Self {
        Self {
            question: Question::new(ranges),
            yes_button: button::State::new(),
            no_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        Row::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .push(
            match &self.question {
                Some(question) => {
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .spacing(100)
                        .push(Text::new(question.range.name.to_string()))
                        .push(Text::new(question.hand.to_string()))
                        .push(
                            match question.state {
                                AnswerState::Waiting => Element::from(Row::new()
                                  .push(Button::new(&mut self.yes_button, Text::new("Yes"))
                                        .on_press(Message::StudyScreen(StudyMessage::Answer(true)))
                                        )
                                  .push(Button::new(&mut self.no_button, Text::new("No"))
                                        .on_press(Message::StudyScreen(StudyMessage::Answer(false)))
                                        )),
                                AnswerState::Correct => Element::from(Column::new()
                                    .push(Text::new("Correct."))
                                    .push(Button::new(&mut self.next_button, Text::new("Next."))
                                          .on_press(Message::RequestNewQuestion)
                                          )),
                                AnswerState::Wrong => Element::from(Column::new()
                                    .push(Text::new("Wrong."))
                                    .push(Button::new(&mut self.next_button, Text::new("Next."))
                                          .on_press(Message::RequestNewQuestion)
                                          )),
                            }
                        )
                    },
                None => {
                    Column::new()
                        .push(Text::new("No available questions"))
                }
            }
        )
    }

    pub fn update(&mut self, message: StudyMessage) {
        match message {
            StudyMessage::Answer(answer) => {
                if let Some(question) = &mut self.question {
                    if question.range.contains(&question.hand) == answer {
                        question.state = AnswerState::Correct;
                    } else {
                        question.state = AnswerState::Wrong;
                    }
                }
            },
            StudyMessage::NewQuestion(ranges) => {
                self.question = Question::new(&ranges);
            }
        }
    }
}
