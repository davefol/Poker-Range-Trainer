use std::default::Default;

use crate::card::Card;
use crate::hand::Hand;
use crate::hand::Suit;
use crate::messages::{Message, RangesMessage};
use crate::range::Range;
use crate::styles;

use iced::{
    button, scrollable, text_input, Button, Column, Container, Row, Scrollable, Text, TextInput,
};
use iced_native::{
    input, layout, Align, Background, Clipboard, Color, Element, Event, Font, Hasher,
    HorizontalAlignment, Layout, Length, MouseCursor, Point, Rectangle, Size, VerticalAlignment,
    Widget,
};
use iced_wgpu::{Defaults, Primitive, Renderer};

pub struct SelectRangeButton {
    range_name: String,
    range_id: usize,
    button_state: button::State,
}

impl SelectRangeButton {
    fn new(range_id: usize, range_name: String) -> Self {
        Self {
            range_name,
            range_id,
            button_state: button::State::new(),
        }
    }

    fn view(&mut self) -> Button<Message> {
        Button::new(
            &mut self.button_state,
            Text::new(self.range_name.to_string()).horizontal_alignment(HorizontalAlignment::Left),
        )
        .on_press(Message::RangeSelected(self.range_id))
        .width(Length::Fill)
        .style(styles::Button::RangeList { selected: false })
    }
}

#[derive(Debug, Clone)]
pub struct ActiveRange {
    pub id: usize,
    pub range: Range,
    dirty: bool,
}

impl From<ActiveRange> for Range {
    fn from(active_range: ActiveRange) -> Range {
        active_range.range.clone()
    }
}

impl ActiveRange {
    pub fn new(id: usize, range: Range) -> Self {
        Self {
            id,
            range,
            dirty: false,
        }
    }
}

#[derive(Default)]
pub struct RangesScreen {
    pub active_range: Option<ActiveRange>,
    pub current_range_name_state: text_input::State,
    pub save_current_range_button: button::State,
    pub copy_current_range_button: button::State,
    pub delete_current_range_button: button::State,
    pub ranges_scrollable: scrollable::State,
    pub select_range_buttons: Vec<SelectRangeButton>,
    pub new_range_button: button::State,
}

impl RangesScreen {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_range(&mut self, id: usize, range: Range) {
        self.active_range = Some(ActiveRange::new(id, range));
    }

    pub fn view(&mut self) -> Row<Message> {
        let range_matrix = if let Some(active_range) = &mut self.active_range {
            Card::iterator().rev().enumerate().fold(
                Column::new().spacing(4),
                |column, (row_idx, row_card)| {
                    column.push(Card::iterator().rev().enumerate().fold(
                        Row::new().height(Length::Fill).spacing(4),
                        |row, (col_idx, col_card)| {
                            let suited = {
                                if col_idx > row_idx {
                                    Suit::Suited
                                } else {
                                    Suit::Off
                                }
                            };
                            let first = Card::max(*row_card, *col_card);
                            let second = Card::min(*row_card, *col_card);
                            let hand = Hand {
                                first,
                                second,
                                suited,
                            };
                            row.push(HandToggle::<Message>::new(
                                active_range.range.contains(&hand),
                                hand,
                                |h| Message::RangesScreen(RangesMessage::ToggleHand(h)),
                            ))
                        },
                    ))
                },
            )
        } else {
            Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(
                    Text::new("No range selected.")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center)
                        .height(Length::Fill)
                        .height(Length::Fill),
                )
        };

        let range_list = Container::new(
            self.select_range_buttons.iter_mut().fold(
                Scrollable::new(&mut self.ranges_scrollable)
                    .spacing(8),
                |s, b| s.push(b.view()),
            ),
        )
        .style(styles::Container::Basic)
        .height(Length::FillPortion(9))
        .padding(24);

        let range_controls = if let Some(active_range) = &self.active_range {
            Row::new()
                .spacing(8)
                .push(TextInput::new(
                    &mut self.current_range_name_state,
                    "Range name",
                    &active_range.range.name,
                    |s| Message::RangesScreen(RangesMessage::RangeNameChanged(s)),
                ))
                .push(Text::new(if active_range.dirty { "*" } else { "" }))
                .push(
                    Button::new(&mut self.save_current_range_button, Text::new("Save"))
                        .on_press(Message::SaveRange(Some(active_range.clone())))
                        .style(styles::Button::Basic),
                )
                .push(
                    Button::new(&mut self.copy_current_range_button, Text::new("Copy"))
                        .on_press(Message::CopyRange(Some(active_range.clone())))
                        .style(styles::Button::Basic),
                )
                .push(
                    Button::new(&mut self.delete_current_range_button, Text::new("Delete"))
                        .on_press(Message::DeleteRange(Some(active_range.clone())))
                        .style(styles::Button::Basic),
                )
        } else {
            Row::new()
        };

