use crate::{
    app::AppState,
    canvas::{
        textlayout::{self, FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
        Canvas2D, Color, Color4f, Paint, Point, Rect, Size,
    },
    constraints::BoxConstraints,
    widget::{style::Theme, Event, EventCtx, KeyEvent, PaintCtx, Widget},
};
use skia_safe::FontMgr;
use std::ops::Range;
use winit::{
    event::{ElementState, VirtualKeyCode},
    window::CursorIcon,
};

use super::{LayoutCtx, MouseEvent};

#[derive(Default)]
struct EditorState {
    text: String,
    caret_position: usize,
    selection: Range<usize>,
}

pub struct TextBox<State> {
    state: EditorState,
    placeholder: String,
    style: ParagraphStyle,
    on_commit: Option<Box<dyn Fn(&str, &State)>>,
}

impl<State> TextBox<State> {
    pub fn new(placeholder: &str) -> Self {
        Self {
            state: EditorState::default(),
            placeholder: placeholder.to_string(),
            style: ParagraphStyle::new(),
            on_commit: None,
        }
    }

    pub fn on_commit<F>(mut self, f: F) -> Self
    where
        F: Fn(&str, &State) + 'static,
    {
        self.on_commit = Some(Box::new(f));
        self
    }
}

impl<State: AppState + 'static> Widget<State> for TextBox<State> {
    fn event(
        &mut self,
        event: &super::Event,
        event_ctx: &mut EventCtx<State::Message>,
        state: &State,
    ) -> bool {
        match event {
            Event::Mouse(MouseEvent::MouseMove(_)) => {
                event_ctx.change_cursor(CursorIcon::Text);
                true
            }
            Event::Mouse(MouseEvent::MouseLeave(_)) => {
                event_ctx.change_cursor(CursorIcon::Default);
                true
            }
            Event::Key(KeyEvent::Input(event)) => {
                if let Some(keycode) = event.virtual_keycode {
                    if event.state == ElementState::Pressed {
                        match keycode {
                            VirtualKeyCode::Left => self.state.caret_position -= 1,
                            VirtualKeyCode::Right => self.state.caret_position += 1,
                            VirtualKeyCode::Back => {
                                if self.state.caret_position > 0 {
                                    self.state.text.remove(self.state.caret_position - 1);
                                    self.state.caret_position -= 1;
                                }
                            }
                            VirtualKeyCode::Return => {
                                if let Some(on_commit) = &self.on_commit {
                                    (*on_commit)(&self.state.text, state)
                                }
                            }
                            _ => (),
                        }
                    }
                }

                true
            }
            Event::Key(KeyEvent::Char(char)) => {
                if !char.is_ascii_control() {
                    self.state.text.push(*char);
                    self.state.caret_position += 1;
                }

                true
            }

            _ => false,
        }
    }

    fn layout(&mut self, constraints: &BoxConstraints, _ctx: &mut LayoutCtx, _: &State) -> Size {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);
        let mut paragraph_builder = ParagraphBuilder::new(&self.style, font_collection);
        let mut ts = TextStyle::new();
        ts.set_font_size(18.0);
        ts.set_foreground_color(Paint::default());
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(&self.placeholder);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(constraints.max_width().unwrap() - 4.0);
        Size::new(constraints.max_width().unwrap(), paragraph.height())
    }

    fn paint(&self, _: &Theme, ctx: &PaintCtx, canvas: &mut dyn Canvas2D, _: &State) {
        let rect = ctx.rect();

        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);
        let mut paragraph_builder = ParagraphBuilder::new(&self.style, font_collection);
        let mut ts = TextStyle::new();
        ts.set_font_size(18.0);
        if !self.state.text.is_empty() {
            ts.set_foreground_color(Paint::default());
            paragraph_builder.push_style(&ts);
            paragraph_builder.add_text(&self.state.text);
        } else {
            ts.set_foreground_color(Paint::new(Color4f::new(0.0, 0.0, 0.0, 0.5), None));
            paragraph_builder.push_style(&ts);
            paragraph_builder.add_text(&self.placeholder);
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(rect.width() - 4.0);
        let selection_boxes = paragraph.get_rects_for_range(
            self.state.selection.clone(),
            textlayout::RectHeightStyle::IncludeLineSpacingBottom,
            textlayout::RectWidthStyle::Tight,
        );

        let mut selected_rect = Rect::default();
        for textbox in &selection_boxes {
            selected_rect.join(textbox.rect)
        }

        let mut border_paint = Paint::default();
        border_paint.set_color(Color::from_rgb(255, 255, 255));
        canvas.draw_rect(&rect, &border_paint);
        if selected_rect.width() > 0.0 {
            border_paint.set_color(Color::from_rgb(0, 0, 255));
            canvas.draw_rect(&selected_rect, &border_paint);
        }
        border_paint.set_stroke(true);
        border_paint.set_color(Color::from_rgb(0, 0, 0));
        canvas.draw_rect(&rect, &border_paint);
        canvas.draw_paragraph(&Point::new(2.0, 0.0), &paragraph)
    }

    fn flex(&self) -> f32 {
        0.0
    }
}
