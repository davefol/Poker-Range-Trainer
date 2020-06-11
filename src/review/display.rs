use super::item;
use crate::messages::Message;
use iced::{widget, Button, Column, Container, Row, Text, TextInput};
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
    yes_button: button::State,
    no_button: button::State,
    submit_button: button::State,
    fold_button: button::State,
    check_button: button::State,
    raise_bet_button: button::State,
    call_button: button::State,
    answer_input: text_input::State,
    answer_value: String,
}

impl ReviewDisplay {
    pub fn new(review_item: item::ReviewItem) -> Self {
        Self {
            review_item,
            state: ReviewDisplayState::Waiting,
            yes_button: button::State::new(),
            no_button: button::State::new(),
            submit_button: button::State::new(),
            fold_button: button::State::new(),
            check_button: button::State::new(),
            raise_bet_button: button::State::new(),
            call_button: button::State::new(),
            answer_input: text_input::State::new(),
            answer_value: String::from(""),
        }
    }

    pub fn view(&mut self) -> Container<Message> {
        let mut col = Column::new();

        // Asking the question
        col = match &self.review_item.question.presentation {
            item::Presentation::Text(s) => col.push(Text::new(s)),
            item::Presentation::TextHand(s, h) => {
                let (first_card_svg, second_card_svg) = <(&[u8], &[u8])>::from(*h);
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

        // Providing answers
        col = match &self.review_item.question.options {
            item::Options::Binary => col.push(
                Row::new()
                    .push(
                        Button::new(&mut self.yes_button, Text::new("Yes")).on_press(
                            Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                                item::Answer::Yes,
                            ),
                        ),
                    )
                    .push(Button::new(&mut self.no_button, Text::new("No")).on_press(
                        Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                            item::Answer::No,
                        ),
                    )),
            ),
            item::Options::Numbers => col
                .push(TextInput::new(
                    &mut self.answer_input,
                    "",
                    &self.answer_value,
                    |s| Message::ReviewDisplayMessage(ReviewDisplayMessage::AnswerChanged(s)),
                ))
                .push(
                    Button::new(&mut self.submit_button, Text::new("Submit")).on_press(
                        Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                            item::Answer::Text(item::DisplayString(self.answer_value.to_string())),
                        ),
                    ),
                ),
            item::Options::PokerAction => col.push(
                Row::new()
                    .push(
                        Button::new(&mut self.fold_button, Text::new("Fold")).on_press(
                            Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                                item::Answer::PokerAction(item::PokerAction::Fold),
                            ),
                        ),
                    )
                    .push(
                        Button::new(&mut self.check_button, Text::new("Check")).on_press(
                            Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                                item::Answer::PokerAction(item::PokerAction::Check),
                            ),
                        ),
                    )
                    .push(
                        Button::new(&mut self.call_button, Text::new("Call")).on_press(
                            Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                                item::Answer::PokerAction(item::PokerAction::Call),
                            ),
                        ),
                    )
                    .push(TextInput::new(
                        &mut self.answer_input,
                        "",
                        &self.answer_value,
                        |s| Message::ReviewDisplayMessage(ReviewDisplayMessage::AnswerChanged(s)),
                    ))
                    .push(
                        Button::new(&mut self.raise_bet_button, Text::new("Raise/Bet")).on_press(
                            Message::AnswerReviewItem(
                                (
                                    self.review_item.question.presentation.clone(),
                                    self.review_item.question.answer.clone(),
                                ),
                                item::Answer::PokerAction(item::PokerAction::Raise(
                                    self.answer_value.to_string(),
                                )),
                            ),
                        ),
                    ),
            ),
        };

        Container::new(col)
    }

    pub fn update(&mut self, message: ReviewDisplayMessage) {
        match message {
            ReviewDisplayMessage::AnswerChanged(s) => {
                self.answer_value = s;
            },
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
