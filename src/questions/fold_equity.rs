use crate::messages::Message;
use crate::questions::{Question, QuestionAnswer, QuestionState};
use crate::study_screen::StudyMessage;
use crate::styles;
use iced::{button, text_input, Align, Button, Column, Container, Length, Row, Text, TextInput};

#[derive(Debug, Clone)]
pub enum FoldEquityMessage {
    Submit,
    TextChanged(String),
}

use rand::Rng;

pub struct FoldEquity {
    state: QuestionState,
    ans_input: text_input::State,
    submit_button: button::State,
    next_button: button::State,
    user_ans: String,
    risk: f32,   // in BB
    reward: f32, // in  BB
    ans: u32,    // percentage
}

impl FoldEquity {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let risk = (rng.gen_range::<u32, u32, u32>(2, 40) as f32) / 2.0;
        let reward = (rng.gen_range::<u32, u32, u32>(2, 40) as f32) / 2.0;
        let ans = (risk / (risk + reward) * 100.0).round() as u32;
        Self {
            state: QuestionState::Waiting,
            ans_input: text_input::State::new(),
            submit_button: button::State::new(),
            next_button: button::State::new(),
            user_ans: String::new(),
            risk: risk,
            reward: reward,
            ans: ans,
        }
    }
}

impl Question for FoldEquity {
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
                        .push(Text::new(format!(
                            "Hero risk {} BB to win {} BB.\nWhat is the required fold equity?",
                            self.risk, self.reward
                        )))
                        .push(
                            TextInput::new(&mut self.ans_input, "RFE", &self.user_ans, |s| {
                                Message::StudyScreen(StudyMessage::QuestionAnswer(
                                    QuestionAnswer::FoldEquity(FoldEquityMessage::TextChanged(s)),
                                ))
                            })
                            .width(Length::Units(80))
                            .size(32),
                        )
                        .push(match self.state {
                            QuestionState::Waiting => {
                                Button::new(&mut self.submit_button, Text::new("Submit"))
                                    .on_press(Message::StudyScreen(StudyMessage::QuestionAnswer(
                                        QuestionAnswer::FoldEquity(FoldEquityMessage::Submit),
                                    )))
                                    .style(styles::Button::Basic)
                            }
                            QuestionState::Correct => Button::new(
                                &mut self.next_button,
                                Text::new(format!("Correct: {} BB", self.ans)),
                            )
                            .on_press(Message::RequestNewQuestion)
                            .style(styles::Button::Basic),
                            QuestionState::Wrong => Button::new(
                                &mut self.next_button,
                                Text::new(format!("Wrong: {} BB", self.ans)),
                            )
                            .on_press(Message::RequestNewQuestion)
                            .style(styles::Button::Basic),
                        }),
                )
                .center_x()
                .center_y(),
            )
    }

    fn update(&mut self, message: QuestionAnswer) {
        match message {
            QuestionAnswer::FoldEquity(FoldEquityMessage::TextChanged(s)) => self.user_ans = s,
            QuestionAnswer::FoldEquity(FoldEquityMessage::Submit) => {
                if let Ok(user_ans) = u32::from_str_radix(&self.user_ans, 10) {
                    if (self.ans as i32 - user_ans as i32).abs() <= 9 {
                        self.state = QuestionState::Correct;
                    } else {
                        self.state = QuestionState::Wrong;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
