use crate::hand::Hand;
use crate::range::Range;
use crate::ranges_screen::ActiveRange;
use crate::review;

#[derive(Debug, Clone,)]
pub enum Message {
    SaveRanges,
    ViewStudyScreen,
    ViewRangesScreen,
    RangesScreen(RangesMessage),
    CreateNewRange,
    RangeSelected(usize),
    SaveRange(Option<ActiveRange>),
    CopyRange(Option<ActiveRange>),
    DeleteRange(Option<ActiveRange>),
    RequestNewQuestion,
    AnswerReviewItem(crate::range_trainer::ReviewItemsKey, review::item::Answer),
    ReviewDisplayMessage(review::display::ReviewDisplayMessage),
}

#[derive(Debug, Clone)]
pub enum RangesMessage {
    ToggleHand(Hand),
    RangeNameChanged(String),
    UpdateSelectRangeButtons(Vec<Range>),
    RangesHaveBeenSaved,
}

