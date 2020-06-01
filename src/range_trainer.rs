use crate::{
    range,
    fileio,
    toolbar,
    study_screen,
    ranges_screen
};

use crate::messages::{
    Message, RangesMessage,
};

use crate::ranges_screen::ActiveRange;

use iced::{
    Application,
    executor, 
    Command,
    Element,
    Column
};

use iced_native::{
    Align
};

enum ScreenType {
    Study,
    Ranges
}

impl Default for ScreenType {
    fn default() -> Self { ScreenType::Ranges }
}

pub struct RangeTrainer {
    ranges: Vec<range::Range>,
    toolbar: toolbar::ToolBar,
    ranges_screen: ranges_screen::RangesScreen,
    current_screen: ScreenType,
    study_screen: study_screen::StudyScreen,
}

impl Application for RangeTrainer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let ranges = fileio::load_ranges();
        let mut ranges_screen = ranges_screen::RangesScreen::new();
        ranges_screen.update(
            RangesMessage::UpdateSelectRangeButtons(ranges.to_vec())
            );

        let study_screen = study_screen::StudyScreen::new(&ranges);
        (Self {
            ranges: ranges,
            ranges_screen: ranges_screen,
            study_screen: study_screen,
            toolbar: toolbar::ToolBar::default(),
            current_screen: ScreenType::default(),
        }, 
        Command::none())
    }

    fn title(&self) -> String {
        String::from("Range Trainer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Message::SaveRanges => { 
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {},
                    Err(e) => {dbg!(e);}
                }
                self.ranges_screen.update(RangesMessage::RangesHaveBeenSaved);
            },
            Message::ViewStudyScreen => { self.current_screen = ScreenType::Study; }
            Message::ViewRangesScreen => { self.current_screen = ScreenType::Ranges; },
            Message::CreateNewRange => { 
                let new_range = range::Range::new();
                self.ranges_screen.set_range(self.ranges.len(), new_range.clone());
                self.ranges.push(new_range); 
                self.ranges_screen.update(RangesMessage::UpdateSelectRangeButtons(self.ranges.to_vec()));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {},
                    Err(e) => {dbg!(e);}
                }
            },
            Message::RangeSelected(range_id) => {
                match self.ranges.get(range_id) {
                    Some(range) => {
                        self.ranges_screen.active_range = Some(ActiveRange::new(range_id, range.clone()));
                    },
                    None => {}
                }
            }
            Message::RangesScreen(ranges_message) => { self.ranges_screen.update(ranges_message) },
            Message::StudyScreen(study_message) => { self.study_screen.update(study_message) }, 
            Message::RequestNewQuestion => { self.study_screen.new_question(&self.ranges)},
            Message::SaveRange(Some(active_range)) => {
                if let Some(r) = self.ranges.get_mut(active_range.id) {
                    *r = active_range.range.clone();
                    self.ranges_screen.update(RangesMessage::UpdateSelectRangeButtons(self.ranges.to_vec()));
                    match fileio::save_ranges(&self.ranges) {
                        Ok(_) => {},
                        Err(e) => {dbg!(e);}
                    }
                    self.ranges_screen.update(RangesMessage::RangesHaveBeenSaved);
                    self.study_screen = study_screen::StudyScreen::new(&self.ranges);
                }
            },
            Message::SaveRange(None) => {},
            Message::DeleteRange(Some(active_range)) => {
                self.ranges.remove(active_range.id);

                if let Some(last_range) = self.ranges.last() {
                    self.ranges_screen.active_range = Some(ActiveRange::new(self.ranges.len() - 1, last_range.clone()));
                } else {
                    self.ranges_screen.active_range = None;
                }

                self.ranges_screen.update(RangesMessage::UpdateSelectRangeButtons(self.ranges.to_vec()));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {},
                    Err(e) => {dbg!(e);}
                }
                self.ranges_screen.update(RangesMessage::RangesHaveBeenSaved);
            },
            Message::DeleteRange(None) => {},
            Message::CopyRange(Some(active_range)) => {
                let mut copied_range = range::Range::from(active_range);
                copied_range.name.push_str(" Copy");
                self.ranges_screen.active_range = Some(ActiveRange::new(self.ranges.len(), copied_range.clone()));
                self.ranges.push(copied_range);
                self.ranges_screen.update(RangesMessage::UpdateSelectRangeButtons(self.ranges.to_vec()));
                match fileio::save_ranges(&self.ranges) {
                    Ok(_) => {},
                    Err(e) => {dbg!(e);}
                }
                self.ranges_screen.update(RangesMessage::RangesHaveBeenSaved);
            },
            Message::CopyRange(None) => {}
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
                ScreenType::Ranges => self.ranges_screen.view(),
            })
        .into()
    }
}

