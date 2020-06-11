use crate::{fileio, range, ranges_screen, review, study_screen, toolbar};
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::error::Error;

use crate::messages::{Message, RangesMessage};

use crate::ranges_screen::ActiveRange;

use iced::{executor, Application, Column, Command, Container, Element};

use iced_native::Align;

pub type ReviewItemsKey = (review::item::Presentation, review::item::Answer);
type ReviewItems = fileio::ReviewCollection;




#[derive(Debug, Clone)]
struct ScheduleItem {
    pub key: ReviewItemsKey,
    pub percent_overdue: f32,
}

impl PartialOrd for ScheduleItem {
    fn partial_cmp(&self, other: &ScheduleItem) -> Option<Ordering> {
        self.percent_overdue.partial_cmp(&other.percent_overdue)
    }
}

impl Ord for ScheduleItem {
    fn cmp(&self, other: &ScheduleItem) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for ScheduleItem {
    fn eq(&self, other: &ScheduleItem) -> bool {
        self.percent_overdue == other.percent_overdue
    }
}

impl Eq for ScheduleItem {}

enum ScreenType {
    Study,
    Ranges,
}

impl Default for ScreenType {
    fn default() -> Self {
        ScreenType::Ranges
    }
}

pub struct RangeTrainer {
    ranges: Vec<range::Range>,
    review_items: ReviewItems,
    toolbar: toolbar::ToolBar,
    ranges_screen: ranges_screen::RangesScreen,
    current_screen: ScreenType,
    study_screen: study_screen::StudyScreen,
    review_schedule: BinaryHeap<ScheduleItem>,
}

impl RangeTrainer {
    fn rebuild_review_items(&mut self) -> Result<(), Box<dyn Error>> {
        let mut review_items = ReviewItems::new();

        // binary range questions
        for range in self.ranges.iter() {
            for hand in range.hands.iter() {
                let key = (
                    review::item::Presentation::TextHand(review::item::DisplayString::new(&range.name), *hand),
                    review::item::Answer::Yes,
                );
                match self.review_items.get(&key) {
                    Some(review_item) => {
                        review_items.insert(key.clone(), review_item.clone());
                    }
                    None => {
                        review_items.insert(
                            key.clone(),
                            review::item::ReviewItem {
                                difficulty: 0.3,
                                days_between_review_attempts: 3.0,
                                date_last_reviewed: None,
                                last_answer_correct: false,
                                question: review::item::Question {
                                    presentation: key.0,
                                    options: review::item::Options::Binary,
                                    answer: key.1,
                                    tolerance: None,
                                },
                            },
                        );
                    }
                }
            }
        }

        // required fold equity questions
        for bet in 2..=200 {
            for pot in 3..=200 {
                let key = (
                    review::item::Presentation::Text(
                        review::item::DisplayString(format!(
                            "RFE for {}BB bet to win {}BB pot",
                            bet as f32 / 2.0,
                            pot as f32 / 2.0
                        )
                        .to_string()),
                    ),
                    review::item::Answer::Text(review::item::DisplayString((bet / (bet + pot) * 100).to_string())),
                );
                match self.review_items.get(&key) {
                    Some(review_item) => {
                        review_items.insert(key.clone(), review_item.clone());
                    }
                    None => {
                        review_items.insert(
                            key.clone(),
                            review::item::ReviewItem {
                                difficulty: 0.3,
                                days_between_review_attempts: 3.0,
                                date_last_reviewed: None,
                                last_answer_correct: false,
                                question: review::item::Question {
                                    presentation: key.0,
                                    options: review::item::Options::Numbers,
                                    answer: key.1,
                                    tolerance: Some(9.6),
                                },
                            },
                        );
                    }
                }
            }
        }

        let mut review_schedule: BinaryHeap<ScheduleItem> = BinaryHeap::<ScheduleItem>::new();
        for (key, review_item) in review_items.iter() {
            review_schedule.push(ScheduleItem {
                key: key.clone(),
                percent_overdue: review_item.percent_overdue(),
            });
        }

        let study_screen = match review_schedule.pop() {
            Some(schedule_item) => {
                study_screen::StudyScreen::new(Some(review::display::ReviewDisplay::new(
                    review_items.get(&schedule_item.key).unwrap().clone(),
                )))
            }
            None => study_screen::StudyScreen::new(None),
        };

        self.study_screen = study_screen;
        self.review_schedule = review_schedule;

        self.review_items = review_items;
        fileio::save_review_items(&self.review_items)
    }
}

impl Application for RangeTrainer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let ranges = fileio::load_ranges();
        let review_items = fileio::load_review_items();
        let mut review_schedule: BinaryHeap<ScheduleItem> = BinaryHeap::<ScheduleItem>::new();

        // schedule review items on startup
        for (key, review_item) in review_items.iter() {
            review_schedule.push(ScheduleItem {
                key: key.clone(),
                percent_overdue: review_item.percent_overdue(),
            });
        }

        // select first review item for study
        let study_screen = match review_schedule.pop() {
            Some(schedule_item) => {
                study_screen::StudyScreen::new(Some(review::display::ReviewDisplay::new(
                    review_items.get(&schedule_item.key).unwrap().clone(),
                )))
            }
            None => study_screen::StudyScreen::new(None),
        };

        let mut ranges_screen = ranges_screen::RangesScreen::new();
        ranges_screen.update(RangesMessage::UpdateSelectRangeButtons(ranges.to_vec()));

