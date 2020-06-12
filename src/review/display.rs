use super::item;
use crate::messages::Message;
use crate::styles;
use iced::{widget, Align, Button, Column, Container, Length, Row, Text, TextInput};
use iced_native::{button, text_input};

#[derive(Debug, Clone)]
pub enum ReviewDisplayMessage {
    AnswerChanged(String),
    AnswerSubmitted(bool),
}

enum ReviewDisplayState {
    Waiting,
    Correct,
    Wrong,
}

pub struct ReviewDisplay {
    review_item: item::ReviewItem,
    state: ReviewDisplayState,
    next_button: button::State,
    yes_button: button::State,
    no_button: button::State,
    submit_button: button::State,
    fold_button: button::State,
    check_button: button::State,
    raise_bet_button: button::State,
    call_button: button::State,
    answer_input: text_input::State,
    answer_value: String,
    cached_hand_svg: Option<(&'static [u8], &'static [u8])>,
}

impl ReviewDisplay {
    pub fn new(review_item: item::ReviewItem) -> Self {
        Self {
            review_item,
            state: ReviewDisplayState::Waiting,
            next_button: button::State::new(),
            yes_button: button::State::new(),
            no_button: button::State::new(),
            submit_button: button::State::new(),
            fold_button: button::State::new(),
            check_button: button::State::new(),
            raise_bet_button: button::State::new(),
            call_button: button::State::new(),
            answer_input: text_input::State::new(),
            answer_value: String::from(""),
            cached_hand_svg: None,
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let mut col = Column::new().spacing(8).align_items(Align::Center);

        // Asking the question
        col = match &self.review_item.question.presentation {
            item::Presentation::Text(s) => col.push(Text::new(s)),
            item::Presentation::TextHand(s, h) => {
                let (first_card_svg, second_card_svg) = if let Some((f, s)) = self.cached_hand_svg {
                    (f, s)
                } else {
                    self.cached_hand_svg = Some(<(&[u8], &[u8])>::from(*h));
                    self.cached_hand_svg.unwrap()
                };
                col.push(Text::new(s)).push(
                    Row::new()
                        .spacing(8)
                        .push(widget::svg::Svg::new(widget::svg::Handle::from_memory(
                            first_card_svg,
                        )))
                        .push(widget::svg::Svg::new(widget::svg::Handle::from_memory(
                            second_card_svg,
                        ))),
                )
            }
        };

        // Providing answers and showing correct/wrong
        col = match self.state {
            ReviewDisplayState::Waiting => match &self.review_item.question.options {
                item::Options::Binary => col.push(
                    Row::new()
                        .spacing(8)
                        .push(
                            Button::new(&mut self.yes_button, Text::new("Yes"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::Yes,
                                ))
                                .style(styles::Button::Basic),
                        )
                        .push(
                            Button::new(&mut self.no_button, Text::new("No"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::No,
                                ))
                                .style(styles::Button::Basic),
                        ),
                ),
                item::Options::Numbers => col.push(
                    Row::new()
                        .spacing(8)
                        .push(
                            TextInput::new(&mut self.answer_input, "", &self.answer_value, |s| {
                                Message::ReviewDisplayMessage(ReviewDisplayMessage::AnswerChanged(
                                    s,
                                ))
                            })
                            .size(32)
                            .width(Length::Units(80)),
                        )
                        .push(
                            Button::new(&mut self.submit_button, Text::new("Submit"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::Text(item::DisplayString(
                                        self.answer_value.to_string(),
                                    )),
                                ))
                                .style(styles::Button::Basic),
                        ),
                ),
                item::Options::PokerAction => col.push(
                    Row::new()
                        .spacing(8)
                        .push(
                            Button::new(&mut self.fold_button, Text::new("Fold"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::PokerAction(item::PokerAction::Fold),
                                ))
                                .style(styles::Button::Basic),
                        )
                        .push(
                            Button::new(&mut self.check_button, Text::new("Check"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::PokerAction(item::PokerAction::Check),
                                ))
                                .style(styles::Button::Basic),
                        )
                        .push(
                            Button::new(&mut self.call_button, Text::new("Call"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::PokerAction(item::PokerAction::Call),
                                ))
                                .style(styles::Button::Basic),
                        )
                        .push(TextInput::new(
                            &mut self.answer_input,
                            "",
                            &self.answer_value,
                            |s| {
                                Message::ReviewDisplayMessage(ReviewDisplayMessage::AnswerChanged(
                                    s,
                                ))
                            },
                        ))
                        .push(
                            Button::new(&mut self.raise_bet_button, Text::new("Raise/Bet"))
                                .on_press(Message::AnswerReviewItem(
                                    (
                                        self.review_item.question.presentation.clone(),
                                        self.review_item.question.answer.clone(),
                                    ),
                                    item::Answer::PokerAction(item::PokerAction::Raise(
                                        self.answer_value.to_string(),
                                    )),
                                ))
                                .style(styles::Button::Basic),
                        ),
                ),
            },
            ReviewDisplayState::Correct => col.push(
                Button::new(
                    &mut self.next_button,
                    Text::new(format!(
                        "Correct: {}",
                        self.review_item.question.answer.reveal()
                    )),
                )
                .on_press(Message::RequestNewQuestion),
            ),
            ReviewDisplayState::Wrong => col.push(
                Button::new(
                    &mut self.next_button,
                    Text::new(format!(
                        "Wrong: {}",
                        self.review_item.question.answer.reveal()
                    )),
                )
                .on_press(Message::RequestNewQuestion),
            ),
        };

        let info_bar = Row::new()
            .spacing(24)
            .align_items(Align::Center)
            .push(
                Text::new(format!("Difficulty: {}", self.review_item.difficulty))
                    .color([0.75, 0.75, 0.75]),
            )
            .push(
                Text::new(format!(
                    "Days between review attempts: {}",
                    self.review_item.days_between_review_attempts
                ))
                .color([0.75, 0.75, 0.75]),
            )
            .push(
                Text::new(format!(
                    "Date last reviewed: {}",
                    match self.review_item.date_last_reviewed {
                        Some(date_time) => date_time.to_string(),
                        None => String::from("Never"),
                    }
                ))
                .color([0.75, 0.75, 0.75]),
            );

        Container::new(
            Column::new()
                .align_items(Align::Center)
                .push(
                    Container::new(col)
                        .center_x()
                        .center_y()
                        .height(Length::FillPortion(9))
                        .width(Length::Fill),
                )
                .push(
                    Container::new(info_bar)
                        .center_x()
                        .height(Length::FillPortion(1)),
                ),
        )
        .padding(24)
        .center_x()
    }

    pub fn update(&mut self, message: ReviewDisplayMessage) {
        match message {
            ReviewDisplayMessage::AnswerChanged(s) => {
                self.answer_value = s;
            }
            ReviewDisplayMessage::AnswerSubmitted(correct) => {
                if correct {
                    self.state = ReviewDisplayState::Correct;
                } else {
                    self.state = ReviewDisplayState::Wrong;
                }
            }
        }
    }
}
