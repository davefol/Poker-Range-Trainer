use iced::{
    Row,
};
use crate::messages::{Message};

pub use self::binary_range_question::BinaryRangeQuestion;
pub use self::fold_equity::{FoldEquity, FoldEquityMessage};

pub mod binary_range_question;
pub mod fold_equity;

pub trait Question {
    fn view(&mut self) -> Row<Message>;
    fn update(&mut self, message: QuestionAnswer) -> ();
}

pub enum QuestionWrapper {
    BinaryRangeQuestion(BinaryRangeQuestion),
    FoldEquity(FoldEquity)
}

impl Question for QuestionWrapper {
    fn view(&mut self) -> Row<Message> {
        match *self {
            QuestionWrapper::BinaryRangeQuestion(ref mut binary_range_question) => binary_range_question.view(),
            QuestionWrapper::FoldEquity(ref mut fold_equity) => fold_equity.view(),
        }
    }

    fn update(&mut self, message: QuestionAnswer) {
        match *self {
            QuestionWrapper::BinaryRangeQuestion(ref mut binary_range_question) => binary_range_question.update(message),
            QuestionWrapper::FoldEquity(ref mut fold_equity) => fold_equity.update(message),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum QuestionState {
    Waiting,
    Correct,
    Wrong
}

#[derive(Debug, Clone)]
pub enum QuestionAnswer {
    BinaryRangeQuestion(bool),
    FoldEquity(FoldEquityMessage),
}