        (
            Self {
                ranges: ranges,
                review_items,
                review_schedule,
                ranges_screen: ranges_screen,
                study_screen: study_screen,
                toolbar: toolbar::ToolBar::default(),
                current_screen: ScreenType::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Range Trainer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::SaveRanges => {
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                self.ranges_screen
                    .update(RangesMessage::RangesHaveBeenSaved);
            }
            Message::ViewStudyScreen => {
                self.current_screen = ScreenType::Study;
            }
            Message::ViewRangesScreen => {
                self.current_screen = ScreenType::Ranges;
            }
            Message::CreateNewRange => {
                let new_range = range::Range::new();
                self.ranges_screen
                    .set_range(self.ranges.len(), new_range.clone());
                self.ranges.push(new_range);
                self.ranges_screen
                    .update(RangesMessage::UpdateSelectRangeButtons(
                        self.ranges.to_vec(),
                    ));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                match self.rebuild_review_items() {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
            Message::RangeSelected(range_id) => match self.ranges.get(range_id) {
                Some(range) => {
                    self.ranges_screen.active_range =
                        Some(ActiveRange::new(range_id, range.clone()));
                }
                None => {}
            },
            Message::RangesScreen(ranges_message) => self.ranges_screen.update(ranges_message),
            Message::SaveRange(Some(active_range)) => {
                if let Some(r) = self.ranges.get_mut(active_range.id) {
                    *r = active_range.range.clone();
                    self.ranges_screen
                        .update(RangesMessage::UpdateSelectRangeButtons(
                            self.ranges.to_vec(),
                        ));
                    match fileio::save_ranges(&self.ranges) {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                    match self.rebuild_review_items() {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                    self.ranges_screen
                        .update(RangesMessage::RangesHaveBeenSaved);
                }
            }
            Message::SaveRange(None) => {}
            Message::DeleteRange(Some(active_range)) => {
                self.ranges.remove(active_range.id);

                if let Some(last_range) = self.ranges.last() {
                    self.ranges_screen.active_range =
                        Some(ActiveRange::new(self.ranges.len() - 1, last_range.clone()));
                } else {
                    self.ranges_screen.active_range = None;
                }

                self.ranges_screen
                    .update(RangesMessage::UpdateSelectRangeButtons(
                        self.ranges.to_vec(),
                    ));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                match self.rebuild_review_items() {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                self.ranges_screen
                    .update(RangesMessage::RangesHaveBeenSaved);
            }
            Message::DeleteRange(None) => {}
            Message::CopyRange(Some(active_range)) => {
                let mut copied_range = range::Range::from(active_range);
                copied_range.name.push_str(" Copy");
                self.ranges_screen.active_range =
                    Some(ActiveRange::new(self.ranges.len(), copied_range.clone()));
                self.ranges.push(copied_range);
                self.ranges_screen
                    .update(RangesMessage::UpdateSelectRangeButtons(
                        self.ranges.to_vec(),
                    ));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                match self.rebuild_review_items() {
                    Ok(_) => {}
                    Err(e) => {
                        dbg!(e);
                    }
                }
                self.ranges_screen
                    .update(RangesMessage::RangesHaveBeenSaved);
            }
            Message::CopyRange(None) => {}
            Message::AnswerReviewItem(key, ans) => {
                let review_item = self.review_items.get_mut(&key).unwrap();
                let correct = match ans {
                    review::item::Answer::Yes => ans == review_item.question.answer,
                    review::item::Answer::No => ans == review_item.question.answer,
                    review::item::Answer::Text(s) => {
                        if let Ok(num) = s.parse::<f32>() {
                            if let review::item::Answer::Text(x) =
                                review_item.question.answer.clone()
                            {
                                if let Ok(ans_num) = x.parse::<f32>() {
                                    (num - ans_num).abs() < review_item.question.tolerance.unwrap()
                                } else {
                                    return Command::none();
                                }
                            } else {
                                return Command::none();
                            }
                        } else {
                            return Command::none();
                        }
                    }
                    review::item::Answer::PokerAction(review::item::PokerAction::Fold) => {
                        ans == review_item.question.answer
                    }
                    review::item::Answer::PokerAction(review::item::PokerAction::Check) => {
                        ans == review_item.question.answer
                    }
                    review::item::Answer::PokerAction(review::item::PokerAction::Call) => {
                        ans == review_item.question.answer
                    }
                    review::item::Answer::PokerAction(review::item::PokerAction::Raise(s)) => {
                        if let Ok(num) = s.parse::<f32>() {
                            if let review::item::Answer::PokerAction(
                                review::item::PokerAction::Raise(x),
                            ) = review_item.question.answer.clone()
                            {
                                if let Ok(ans_num) = x.parse::<f32>() {
                                    (num - ans_num).abs() < review_item.question.tolerance.unwrap()
                                } else {
                                    return Command::none();
                                }
                            } else {
                                return Command::none();
                            }
                        } else {
                            return Command::none();
                        }
                    }
                };
                review_item.update(correct);
                self.review_schedule.push(ScheduleItem {
                    key: (review_item.question.presentation.clone(), review_item.question.answer.clone()),
                    percent_overdue: review_item.percent_overdue(),
                });
                self.study_screen.update(review::display::ReviewDisplayMessage::AnswerSubmitted(correct));
                
            }
            Message::ReviewDisplayMessage(_message) => {
                self.study_screen.update(_message);
            }
            Message::RequestNewQuestion => {
                self.study_screen = match self.review_schedule.pop() {
                    Some(schedule_item) => {
                        study_screen::StudyScreen::new(Some(review::display::ReviewDisplay::new(
                            self.review_items.get(&schedule_item.key).unwrap().clone(),
                        )))
                    }
                    None => study_screen::StudyScreen::new(None),
                };
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(8)
            .spacing(8)
            .align_items(Align::Center)
            .push(self.toolbar.view())
            .push(match &self.current_screen {
                ScreenType::Study => self.study_screen.view(),
                ScreenType::Ranges => Container::new(self.ranges_screen.view()),
            })
            .into()
    }
}
