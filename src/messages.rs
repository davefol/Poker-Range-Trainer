use crate::hand::Hand;
use crate::range::Range;
use crate::ranges_screen::ActiveRange;
use crate::study_screen::StudyMessage;

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

