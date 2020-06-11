use std::fmt;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

use crate::hand::Hand;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReviewItem {
    pub difficulty: f32, // [0,1]
    pub days_between_review_attempts: f32,
    pub date_last_reviewed: Option<DateTime<Local>>,
    pub last_answer_correct: bool,
    pub question: Question,
}

impl ReviewItem {
    pub fn percent_overdue(&self) -> f32 {
        if self.last_answer_correct {
            1.0
        } else {
            if let Some(date_last_reviewed) = self.date_last_reviewed {
                f32::min(
                    2.0,
                    (Local::now() - date_last_reviewed).num_days() as f32
                        / self.days_between_review_attempts,
                )
            } else {
                1.0
            }
        }
    }
    
    pub fn update(&mut self, correct: bool) {
        self.date_last_reviewed = Some(Local::now());
        self.last_answer_correct = correct;
        let performace_rating = if self.last_answer_correct { 1.0 } else { 0.0 };
        self.difficulty += self.percent_overdue() * (1.0/17.0) * (8.0 - 9.0 * performace_rating);
        if self.difficulty > 1.0 {self.difficulty = 1.0};
        if self.difficulty < 0.0 {self.difficulty = 0.0};
        let difficulty_weight = 3.0 - 1.7 * self.difficulty;
        if self.last_answer_correct {
            self.days_between_review_attempts = 1.0 + (difficulty_weight - 1.0) * self.percent_overdue();
        } else {
            self.days_between_review_attempts = 1.0 / difficulty_weight.powf(2.0);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Question {
    pub presentation: Presentation,
    pub options: Options,
    pub answer: Answer,
    pub tolerance: Option<f32>,
}

// Display strings are not allowed to have the characters ⨼ or ⦙ 
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct DisplayString(pub String);

impl DisplayString {
    pub fn new(s: &str) -> DisplayString {
        DisplayString(s
            .replace('⨼', "_")
            .replace('⦙', "|")
            .replace('\n', "\\n")
            )
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, <T as std::str::FromStr>::Err> {
        self.0.parse::<T>()
    }
}


impl fmt::Display for DisplayString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<&DisplayString> for String {
    fn from(display_string: &DisplayString) -> String {
        display_string.0.replace("\\n", "\n")
    } 
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Presentation {
    Text(DisplayString),
    TextHand(DisplayString, Hand),
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Presentation::Text(s) => write!(f, "Presentation⨼Text⨼{}", s),
            Presentation::TextHand(s,h) => write!(f, "Presentation⨼TextHand⨼{}⨼{}", s, h),
        }
    }
}

pub struct ParsePresentationError;

impl FromStr for Presentation {
    type Err = ParsePresentationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PRESENTATION_RE: Regex = Regex::new("^Presentation⨼(Text|TextHand)⨼(.*?)$").unwrap();
        }
        if let Some(groups) = PRESENTATION_RE.captures(s) {
            match groups.get(1).map(|x| x.as_str()) {
                Some("Text") => {
                    if let Some(text) = groups.get(2) {
                        Ok(Presentation::Text(DisplayString::new(text.as_str())))
                    } else {
                        Err(ParsePresentationError)
                    }
                },
                Some("TextHand") => {
                    if let Some(capture) = groups.get(2) {
                        let data = capture.as_str().split('⨼').collect::<Vec<&str>>();
                        if let (Some(text), Some(hand)) = (data.get(0), data.get(1)) {
                            if let Ok(hand) = hand.parse::<crate::hand::Hand>() {
                                Ok(Presentation::TextHand(DisplayString::new(text), hand))
                            } else {
                                Err(ParsePresentationError)
                            }
                        } else {
                            Err(ParsePresentationError)
                        }
                    } else {
                        Err(ParsePresentationError)
                    }
                },
                _ => Err(ParsePresentationError)
            }
        } else {
            Err(ParsePresentationError)
        }

    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Options {
    Binary,
    Numbers,
    PokerAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Answer {
    Yes,
    No,
    Text(DisplayString),
    PokerAction(PokerAction),
}

impl Answer {
    pub fn reveal(&self) -> String {
        match self {
            Answer::Yes => String::from("Yes"),
            Answer::No => String::from("No"),
            Answer::Text(s) => s.to_string(),
            Answer::PokerAction(p) => match p {
                PokerAction::Fold => String::from("Fold"),
                PokerAction::Call => String::from("Call"),
                PokerAction::Check => String::from("Check"),
                PokerAction::Raise(r) => String::from(format!("Raise {} BB", r))
            }
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Yes => write!(f, "Answer⨼Yes"),
            Answer::No => write!(f, "Answer⨼No"),
            Answer::Text(s) => write!(f, "Answer⨼Text⨼{}", s),
            Answer::PokerAction(p) => write!(f, "Answer⨼PokerAction⨼{}", p),
        }
    }
}

pub struct ParseAnswerError;
impl FromStr for Answer {
    type Err = ParseAnswerError;
    fn from_str(s: &str) -> Result<Answer, ParseAnswerError> {
        lazy_static! {
            static ref ANSWER_RE: Regex = Regex::new("^Answer⨼(Text|PokerAction)⨼(.*?)$").unwrap();
        }
        match s {
            "Answer⨼Yes" => Ok(Answer::Yes),
            "Answer⨼No" => Ok(Answer::No),
            _ => {
                if let Some(groups) = ANSWER_RE.captures(s) {
                    match groups.get(1).map(|x| x.as_str()) {
                        Some("Text") => {
                            if let Some(text) = groups.get(2) {
                                Ok(Answer::Text(DisplayString::new(text.as_str())))
                            } else {
                                Err(ParseAnswerError)
                            }
                        },
                        Some("PokerAction") => {
                            if let Some(poker_action) = groups.get(2) {
                                if let Ok(poker_action) = poker_action.as_str().parse::<PokerAction>() {
                                    Ok(Answer::PokerAction(poker_action))
                                } else {
                                    Err(ParseAnswerError)
                                }
                            } else {
                                Err(ParseAnswerError)
                            }
                        },
                        _ => Err(ParseAnswerError)
                    }
                } else {
                    Err(ParseAnswerError)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum PokerAction {
    Fold,
    Raise(String),
    Check,
    Call,
}

impl fmt::Display for PokerAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokerAction::Fold => write!(f, "PokerAction⨼Fold"),
            PokerAction::Raise(s) => write!(f, "PokerAction⨼Raise⨼{}", s),
            PokerAction::Check => write!(f, "PokerAction⨼Check"),
            PokerAction::Call => write!(f, "PokerAction⨼Call"),
        }
    }
}

pub struct ParsePokerActionError;
impl FromStr for PokerAction {
    type Err = ParsePokerActionError;
    fn from_str(s: &str) -> Result<PokerAction, Self::Err> {
        lazy_static! {
            static ref POKER_ACTION_RAISE_RE: Regex = Regex::new("^PokerAction⨼Raise⨼(.*?)$").unwrap();
        }
        match s {
            "PokerAction⨼Fold" => Ok(PokerAction::Fold),
            "PokerAction⨼Check" => Ok(PokerAction::Check),
            "PokerAction⨼Call" => Ok(PokerAction::Call),
            _ => {
                if let Some(groups) = POKER_ACTION_RAISE_RE.captures(s) {
                    if let Some(raise_amt) = groups.get(1) {
                        if let Ok(text) = raise_amt.as_str().parse::<f32>().map(|x| x.to_string()) {
                            Ok(PokerAction::Raise(text))
                        } else {
                            Err(ParsePokerActionError)
                        }
                    } else {
                        Err(ParsePokerActionError)
                    }
                } else {
                    Err(ParsePokerActionError)
                }
            }
        }
    }
}
