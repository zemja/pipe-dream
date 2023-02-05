use std::default::Default;
use iced::{application, Background, Color};
use iced::widget::{container, scrollable, text_input};
use iced::widget::scrollable::{Scrollbar, Scroller};

mod colour {
    use iced::Color;
    use once_cell::sync::Lazy;

    pub static ERROR: Lazy<Color> = Lazy::new(|| Color::from([0.5, 0.0, 0.0]));
    pub static SHADOW: Lazy<Color> = Lazy::new(|| Color::from([0.7, 0.7, 0.7]));
    pub static EMPHASIS: Lazy<Color> = Lazy::new(|| Color::from([0.7, 0.5, 0.7]));
}

#[derive(Clone, Copy, Debug)]
pub enum Style {
    Default,
    Error,
    Shadow,
    Emphasis,
}

impl Default for Style {
    fn default() -> Style {
        Style::Default
    }
}

#[derive(Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::WHITE,
            text_color: Color::BLACK,
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = Style;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = text_input::Appearance {
            background: Background::Color(Color::from([0.95, 0.95, 0.95])),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from([0.0, 0.0, 0.0]),
        };

        match style {
            Style::Default => { },
            Style::Error => {
                appearance.border_color = *colour::ERROR;
                appearance.border_width = 1.0;
            },
            Style::Shadow => {
                appearance.background = Background::Color(Color::from([0.3, 0.3, 0.3]));
            }
            Style::Emphasis => {
                appearance.background = Background::Color(Color::from([0.85, 0.85, 0.95]));
            }
        }

        appearance
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let mut appearance = text_input::Appearance {
            background: Background::Color(Color::from([1.0, 1.0, 1.0])),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::from([0.0, 0.0, 0.0]),
        };

        match style {
            Style::Default => { },
            Style::Error => {
                appearance.border_color = *colour::ERROR;
                appearance.border_width = 3.0;
            },
            Style::Shadow => {
                appearance.background = Background::Color(Color::from([0.3, 0.3, 0.3]));
            }
            Style::Emphasis => {
                appearance.border_color = *colour::EMPHASIS;
            }
        }

        appearance
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        *colour::SHADOW
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match style {
            Style::Default => Color::BLACK,
            Style::Error => *colour::ERROR,
            Style::Shadow => *colour::SHADOW,
            Style::Emphasis => *colour::EMPHASIS,
        }
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from([0.9, 0.9, 1.0])
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}

impl iced::widget::text::StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
        match style {
            Style::Default => iced::widget::text::Appearance { color: None },
            Style::Error => iced::widget::text::Appearance { color: Some(*colour::ERROR) },
            Style::Shadow => iced::widget::text::Appearance { color: Some(*colour::SHADOW) },
            Style::Emphasis => iced::widget::text::Appearance { color: Some(*colour::EMPHASIS) },
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default()
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = Style;

    fn active(&self, _style: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: Scroller {
                color: Color::BLACK,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }
}