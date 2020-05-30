use crate::hand::Hand;
use crate::range::Range;
use crate::ranges_screen::ActiveRange;

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
    StudyScreen(StudyMessage),
    RequestNewQuestion,
}

#[derive(Debug, Clone)]
pub enum RangesMessage {
    ToggleHand(Hand),
    RangeNameChanged(String),
    UpdateSelectRangeButtons(Vec<Range>),
    RangesHaveBeenSaved,
}

#[derive(Debug, Clone)]
pub enum StudyMessage {
    Answer(bool),
    NewQuestion(Vec<Range>),
}
