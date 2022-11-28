use skia_safe::{font::Edging, FontStyle, Typeface};

use crate::canvas::{Color, Font};
use std::collections::HashMap;
#[derive(Default)]
pub struct SliderThumbStyle {
    pub color: Color,
    pub size: f32,
}
#[derive(Default)]
pub struct SliderStyle {
    pub background: Color,
    pub fill: Color,
    pub thumb: SliderThumbStyle,
}

#[derive(Default)]
pub struct TextButtonStyle {
    pub inactive: Color,
    pub active: Color,
    pub hoverd: Color,
    pub text: Color,
    pub font: Font,
    pub rounding: f32,
}

impl TextButtonStyle {
    pub fn new() -> Self {
        Self {
            rounding: 4.0,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct LabelStyle {
    pub background: Color,
    pub text: Color,
    pub font: Font,
}
#[derive(Default)]
pub struct Theme {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub font: Font,

    pub button: TextButtonStyle,
    pub slider: SliderStyle,
    pub label: LabelStyle,
}

impl Theme {
    pub fn default_light() -> Self {
        Self {
            background: Color::new(0xFFFFFFFF),
            primary: Color::new(0xFF766AC8),
            secondary: Color::new(0xFF73C8A6),
            text: Color::new(0xFFFFFFFF),
            font: Self::create_default_font(),
            label: LabelStyle {
                background: Color::new(0xFFFFFFFF),
                text: Color::new(0xFFFFFFFF),
                font: Self::create_default_font(),
            },
            button: TextButtonStyle::new(),
            slider: SliderStyle {
                background: Color::new(0xFFFFFFFF),
                fill: Color::new(0xFF766AC8),
                thumb: SliderThumbStyle {
                    color: Color::new(0xFF73C8A6),
                    size: 0.125,
                },
            },
        }
    }

    fn create_default_font() -> Font {
        let typeface = Typeface::from_name("Roboto", FontStyle::normal()).unwrap();
        let mut font = Font::from_typeface(typeface, Some(18.0));
        font.set_edging(Edging::SubpixelAntiAlias);
        font
    }

    pub fn default_dark() -> Self {
        Self {
            background: Color::new(0xFF333333),
            primary: Color::new(0xFF1E38A1),
            secondary: Color::new(0xFF73C8A6),
            text: Color::new(0xFFFFFFFF),
            font: Self::create_default_font(),
            button: TextButtonStyle::new(),
            label: LabelStyle {
                background: Color::new(0xFF333333),
                text: Color::new(0xFFFFFFFF),
                font: Self::create_default_font(),
            },
            slider: SliderStyle {
                background: Color::new(0xFF1E1E1E),
                fill: Color::new(0xFF1E38A1),
                thumb: SliderThumbStyle {
                    color: Color::new(0xFF1E38A1),
                    size: 0.125,
                },
            },
        }
    }
}

#[derive(Default)]
pub struct StyleContext {
    themes: HashMap<String, Theme>,
}

impl StyleContext {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("light".to_string(), Theme::default_light());
        themes.insert("dark".to_string(), Theme::default_dark());
        Self { themes }
    }

    pub fn theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
}
