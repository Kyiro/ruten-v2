use iced::{widget::*, Background, Color, Vector};
use iced_aw::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    dark_mode: bool,
    background_colour: Option<Color>,
    text_colour: Option<Color>,
    main_colour: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            dark_mode: true,
            background_colour: None,
            text_colour: None,
            main_colour: Color::from_rgb8(0x00, 0x99, 0xff),
        }
    }
}

impl Theme {
    pub fn dark_mode(&self) -> bool {
        self.dark_mode
    }

    pub fn text_colour(&self) -> Color {
        self.text_colour.unwrap_or(match self.dark_mode {
            true => Color::WHITE,
            false => Color::BLACK,
        })
    }

    pub fn background_colour(&self) -> Color {
        self.background_colour.unwrap_or(match self.dark_mode {
            true => Color::from_rgb8(0x20, 0x22, 0x24),
            false => Color::WHITE,
        })
    }

    pub fn main_colour(&self) -> Color {
        self.main_colour
    }

    pub fn minus_bg(&self, colour: Color) -> Color {
        let Color { r, g, b, a } = self.background_colour();

        Color::from_rgba(r - colour.r, g - colour.g, b - colour.b, a)
    }

    pub fn plus_bg(&self, colour: Color) -> Color {
        let Color { r, g, b, a } = self.background_colour();

        Color::from_rgba(r + colour.r, g + colour.g, b + colour.b, a)
    }

    pub fn diff_bg(&self, colour: Color) -> Color {
        match self.dark_mode {
            true => self.minus_bg(colour),
            false => self.plus_bg(colour),
        }
    }

    pub fn minus_main(&self, colour: Color) -> Color {
        let Color { r, g, b, a } = self.main_colour;

        Color::from_rgba(r - colour.r, g - colour.g, b - colour.b, a)
    }

    pub fn plus_main(&self, colour: Color) -> Color {
        let Color { r, g, b, a } = self.main_colour;

        Color::from_rgba(r + colour.r, g + colour.g, b + colour.b, a)
    }
}

impl container::StyleSheet for Theme {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(self.text_colour()),
            background: Some(Background::Color(self.background_colour())),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::WHITE,
        }
    }
}

impl button::StyleSheet for Theme {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(self.main_colour())),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::WHITE,
            text_color: self.text_colour(),
        }
    }

    fn hovered(&self) -> button::Style {
        let mut style = self.active();

        style.background = Some(Background::Color(
            self.minus_main(Color::from_rgb8(10, 10, 10)),
        ));

        style
    }
}

impl split::StyleSheet for Theme {
    fn active(&self) -> split::Style {
        split::Style {
            background: Some(Background::Color(self.background_colour())),
            first_background: Some(Background::Color(self.background_colour())),
            second_background: Some(Background::Color(self.background_colour())),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            divider_background: Background::Color(self.diff_bg(Color::from_rgb8(10, 10, 10))),
            divider_border_width: 2.5,
            divider_border_color: self.diff_bg(Color::from_rgb8(15, 15, 15)),
        }
    }

    fn hovered(&self) -> split::Style {
        let mut style = self.active();

        style.divider_background = Background::Color(self.diff_bg(Color::from_rgb8(25, 25, 25)));

        style
    }

    fn dragged(&self) -> split::Style {
        let mut style = self.active();

        style.divider_background = Background::Color(self.diff_bg(Color::from_rgb8(25, 25, 25)));

        style
    }
}
