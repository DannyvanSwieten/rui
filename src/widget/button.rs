use crate::{
    app::AppState,
    canvas::{font::Edging, Canvas2D, Color4f, Font, FontStyle, Paint, Size, TextBlob, Typeface},
    constraints::BoxConstraints,
    widget::{style::Theme, Event, EventCtx, MouseEvent, PaintCtx, Widget},
};

use super::LayoutCtx;

enum ButtonState {
    Inactive,
    Active,
    Hover,
}

pub enum ButtonStyle {
    Text,
    Outline,
    Fill,
}

pub struct TextButton<Message> {
    state: ButtonState,
    style: ButtonStyle,
    text: String,
    font: Font,
    on_click: Option<Message>,
}

impl<Message> TextButton<Message> {
    pub fn new(text: &str, font_size: f32) -> Self {
        let mut font = Font::new(
            Typeface::new("Monospace", FontStyle::normal()).unwrap(),
            font_size,
        );
        font.set_edging(Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        let mut bg_paint = Paint::default();
        bg_paint.set_anti_alias(true);
        bg_paint.set_color4f(Color4f::new(0.25, 0.25, 0.25, 1.0), None);
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color4f(Color4f::new(1.0, 1.0, 1.0, 1.0), None);
        Self {
            state: ButtonState::Inactive,
            text: text.to_string(),
            font,
            on_click: None,
            style: ButtonStyle::Outline,
        }
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }
}

impl<State: AppState> Widget<State> for TextButton<State::Message> {
    fn event(&mut self, event: &Event, ctx: &mut EventCtx<State::Message>, _: &State) -> bool {
        match event {
            Event::Mouse(MouseEvent::MouseEnter(_)) => {
                self.state = ButtonState::Hover;
                true
            }
            Event::Mouse(MouseEvent::MouseMove(_)) => {
                self.state = ButtonState::Hover;
                true
            }
            Event::Mouse(MouseEvent::MouseLeave(_)) => {
                self.state = ButtonState::Inactive;
                true
            }
            Event::Mouse(MouseEvent::MouseDown(_)) => {
                self.state = ButtonState::Active;
                true
            }
            Event::Mouse(MouseEvent::MouseUp(_)) => {
                if let Some(message) = &self.on_click {
                    ctx.publish(message.clone())
                }

                self.state = ButtonState::Inactive;
                true
            }
            _ => false,
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, _ctx: &mut LayoutCtx, _: &State) -> Size {
        let blob = TextBlob::from_str(&self.text, &self.font);
        let size = blob.unwrap().bounds().size();
        let width = constraints.max_width().unwrap_or(size.width);
        let height = constraints.max_height().unwrap_or(size.height);
        Size::new(width, height)
    }

    fn paint(&self, theme: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, _: &State) {
        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);

        let rect = ctx.rect();
        match self.style {
            ButtonStyle::Fill => {
                let mut bg_paint = Paint::default();
                bg_paint.set_anti_alias(true);
                match self.state {
                    ButtonState::Inactive => bg_paint.set_color(theme.primary.with_a(200)),
                    ButtonState::Active => bg_paint.set_color(theme.primary),
                    ButtonState::Hover => bg_paint.set_color(theme.primary.with_a(230)),
                };
                canvas.draw_rounded_rect(
                    &rect,
                    theme.button.rounding,
                    theme.button.rounding,
                    &bg_paint,
                );
                text_paint.set_color(theme.text);
                canvas.draw_string(&rect, &self.text, &self.font, &text_paint);
            }
            ButtonStyle::Outline => {
                let mut bg_paint = Paint::default();
                bg_paint.set_anti_alias(true);

                bg_paint.set_color(theme.primary);
                bg_paint.set_stroke(true);
                canvas.draw_rounded_rect(
                    &rect,
                    theme.button.rounding,
                    theme.button.rounding,
                    &bg_paint,
                );

                match self.state {
                    ButtonState::Inactive => (),
                    ButtonState::Active => {
                        bg_paint.set_color(theme.primary.with_a(100));
                        bg_paint.set_stroke(false);
                        canvas.draw_rounded_rect(
                            &rect,
                            theme.button.rounding,
                            theme.button.rounding,
                            &bg_paint,
                        );
                    }
                    ButtonState::Hover => {
                        bg_paint.set_color(theme.primary.with_a(50));
                        bg_paint.set_stroke(false);
                        canvas.draw_rounded_rect(
                            &rect,
                            theme.button.rounding,
                            theme.button.rounding,
                            &bg_paint,
                        );
                    }
                }

                text_paint.set_color(theme.primary);

                canvas.draw_string(&rect, &self.text, &self.font, &text_paint);
            }
            ButtonStyle::Text => {
                text_paint.set_color(theme.primary);

                let mut bg_paint = Paint::default();
                bg_paint.set_anti_alias(true);
                match self.state {
                    ButtonState::Inactive => (),
                    ButtonState::Active => {
                        bg_paint.set_color(theme.primary.with_a(100));
                        bg_paint.set_stroke(false);
                        canvas.draw_rounded_rect(
                            &rect,
                            theme.button.rounding,
                            theme.button.rounding,
                            &bg_paint,
                        );
                    }
                    ButtonState::Hover => {
                        bg_paint.set_color(theme.primary.with_a(50));
                        bg_paint.set_stroke(false);
                        canvas.draw_rounded_rect(
                            &rect,
                            theme.button.rounding,
                            theme.button.rounding,
                            &bg_paint,
                        );
                    }
                }

                canvas.draw_string(&rect, &self.text, &self.font, &text_paint);
            }
        }
    }
}
