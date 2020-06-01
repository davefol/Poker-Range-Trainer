use iced::{
    button, Row, Container, Column, Text, widget, Button,
};
use iced_native::{
    Length, Align, VerticalAlignment, Element,
};

use crate::messages::{Message};
use crate::study_screen::{StudyMessage};
use crate::range::Range;
use crate::hand::Hand;
use crate::questions::{
    QuestionState,
    QuestionAnswer,
    Question,
};

use rand::{seq::SliceRandom, Rng};

#[derive(Debug, Clone)]
pub struct BinaryRangeQuestion {
    range: Range,
    hand: Hand,
    state: QuestionState,
    first_card_svg: &'static [u8],
    second_card_svg: &'static [u8],
    yes_button: button::State,
    no_button: button::State,
    next_button: button::State,
}

impl BinaryRangeQuestion {
    pub fn new(ranges: &Vec<Range>) -> Self {
        let mut rng = rand::thread_rng();
        let hand: Hand = rng.gen();
        let (first_svg, second_svg) = <(&[u8], &[u8])>::from(hand);
        match ranges.choose(&mut rng) {
            Some(range) => {
                Self{
                    range: range.clone(),
                    hand: hand,
                    state: QuestionState::Waiting,
                    first_card_svg: first_svg,
                    second_card_svg: second_svg,
                    yes_button: button::State::new(),
                    no_button: button::State::new(),
                    next_button: button::State::new(),
                }
            }, 
            None => unreachable!()
        }
    }
}

impl Question for BinaryRangeQuestion {
    fn view(&mut self) -> Row<Message> {
        Row::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .push(
                Container::new( 
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .spacing(100)
                        .push(Text::new(self.range.name.to_string())
                              .vertical_alignment(VerticalAlignment::Center))
                        .push(Row::new()
                            .spacing(8)
                            .push(widget::svg::Svg::new(widget::svg::Handle::from_memory(self.first_card_svg)))
                            .push(widget::svg::Svg::new(widget::svg::Handle::from_memory(self.second_card_svg)))
                              )
                        .push(
                            match self.state {
                                QuestionState::Waiting => Element::from(Row::new()
                                  .spacing(8)
                                  .push(Button::new(&mut self.yes_button, Text::new("Yes"))
                                        .on_press(Message::StudyScreen(StudyMessage::QuestionAnswer(QuestionAnswer::BinaryRangeQuestion(true))))
                                        )
                                  .push(Button::new(&mut self.no_button, Text::new("No"))
                                        .on_press(Message::StudyScreen(StudyMessage::QuestionAnswer(QuestionAnswer::BinaryRangeQuestion(false))))
                                        )),
                                QuestionState::Correct => Element::from(Row::new()
                                    .push(Button::new(&mut self.next_button, Text::new(format!("Correct")))
                                          .on_press(Message::RequestNewQuestion)
                                          )),
                                QuestionState::Wrong => Element::from(Row::new()
                                    .push(Button::new(&mut self.next_button, Text::new("Wrong"))
                                          .on_press(Message::RequestNewQuestion)
                                          )),
                            }
                        )
            )
                .center_x()
                .center_y()
        )
        
    }
    fn update(&mut self, message: QuestionAnswer) {
       match message {
            QuestionAnswer::BinaryRangeQuestion(true) => {
                if self.range.contains(&self.hand) {
                    self.state = QuestionState::Correct;
                } else {
                    self.state = QuestionState::Wrong;
                }
            },
            QuestionAnswer::BinaryRangeQuestion(false) => {
                if self.range.contains(&self.hand) {
                    self.state = QuestionState::Wrong;
                } else {
                    self.state = QuestionState::Correct;
                }
            },
            _ => unreachable!()
       }
    }
}
