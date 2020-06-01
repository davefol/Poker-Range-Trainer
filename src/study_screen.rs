use rand::Rng;

use crate::messages::{
    Message
};
use crate::range::Range;

use crate::questions::{
    QuestionWrapper, BinaryRangeQuestion, QuestionAnswer, Question, FoldEquity,
};


use iced::{Text, Row};

#[derive(Debug, Clone)]
pub enum StudyMessage {
    QuestionAnswer(QuestionAnswer),
}

pub struct StudyScreen {
    question: Option<QuestionWrapper>,
}

impl StudyScreen {
    pub fn new(ranges: &Vec<Range>) -> Self {
        let question_choice = rand::thread_rng().gen_range(0,2);
        let question = match question_choice {
            0 => Some(QuestionWrapper::BinaryRangeQuestion(BinaryRangeQuestion::new(ranges))),
            1 => Some(QuestionWrapper::FoldEquity(FoldEquity::new())),
            _ => unreachable!()
        };
        Self {
            question: question,
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        match &mut self.question {
            Some(question) => question.view(),
            None => Row::new().push(Text::new("No available questions."))
        }
    }

    pub fn update(&mut self, message: StudyMessage) {
        match message {
            StudyMessage::QuestionAnswer(m) => {
                if let Some(q) = &mut self.question {
                    q.update(m);
                }
            },
        }
    }

    pub fn new_question(&mut self, ranges: &Vec<Range>) {
        let question_choice = rand::thread_rng().gen_range(0,2);
        let question = match question_choice {
            0 => Some(QuestionWrapper::BinaryRangeQuestion(BinaryRangeQuestion::new(ranges))),
            1 => Some(QuestionWrapper::FoldEquity(FoldEquity::new())),
            _ => unreachable!()
        };
        self.question = question; 
    }
}
