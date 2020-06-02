use iced::{button, Color, Background, container};

const DARK_GREY: Color = Color{r: 0.25, g: 0.25, b: 0.25, a: 1.0};
const LIGHT_GREY: Color = Color{r: 0.75, g: 0.75, b: 0.75, a: 1.0};

pub enum Button {
    Toolbar,
    RangeList { selected: bool },
    NewRange,
    Basic,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Toolbar => button::Style {
                border_color: Color::TRANSPARENT,
                text_color: DARK_GREY,
                background: None,
                ..button::Style::default()
            },
            Button::RangeList { selected } => button::Style {
                border_color: DARK_GREY,
                border_width: 1,
                border_radius: 4,
                background: if *selected {Some(Background::Color(LIGHT_GREY))} else {None},
                ..button::Style::default()
            },
            Button::NewRange => button::Style {
                border_color: DARK_GREY,
                border_width: 1,
                border_radius: 0,
                background: None,
                ..button::Style::default()
            },
            Button::Basic => button::Style {
                border_color: DARK_GREY,
                border_width: 1,
                border_radius: 4,
                background: None,
                ..button::Style::default()
            }
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            background: Some(Background::Color(LIGHT_GREY)),
            ..active
        }
    }
}

pub enum Container {
    Basic,
}

impl container::StyleSheet for Container {
    fn style (&self) -> container::Style {
        match self {
            Container::Basic => container::Style {
                border_color: DARK_GREY,
                border_radius: 4,
                border_width: 1,
                background: None,
                ..container::Style::default()
            }
        }
    }
}