        let new_range_button = Button::new(
            &mut self.new_range_button,
            Text::new("New Range")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .height(Length::FillPortion(1))
        .on_press(Message::CreateNewRange)
        .style(styles::Button::Basic);

        Row::new() // master containewr
            .spacing(8)
            .push(
                Column::new() // main column
                    .spacing(8)
                    .align_items(Align::Center)
                    .width(Length::FillPortion(2))
                    .push(Row::new()) // range info
                    .push(range_controls)
                    .push(range_matrix),
            )
            .push(
                Column::new() // side bar
                    .width(Length::FillPortion(1))
                    .push(range_list) // list of ranges
                    .push(new_range_button) // new range button
                    .spacing(4),
            )
    }

    pub fn update(&mut self, message: RangesMessage) {
        match message {
            RangesMessage::ToggleHand(hand) => {
                if let Some(active_range) = &mut self.active_range {
                    active_range.range.toggle(hand);
                    active_range.dirty = true;
                }
            }
            RangesMessage::UpdateSelectRangeButtons(ranges) => {
                self.select_range_buttons = ranges
                    .iter()
                    .enumerate()
                    .map(|(i, r)| SelectRangeButton::new(i, r.name.to_string()))
                    .collect()
            }
            RangesMessage::RangeNameChanged(new_name) => {
                if let Some(active_range) = &mut self.active_range {
                    active_range.range.name = new_name;
                }
            }
            RangesMessage::RangesHaveBeenSaved => {
                if let Some(active_range) = &mut self.active_range {
                    active_range.dirty = false;
                }
            }
        }
    }
}

struct HandToggle<Message> {
    is_active: bool,
    on_toggle: Box<dyn Fn(Hand) -> Message>,
    hand: Hand,
}

impl<Message> HandToggle<Message> {
    pub fn new<F>(is_active: bool, hand: Hand, on_toggle: F) -> Self
    where
        F: 'static + Fn(Hand) -> Message,
    {
        Self {
            is_active,
            on_toggle: Box::new(on_toggle),
            hand,
        }
    }

    fn color(&self) -> Background {
        match self.is_active {
            true => Background::Color(Color::from_rgba8(136, 208, 247, 1.0)),
            false => Background::Color(Color::WHITE),
        }
    }
}

impl<Message> Widget<Message, Renderer> for HandToggle<Message> {
    fn width(&self) -> Length {
        Length::Fill
    }
    fn height(&self) -> Length {
        Length::Fill
    }
    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let size = limits
            .height(Length::Fill)
            .width(Length::Fill)
            .resolve(Size::ZERO);
        layout::Node::new(size)
    }
    fn draw(
        &self,
        _renderer: &mut Renderer,
        defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        let background = Primitive::Quad {
            bounds: layout.bounds(),
            background: self.color(),
            border_radius: 5,
            border_color: Color::BLACK,
            border_width: 1,
        };
        let hand_text = Primitive::Text {
            content: self.hand.to_string(),
            bounds: Rectangle {
                x: layout.bounds().center_x(),
                y: layout.bounds().center_y(),
                ..layout.bounds()
            },
            color: Color {
                a: defaults.text.color.a * 0.7,
                ..defaults.text.color
            },
            font: Font::Default,
            size: 20.0,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        };
        (
            Primitive::Group {
                primitives: vec![background, hand_text],
            },
            MouseCursor::Pointer,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        self.is_active.hash(state)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        match event {
            Event::Mouse(input::mouse::Event::Input {
                button: input::mouse::Button::Left,
                state: input::ButtonState::Pressed,
            }) => {
                let mouse_over = layout.bounds().contains(cursor_position);
                if mouse_over {
                    messages.push((self.on_toggle)(self.hand));
                }
            }
            _ => {}
        }
    }
}

impl<'a, Message: 'a> From<HandToggle<Message>> for Element<'a, Message, Renderer> {
    fn from(hand_toggle: HandToggle<Message>) -> Element<'a, Message, Renderer> {
        Element::new(hand_toggle)
    }
}
